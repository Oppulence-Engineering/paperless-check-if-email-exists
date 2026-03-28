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


def camel_to_snake(name: str) -> str:
    first_pass = re.sub(r"(.)([A-Z][a-z]+)", r"\1_\2", name)
    return re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", first_pass).lower()


def required_fields(schema_name: str) -> set[str]:
    return set(SCHEMAS.get(schema_name, {}).get("required", [])) | SPECIAL_REQUIRED_FIELDS.get(
        schema_name, set()
    )


def write_if_changed(path: Path, content: str) -> None:
    if not path.exists() or path.read_text() != content:
        path.write_text(content)


def normalize_go_models() -> None:
    utils_path = GO_SDK_DIR / "utils.go"
    if not utils_path.exists():
        return
    utils_text = utils_path.read_text()
    utils_text = utils_text.replace("\tdec.DisallowUnknownFields()\n", "")
    write_if_changed(utils_path, utils_text)

    for path in GO_SDK_DIR.glob("model_*.go"):
        text = path.read_text()
        text = text.replace("\tdecoder.DisallowUnknownFields()\n", "")

        if path.name == "model_pipeline_source.go":
            text = text.replace(
                "if err = validator.Validate(dst.PipelineSourceOneOf); err != nil {\n\t\t\t\tdst.PipelineSourceOneOf = nil\n\t\t\t} else {\n\t\t\t\tmatch++\n\t\t\t}",
                "if err = validator.Validate(dst.PipelineSourceOneOf); err != nil || dst.PipelineSourceOneOf.Type != \"list_snapshot\" {\n\t\t\t\tdst.PipelineSourceOneOf = nil\n\t\t\t} else {\n\t\t\t\tmatch++\n\t\t\t}",
            )
            text = text.replace(
                "if err = validator.Validate(dst.PipelineSourceOneOf1); err != nil {\n\t\t\t\tdst.PipelineSourceOneOf1 = nil\n\t\t\t} else {\n\t\t\t\tmatch++\n\t\t\t}",
                "if err = validator.Validate(dst.PipelineSourceOneOf1); err != nil || dst.PipelineSourceOneOf1.Type != \"integration\" {\n\t\t\t\tdst.PipelineSourceOneOf1 = nil\n\t\t\t} else {\n\t\t\t\tmatch++\n\t\t\t}",
            )
            text = text.replace(
                "if err = validator.Validate(dst.PipelineSourceOneOf2); err != nil {\n\t\t\t\tdst.PipelineSourceOneOf2 = nil\n\t\t\t} else {\n\t\t\t\tmatch++\n\t\t\t}",
                "if err = validator.Validate(dst.PipelineSourceOneOf2); err != nil || dst.PipelineSourceOneOf2.Type != \"push\" {\n\t\t\t\tdst.PipelineSourceOneOf2 = nil\n\t\t\t} else {\n\t\t\t\tmatch++\n\t\t\t}",
            )
            text = text.replace(
                "if err = validator.Validate(dst.PipelineSourceOneOf3); err != nil {\n\t\t\t\tdst.PipelineSourceOneOf3 = nil\n\t\t\t} else {\n\t\t\t\tmatch++\n\t\t\t}",
                "if err = validator.Validate(dst.PipelineSourceOneOf3); err != nil || dst.PipelineSourceOneOf3.Type != \"bucket\" {\n\t\t\t\tdst.PipelineSourceOneOf3 = nil\n\t\t\t} else {\n\t\t\t\tmatch++\n\t\t\t}",
            )

        write_if_changed(path, text)


def normalize_go_docs() -> None:
    if not GO_DOCS_DIR.exists():
        return
    for path in GO_DOCS_DIR.glob("*.md"):
        text = path.read_text()
        model_name = text.splitlines()[0].removeprefix("# ").strip()
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

        write_if_changed(path, "\n".join(lines) + "\n")


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

    if type_text.startswith("[**"):
        return "{} as any"
    if "Array&lt;" in type_text or type_text.startswith("Array<"):
        return "[]"
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


def main() -> None:
    normalize_go_models()
    normalize_go_docs()
    normalize_typescript_docs()


if __name__ == "__main__":
    main()
