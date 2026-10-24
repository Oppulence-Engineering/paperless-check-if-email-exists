"""Regression cases for release boundaries, including forks and manual dispatch."""
import contextlib
import io
import json
import os
import subprocess
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import yaml

import release_policy as policy


ROOT = Path(__file__).resolve().parents[2]
REPOSITORY = 'Oppulence-Engineering/paperless-check-if-email-exists'
SHA = 'a' * 40


def promotion(base='main', head='develop', repository=REPOSITORY):
    return {'merged_at': '2026-10-24T00:00:00Z', 'merge_commit_sha': SHA,
            'base': {'ref': base}, 'head': {'ref': head, 'repo': {'full_name': repository}}}


class BranchPolicyTests(unittest.TestCase):
    def evaluate(self, base, head, repository=REPOSITORY):
        workflow = yaml.safe_load((ROOT / '.github/workflows/branch-policy.yml').read_text())
        job = workflow['jobs']['branch-policy']
        self.assertEqual(len(job['steps']), 1, 'Trusted guard must not check out PR code')
        code = job['steps'][0]['run']
        event = {'repository': {'full_name': REPOSITORY},
                 'pull_request': promotion(base, head, repository)}
        with tempfile.NamedTemporaryFile(mode='w') as event_file:
            json.dump(event, event_file)
            event_file.flush()
            with patch.dict(os.environ, GITHUB_EVENT_PATH=event_file.name), contextlib.redirect_stdout(io.StringIO()):
                exec(compile(code, 'branch-policy.yml', 'exec'), {})

    def test_features_fixes_dependencies_and_release_preparation_target_develop(self):
        for head in ('feature/audit', 'fix/migrations', 'dependabot/axios', 'release-please--branches--develop'):
            with self.subTest(head=head):
                self.evaluate('develop', head)

    def test_fork_contributions_can_target_develop(self):
        self.evaluate('develop', 'feature/audit', 'contributor/fork')

    def test_only_same_repository_develop_can_target_main(self):
        self.evaluate('main', 'develop')
        for head, repository in [('feature/audit', REPOSITORY), ('develop', 'contributor/fork')]:
            with self.subTest(head=head, repository=repository), self.assertRaises(SystemExit):
                self.evaluate('main', head, repository)

    def test_legacy_release_branch_is_rejected(self):
        with self.assertRaises(SystemExit):
            self.evaluate('master', 'develop')


class DeploymentPolicyTests(unittest.TestCase):
    def test_push_routing_and_bootstrap(self):
        self.assertEqual(policy.deployment_target('push', 'refs/heads/main', {}), 'production')
        self.assertEqual(policy.deployment_target('push', 'refs/heads/develop', {}), 'staging')
        self.assertEqual(policy.deployment_target('push', 'refs/heads/main', {'created': True}), 'bootstrap')
        self.assertEqual(policy.deployment_target('push', 'refs/tags/v4.6.0', {'created': True}), 'tag')
        self.assertEqual(policy.deployment_target('push', 'refs/heads/master', {}), 'invalid')

    def test_manual_deployment_cannot_cross_environments(self):
        for branch, target in [('develop', 'production'), ('main', 'staging'), ('feature/test', 'production'), ('main', 'all')]:
            with self.subTest(branch=branch, target=target), self.assertRaises(ValueError):
                policy.deployment_target('workflow_dispatch', f'refs/heads/{branch}', {'inputs': {'environment': target}})
        self.assertEqual(policy.deployment_target('workflow_dispatch', 'refs/heads/main', {'inputs': {'environment': 'production'}}), 'production')
        self.assertEqual(policy.deployment_target('workflow_dispatch', 'refs/heads/develop', {'inputs': {'environment': 'staging'}}), 'staging')
        self.assertEqual(policy.deployment_target('workflow_dispatch', 'refs/heads/feature/test', {'inputs': {'environment': 'build-only'}}), 'build-only')

    def test_publication_requires_actual_merged_promotion_commit(self):
        self.assertTrue(policy.promotion_matches(promotion(), REPOSITORY, SHA))
        cases = [promotion(head='feature/audit'), promotion(repository='attacker/fork'), promotion(base='develop'),
                 dict(promotion(), merged_at=None), dict(promotion(), merge_commit_sha='b' * 40)]
        for pr in cases:
            with self.subTest(pr=pr):
                self.assertFalse(policy.promotion_matches(pr, REPOSITORY, SHA))

    def test_direct_main_push_fails_closed(self):
        with patch.object(policy.subprocess, 'check_output', return_value='[]'), self.assertRaises(ValueError):
            policy.verify_promotion(REPOSITORY, SHA)


