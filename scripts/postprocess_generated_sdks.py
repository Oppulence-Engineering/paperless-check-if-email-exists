#!/usr/bin/env python3

import json
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
SPEC_PATH = ROOT / "backend" / "openapi.json"
GO_SDK_DIR = ROOT / "sdks" / "golang"
TS_DOCS_DIR = ROOT / "sdks" / "typescript" / "src" / "docs"
GO_DOCS_DIR = GO_SDK_DIR / "docs"


def load_schemas() -> dict[str, dict]:
    spec = json.loads(SPEC_PATH.read_text())
    return spec.get("components", {}).get("schemas", {})


SCHEMAS = load_schemas()
SPECIAL_REQUIRED_FIELDS = {
    "PipelineSourceOneOf": {"list_id", "type"},
    "PipelineSourceOneOf1": {"audience_id", "connection_id", "provider", "type"},
    "PipelineSourceOneOf2": {"accepted_format", "token_id", "type"},
    "PipelineSourceOneOf3": {"bucket", "provider", "type"},
}
GO_UNION_DOCS = {
    "CheckEmailOutputMisc": {
        "description": "Additional information about the email account.",
        "variants": [
            ("CoreError", "CoreErrorAsCheckEmailOutputMisc"),
            ("MiscDetails", "MiscDetailsAsCheckEmailOutputMisc"),
        ],
    },
    "CheckEmailOutputMx": {
        "description": "Details obtained from querying the mail server's MX records.",
        "variants": [
            ("CoreError", "CoreErrorAsCheckEmailOutputMx"),
            ("MxDetails", "MxDetailsAsCheckEmailOutputMx"),
        ],
    },
    "CheckEmailOutputSmtp": {
        "description": "Results from connecting to the mail server via SMTP.",
        "variants": [
            ("CoreError", "CoreErrorAsCheckEmailOutputSmtp"),
            ("SmtpDetails", "SmtpDetailsAsCheckEmailOutputSmtp"),
        ],
    },
}


def camel_to_snake(name: str) -> str:
    first_pass = re.sub(r"(.)([A-Z][a-z]+)", r"\1_\2", name)
    return re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", first_pass).lower()


def required_fields(schema_name: str) -> set[str]:
    spec_required = set(SCHEMAS.get(schema_name, {}).get("required", []))
    if spec_required or schema_name in SPECIAL_REQUIRED_FIELDS:
        return spec_required | SPECIAL_REQUIRED_FIELDS.get(schema_name, set())
    return required_fields_from_go_model(schema_name)


def write_if_changed(path: Path, content: str) -> None:
    if not path.exists() or path.read_text() != content:
        path.write_text(content)


def replace_or_raise(text: str, old: str, new: str, *, file: Path, label: str) -> str:
    if old in text:
        return text.replace(old, new)
    if new in text:
        return text
    raise RuntimeError(f"Postprocess failed in {file}: missing expected snippet for {label}")


def go_model_path(model_name: str) -> Path:
    direct_path = GO_SDK_DIR / f"model_{camel_to_snake(model_name)}.go"
    if direct_path.exists():
        return direct_path

    for path in GO_SDK_DIR.glob("model_*.go"):
        text = path.read_text()
        if f"type {model_name} struct" in text or f"type _{model_name} " in text:
            return path

    return direct_path


def required_fields_from_go_model(model_name: str) -> set[str]:
    model_path = go_model_path(model_name)
    if not model_path.exists():
        return set()

    match = re.search(
        rf"type {re.escape(model_name)} struct \{{(.*?)\n\}}",
        model_path.read_text(),
        flags=re.S,
    )
    if match is None:
        return set()

    required = set()
    for line in match.group(1).splitlines():
        tag_match = re.search(r'`json:"([^"]+)"`', line)
        if tag_match is None:
            continue
        tag_parts = tag_match.group(1).split(",")
        json_name, options = tag_parts[0], set(tag_parts[1:])
        if "omitempty" not in options:
            required.add(json_name)

    return required


def extract_go_signatures(model_name: str) -> dict[str, str]:
    model_path = go_model_path(model_name)
    if not model_path.exists():
        return {}

    signatures: dict[str, str] = {}
    function_pattern = re.compile(r"^func\s+(?:\(([^)]*)\)\s+)?([A-Za-z0-9_]+)\(")
    for line in model_path.read_text().splitlines():
        stripped = line.strip()
        if not stripped.startswith("func "):
            continue

        match = function_pattern.match(stripped)
        if match is None:
            continue

        receiver, name = match.groups()
        if stripped.endswith("{"):
            stripped = stripped[:-1].rstrip()
        if receiver is None:
            if name.startswith(f"New{model_name}") or name.endswith(f"As{model_name}"):
                signatures[name] = stripped
            continue

        if model_name not in receiver:
            continue
        signatures[name] = stripped

    return signatures


def normalize_go_doc_methods(text: str, model_name: str) -> str:
    if "## Methods" not in text:
        return text

    signatures = extract_go_signatures(model_name)
    if not signatures:
        return text

    lines = text.splitlines()
    methods_index = lines.index("## Methods")
    footer_index = next(
        (index for index in range(methods_index + 1, len(lines)) if lines[index].startswith("[[Back to Model list]]")),
        len(lines),
    )

    normalized = lines[: methods_index + 1] + [""]
    index = methods_index + 1
    while index < footer_index:
        if not lines[index].startswith("### "):
            index += 1
            continue

        title = lines[index].removeprefix("### ").strip()
        next_index = index + 1
        while next_index < footer_index and not lines[next_index].startswith("### "):
            next_index += 1

        if title in signatures:
            section = lines[index:next_index]
            for section_index, line in enumerate(section):
                if line.startswith("`func "):
                    section[section_index] = f"`{signatures[title]}`"
                    break
            normalized.extend(section)
            normalized.append("")

        index = next_index

    normalized.extend(lines[footer_index:])
    return "\n".join(normalized).replace("\n\n\n", "\n\n").strip() + "\n"


def render_go_union_doc(model_name: str) -> str:
    config = GO_UNION_DOCS[model_name]
    variant_lines = [
        f"- [{variant}]({variant}.md) via `{constructor}(v *{variant}) {model_name}`"
        for variant, constructor in config["variants"]
    ]
    methods = [
        f"### {constructor}\n\n`func {constructor}(v *{variant}) {model_name}`"
        for variant, constructor in config["variants"]
    ]
    methods.extend(
        [
            f"### GetActualInstance\n\n`func (obj *{model_name}) GetActualInstance() (interface{{}})`",
            f"### GetActualInstanceValue\n\n`func (obj {model_name}) GetActualInstanceValue() (interface{{}})`",
        ]
    )

    return "\n".join(
        [
            f"# {model_name}",
            "",
            config["description"],
            "",
            "## OneOf variants",
            "",
            *variant_lines,
            "",
            "## Methods",
            "",
            *methods,
            "",
            "[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)",
            "",
        ]
    )