class VersionPolicyTests(unittest.TestCase):
    def test_unchanged_version_does_not_publish(self):
        self.assertIsNone(policy.release_version({'backend': '4.5.0'}, {'backend': '4.5.0'}))

    def test_increasing_versions_include_semver_prerelease_order(self):
        for old, new in [('4.5.0', '4.6.0'), ('4.6.0-rc.9', '4.6.0-rc.10'), ('4.6.0-rc.1', '4.6.0')]:
            with self.subTest(old=old, new=new):
                self.assertEqual(policy.release_version({'backend': old}, {'backend': new}), new)

    def test_invalid_and_decreasing_versions_fail_before_publication(self):
        for old, new in [('4.5.0', '4.4.9'), ('4.5.0', '4.5.0-rc.1'), ('4.6.0-rc.10', '4.6.0-rc.9'),
                         ('4.5.0', '04.6.0'), ('4.5.0', '4.6.0-01'), ('4.5.0', '4.6.0;echo nope'), ('4.5.0', None)]:
            with self.subTest(old=old, new=new), self.assertRaises(ValueError):
                policy.release_version({'backend': old}, {'backend': new})


class ReleaseRefTests(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.repo = Path(self.directory.name)
        self.git('init', '-q')
        self.git('config', 'user.name', 'Release policy test')
        self.git('config', 'user.email', 'test@example.invalid')
        self.git('commit', '-q', '--allow-empty', '-m', 'Released baseline')
        self.main_sha = self.git('rev-parse', 'HEAD')
        self.git('update-ref', 'refs/remotes/origin/main', self.main_sha)
        self.git('tag', 'v4.6.0')
        self.git('commit', '-q', '--allow-empty', '-m', 'Unreleased development change')
        self.develop_sha = self.git('rev-parse', 'HEAD')
        self.git('tag', 'v4.7.0')

    def git(self, *args):
        return subprocess.check_output(['git', *args], cwd=self.repo, text=True).strip()

    def validate(self, tag, sha):
        workflow = yaml.safe_load((ROOT / '.github/workflows/validate-release-ref.yml').read_text())
        code = workflow['jobs']['validate']['steps'][-1]['run']
        output = self.repo / 'output'
        result = subprocess.run(['bash', '-c', code], cwd=self.repo, capture_output=True, text=True,
                                env={**os.environ, 'RELEASE_TAG': tag, 'EXPECTED_SHA': sha, 'GITHUB_OUTPUT': str(output)})
        return result, output.read_text() if output.exists() else ''

    def test_release_tag_returns_exact_main_commit(self):
        result, output = self.validate('v4.6.0', self.main_sha)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn(f'sha={self.main_sha}', output)
        self.assertIn('version=4.6.0', output)

    def test_tag_on_unreleased_development_commit_is_rejected(self):
        result, output = self.validate('v4.7.0', self.develop_sha)
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(output, '')

    def test_moved_tag_cannot_change_triggering_release_source(self):
        result, output = self.validate('v4.6.0', self.develop_sha)
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(output, '')

    def test_branch_or_shell_text_is_not_a_release_tag(self):
        for tag in ('main', 'v4.6.0;touch injected', '$(touch injected)'):
            with self.subTest(tag=tag):
                result, output = self.validate(tag, self.main_sha)
                self.assertNotEqual(result.returncode, 0)
                self.assertEqual(output, '')
                self.assertFalse((self.repo / 'injected').exists())


class PublicationTests(unittest.TestCase):
    def publish(self, old, new, existing_tag=None, existing_release=False):
        event = {'created': False, 'before': 'b' * 40}
        def read(path):
            return json.dumps(event if str(path) == '/tmp/event.json' else {'backend': new})
        def run(args, **kwargs):
            if args[:2] == ['git', 'rev-parse']:
                return policy.subprocess.CompletedProcess(args, 0 if existing_tag else 1, stdout=existing_tag or '')
            if args[:3] == ['gh', 'release', 'view']:
                return policy.subprocess.CompletedProcess(args, 0 if existing_release else 1)
            return policy.subprocess.CompletedProcess(args, 0)
        with patch.dict(os.environ, GITHUB_EVENT_PATH='/tmp/event.json', GITHUB_REF='refs/heads/main',
                        GITHUB_SHA=SHA, GITHUB_REPOSITORY=REPOSITORY), \
             patch.object(policy.Path, 'read_text', read), \
             patch.object(policy.subprocess, 'check_output', side_effect=[json.dumps([promotion()]), json.dumps({'backend': old}), '[]']), \
             patch.object(policy.subprocess, 'run', side_effect=run) as runner, \
             contextlib.redirect_stdout(io.StringIO()):
            policy.main()
            return [call.args[0] for call in runner.call_args_list]

    def test_new_release_targets_exact_promotion_commit(self):
        commands = self.publish('4.5.0', '4.6.0')
        create = [args for args in commands if args[:3] == ['gh', 'release', 'create']]
        self.assertEqual(len(create), 1)
        self.assertEqual(create[0][3], 'v4.6.0')
        self.assertEqual(create[0][create[0].index('--target') + 1], SHA)

    def test_unchanged_version_has_no_publish_side_effect(self):
        self.assertEqual(self.publish('4.5.0', '4.5.0'), [])

    def test_successful_retry_does_not_publish_again(self):
        commands = self.publish('4.5.0', '4.6.0', existing_tag=SHA, existing_release=True)
        self.assertFalse(any(args[:3] == ['gh', 'release', 'create'] for args in commands))

    def test_existing_tag_at_different_commit_is_never_moved(self):
        with self.assertRaisesRegex(ValueError, 'another commit'):
            self.publish('4.5.0', '4.6.0', existing_tag='c' * 40, existing_release=True)


class PreparationCleanupTests(unittest.TestCase):
    def test_only_included_release_preparation_prs_are_marked_tagged(self):
        prs = [
            {'number': 10, 'headRefName': 'release-please--branches--master', 'mergeCommit': {'oid': 'included'}},
            {'number': 11, 'headRefName': 'release-please--branches--develop', 'mergeCommit': {'oid': 'future'}},
            {'number': 12, 'headRefName': 'feature/other', 'mergeCommit': {'oid': 'included'}},
        ]
        def run(args, **kwargs):
            return subprocess.CompletedProcess(args, 1 if 'future' in args else 0)
        with patch.object(policy.subprocess, 'check_output', return_value=json.dumps(prs)), \
             patch.object(policy.subprocess, 'run', side_effect=run) as runner:
            policy.complete_preparation_prs(REPOSITORY, SHA)
        edits = [call.args[0] for call in runner.call_args_list if call.args[0][:3] == ['gh', 'pr', 'edit']]
        self.assertEqual(len(edits), 1)
        self.assertEqual(edits[0][3], '10')
        self.assertIn('autorelease: tagged', edits[0])
        self.assertIn('autorelease: pending', edits[0])

    def test_git_errors_fail_cleanup_instead_of_acknowledging_unknown_history(self):
        prs = [{'number': 10, 'headRefName': 'release-please--branches--develop', 'mergeCommit': {'oid': 'missing'}}]
        with patch.object(policy.subprocess, 'check_output', return_value=json.dumps(prs)), \
             patch.object(policy.subprocess, 'run', return_value=subprocess.CompletedProcess(['git'], 128)), \
             self.assertRaises(subprocess.CalledProcessError):
            policy.complete_preparation_prs(REPOSITORY, SHA)


if __name__ == '__main__':
    unittest.main()