def normalize_go_models() -> None:
    utils_path = GO_SDK_DIR / "utils.go"
    if utils_path.exists():
        utils_text = utils_path.read_text()
        utils_text = utils_text.replace("\tdec.DisallowUnknownFields()\n", "")
        write_if_changed(utils_path, utils_text)

    for path in GO_SDK_DIR.glob("model_*.go"):
        text = path.read_text()
        text = text.replace("\tdecoder.DisallowUnknownFields()\n", "")

        if path.name == "model_pipeline_source.go":
            text = replace_or_raise(
                text,
                "if err = validator.Validate(dst.PipelineSourceOneOf); err != nil {\n\t\t\t\tdst.PipelineSourceOneOf = nil\n\t\t\t} else {\n\t\t\t\tmatch++\n\t\t\t}",
                "if err = validator.Validate(dst.PipelineSourceOneOf); err != nil || dst.PipelineSourceOneOf.Type != \"list_snapshot\" {\n\t\t\t\tdst.PipelineSourceOneOf = nil\n\t\t\t} else {\n\t\t\t\tmatch++\n\t\t\t}",
                file=path,
                label="PipelineSourceOneOf:list_snapshot",
            )
            text = replace_or_raise(
                text,
                "if err = validator.Validate(dst.PipelineSourceOneOf1); err != nil {\n\t\t\t\tdst.PipelineSourceOneOf1 = nil\n\t\t\t} else {\n\t\t\t\tmatch++\n\t\t\t}",
                "if err = validator.Validate(dst.PipelineSourceOneOf1); err != nil || dst.PipelineSourceOneOf1.Type != \"integration\" {\n\t\t\t\tdst.PipelineSourceOneOf1 = nil\n\t\t\t} else {\n\t\t\t\tmatch++\n\t\t\t}",
                file=path,
                label="PipelineSourceOneOf1:integration",
            )
            text = replace_or_raise(
                text,
                "if err = validator.Validate(dst.PipelineSourceOneOf2); err != nil {\n\t\t\t\tdst.PipelineSourceOneOf2 = nil\n\t\t\t} else {\n\t\t\t\tmatch++\n\t\t\t}",
                "if err = validator.Validate(dst.PipelineSourceOneOf2); err != nil || dst.PipelineSourceOneOf2.Type != \"push\" {\n\t\t\t\tdst.PipelineSourceOneOf2 = nil\n\t\t\t} else {\n\t\t\t\tmatch++\n\t\t\t}",
                file=path,
                label="PipelineSourceOneOf2:push",
            )
            text = replace_or_raise(
                text,
                "if err = validator.Validate(dst.PipelineSourceOneOf3); err != nil {\n\t\t\t\tdst.PipelineSourceOneOf3 = nil\n\t\t\t} else {\n\t\t\t\tmatch++\n\t\t\t}",
                "if err = validator.Validate(dst.PipelineSourceOneOf3); err != nil || dst.PipelineSourceOneOf3.Type != \"bucket\" {\n\t\t\t\tdst.PipelineSourceOneOf3 = nil\n\t\t\t} else {\n\t\t\t\tmatch++\n\t\t\t}",
                file=path,
                label="PipelineSourceOneOf3:bucket",
            )

        if path.name == "model_pipeline_run_view.go":
            text = replace_or_raise(
                text,
                "if o.SourceSnapshot != nil {\n\t\ttoSerialize[\"source_snapshot\"] = o.SourceSnapshot\n\t}",
                "toSerialize[\"source_snapshot\"] = o.SourceSnapshot",
                file=path,
                label="PipelineRunView:source_snapshot serialization",
            )
            text = replace_or_raise(
                text,
                "if o.Stats != nil {\n\t\ttoSerialize[\"stats\"] = o.Stats\n\t}",
                "toSerialize[\"stats\"] = o.Stats",
                file=path,
                label="PipelineRunView:stats serialization",
            )

        pipeline_source_variant_patches = {
            "model_pipeline_source_one_of.go": (
                "PipelineSourceOneOf",
                "varPipelineSourceOneOf",
                "list_snapshot",
            ),
            "model_pipeline_source_one_of_1.go": (
                "PipelineSourceOneOf1",
                "varPipelineSourceOneOf1",
                "integration",
            ),
            "model_pipeline_source_one_of_2.go": (
                "PipelineSourceOneOf2",
                "varPipelineSourceOneOf2",
                "push",
            ),
            "model_pipeline_source_one_of_3.go": (
                "PipelineSourceOneOf3",
                "varPipelineSourceOneOf3",
                "bucket",
            ),
        }
        if path.name in pipeline_source_variant_patches:
            model_name, variable_name, expected_type = pipeline_source_variant_patches[path.name]
            text = replace_or_raise(
                text,
                f"\t*o = {model_name}({variable_name})\n\n\treturn err\n",
                (
                    f"\tif {variable_name}.Type != \"{expected_type}\" {{\n"
                    f"\t\treturn fmt.Errorf(\"invalid discriminator value for {model_name}: expected \\\"{expected_type}\\\", got %q\", {variable_name}.Type)\n"
                    "\t}\n\n"
                    f"\t*o = {model_name}({variable_name})\n\n\treturn err\n"
                ),
                file=path,
                label=f"{model_name}:{expected_type} discriminator",
            )

        write_if_changed(path, text)


def normalize_go_docs() -> None:
    if not GO_DOCS_DIR.exists():
        return
    for path in GO_DOCS_DIR.glob("*.md"):
        text = path.read_text()
        model_name = text.splitlines()[0].removeprefix("# ").strip()
        if model_name in GO_UNION_DOCS:
            write_if_changed(path, render_go_union_doc(model_name))
            continue

        required = required_fields(model_name)

        text = re.sub(r",\s*\)\s+\*", r") *", text)
        text = re.sub(r"\(\*\[]([^,)]+), bool\)", r"([]\1, bool)", text)

        lines = text.splitlines()
        in_properties = False
        for idx, line in enumerate(lines):
            if line.strip() == "## Properties":
                in_properties = True
                continue
            if in_properties and line.startswith("## "):
                in_properties = False
            if not in_properties or not line.startswith("**"):
                continue

            parts = [part.strip() for part in line.split("|")]
            while parts and parts[-1] == "":
                parts.pop()
            while len(parts) < 4:
                parts.append("")
            name, type_text, description, _notes = parts[:4]
            prop_name = camel_to_snake(name.strip("*"))
            notes = "[required]" if prop_name in required else "[optional]"
            lines[idx] = f"{name} | {type_text} | {description} | {notes}"

        text = normalize_go_doc_methods("\n".join(lines) + "\n", model_name)
        write_if_changed(path, text)


def ts_value(model_name: str, prop_name: str, type_text: str) -> str:
    special_models = {
        "PipelineSource": {
            "list_id": "123",
            "type": "'list_snapshot'",
        },
        "PipelineSourceOneOf": {
            "list_id": "123",
            "type": "'list_snapshot'",
        },
        "PipelineSourceOneOf1": {
            "audience_id": "'aud_123'",
            "connection_id": "'conn_123'",
            "provider": "'mailchimp'",
            "type": "'integration'",
        },
        "PipelineSourceOneOf2": {
            "accepted_format": "'csv'",
            "token_id": "'tok_123'",
            "type": "'push'",
        },
        "PipelineSourceOneOf3": {
            "bucket": "'example-bucket'",
            "provider": "'s3'",
            "type": "'bucket'",
        },
    }
    if model_name in special_models and prop_name in special_models[model_name]:
        return special_models[model_name][prop_name]

    if "Array&lt;" in type_text or type_text.startswith("Array<"):
        return "[]"
    if type_text.startswith("[**"):
        return "{} as any"
    if "{ [key: string]: string; }" in type_text:
        return "{ Authorization: 'Bearer example' }"
    if type_text == "**boolean**":
        return "true"
    if type_text == "**number**":
        numeric_defaults = {
            "freshness_days": "30",
            "max_attempts": "5",
            "retry_backoff_seconds": "300",
            "missed_run_window_hours": "24",
            "total": "0",
        }
        return numeric_defaults.get(prop_name, "0")
    if type_text == "**string**":
        string_defaults = {
            "url": "'https://example.com/webhook'",
            "cron": "'0 9 * * 1'",
            "timezone": "'UTC'",
            "accepted_format": "'csv'",
            "name": "'Weekly Cleanup'",
            "bucket": "'example-bucket'",
            "prefix": "'imports/'",
            "path_pattern": "'*.csv'",
            "provider": "'mailchimp'",
            "audience_id": "'aud_123'",
            "connection_id": "'conn_123'",
            "token_id": "'tok_123'",
            "region": "'us-east-1'",
            "type": "'example'",
        }
        return string_defaults.get(prop_name, "'example'")
    return "{} as any"


def replace_ts_example(text: str, model_name: str, properties: list[tuple[str, str]]) -> str:
    if model_name == "CreatePipelineInput":
        example_lines = [
            "    name: 'Weekly Cleanup',",
            "    schedule: { cron: '0 9 * * 1', timezone: 'UTC' },",
            "    source: { type: 'list_snapshot', list_id: 123 },",
        ]
    elif model_name == "PipelineSource":
        example_lines = [
            "    list_id: 123,",
            "    type: 'list_snapshot',",
        ]
    else:
        example_lines = []
        for prop_name, type_text in properties:
            example_lines.append(f"    {prop_name}: {ts_value(model_name, prop_name, type_text)},")

    example_block = "\n".join(
        [
            "```typescript",
            f"import {{ {model_name} }} from '@oppulence/reacher-sdk';",
            "",
            f"const instance: {model_name} = {{",
            *example_lines,
            "};",
            "```",
        ]
    )

    return re.sub(
        r"## Example\n\n```typescript.*?```",
        f"## Example\n\n{example_block}",
        text,
        flags=re.S,
    )


def normalize_typescript_docs() -> None:
    if not TS_DOCS_DIR.exists():
        return
    for path in TS_DOCS_DIR.glob("*.md"):
        text = path.read_text()
        model_name = text.splitlines()[0].removeprefix("# ").strip()
        required = required_fields(model_name)

        lines = text.splitlines()
        in_properties = False
        properties: list[tuple[str, str]] = []
        for idx, line in enumerate(lines):
            if line.strip() == "## Properties":
                in_properties = True
                continue
            if in_properties and line.startswith("## "):
                in_properties = False
            if not in_properties or not line.startswith("**"):
                continue

            parts = [part.strip() for part in line.split("|")]
            while parts and parts[-1] == "":
                parts.pop()
            while len(parts) < 4:
                parts.append("")
            name, type_text, description, _notes = parts[:4]
            prop_name = name.strip("*")
            properties.append((prop_name, type_text))
            notes = "[required]" if prop_name in required else "[optional]"
            lines[idx] = f"{name} | {type_text} | {description} | {notes}"

        text = "\n".join(lines) + "\n"
        if "## Example" in text and properties:
            text = replace_ts_example(text, model_name, properties)

        write_if_changed(path, text)


def normalize_typescript_readme() -> None:
    readme_path = ROOT / "sdks" / "typescript" / "src" / "README.md"
    if not readme_path.exists():
        return

    text = readme_path.read_text()
    pipeline_rows = {
        "v1CreatePipeline",
        "v1DeletePipeline",
        "v1GetPipeline",
        "v1GetPipelineRun",
        "v1ListPipelineRuns",
        "v1ListPipelines",
        "v1PausePipeline",
        "v1ResumePipeline",
        "v1TriggerPipeline",
        "v1UpdatePipeline",
    }

    normalized_lines: list[str] = []
    for line in text.splitlines():
        if not line.startswith("*PipelinesApi* |"):
            normalized_lines.append(line)
            continue

        line = line.replace("*PipelinesApi*", "_PipelinesApi_")
        for operation in pipeline_rows:
            line = line.replace(f"[**{operation}**]", f"[__{operation}__]")
        normalized_lines.append(line)

    text = "\n".join(normalized_lines) + "\n"
    write_if_changed(readme_path, text)


def main() -> None:
    normalize_go_models()
    normalize_go_docs()
    normalize_typescript_docs()
    normalize_typescript_readme()


if __name__ == "__main__":
    main()
