#!/usr/bin/env python3
"""Generate the Rust client from a pinned copy of MillionVerifier's OpenAPI."""
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]
SDK = ROOT / "sdks/millionverifier"
version = subprocess.check_output(["openapi-generator-cli", "version"], cwd=ROOT, text=True).strip()
if version != "7.14.0":
    raise SystemExit(f"Expected OpenAPI Generator 7.14.0, got {version}")
spec = json.loads((SDK / "openapi.upstream.json").read_text())
# The platform already queues individual checks, including rows from uploaded lists.
spec["paths"] = {key: spec["paths"][key] for key in ("/api/v3", "/api/v3/credits")}
spec["servers"][0]["url"] = "https://api.millionverifier.com"
spec["components"]["schemas"] = {
    key: spec["components"]["schemas"][key] for key in ("SingleApiResponse", "Subresult")
}
# The three upstream anyOf alternatives have the same shape, including HTTP 200 errors.
spec["paths"]["/api/v3"]["get"]["responses"]["200"]["content"]["application/json"]["schema"] = {
    "$ref": "#/components/schemas/SingleApiResponse"
}
# Empty subresults occur on API errors; unknown future values must fail closed in the adapter.
spec["components"]["schemas"]["Subresult"].pop("enum")
props = spec["components"]["schemas"]["SingleApiResponse"]["properties"]
for key in ("quality", "result", "resultcode"):
    props[key].pop("enum", None)
for path in spec["paths"].values():
    path["get"].pop("x-codeSamples", None)
# The documented path redirects; use the canonical URL without forwarding API-key redirects.
spec["paths"]["/api/v3/"] = spec["paths"].pop("/api/v3")
(SDK / "openapi.json").write_text(json.dumps(spec, indent=2) + "\n")
subprocess.run(
    ["openapi-generator-cli", "generate", "-i", str(SDK / "openapi.json"),
     "-g", "rust", "-o", str(SDK / "generated"),
     "--additional-properties=packageName=millionverifier-sdk,packageVersion=0.1.0,hideGenerationTimestamp=true"],
    cwd=ROOT, check=True,
)
manifest = SDK / "generated/Cargo.toml"
text = manifest.read_text()
text = text.replace('[package]\n', '[package]\npublish = false\n')
# Use the same reqwest/TLS stack as the backend.
text = text.replace('version = "^0.12"', 'version = "0.13"')
text = text.replace('features = ["json", "multipart"]', 'features = ["json", "rustls"]')
text = text.replace('serde_repr = "^0.1"\n', '')
manifest.write_text(text)
# Credentials live in the query string. Generated error formatting must never reveal it.
apis = SDK / "generated/src/apis/mod.rs"
text = apis.read_text().replace(
    "        Error::Reqwest(e)\n", "        Error::Reqwest(e.without_url())\n"
)
apis.write_text(text)
lib = SDK / "generated/src/lib.rs"
lib.write_text(lib.read_text().replace("extern crate serde_repr;\n", ""))
for api in (SDK / "generated/src/apis").glob("*_api.rs"):
    api.write_text(api.read_text().replace(
        "!status.is_client_error() && !status.is_server_error()", "status.is_success()"
    ))
for name in ("git_push.sh", ".travis.yml"):
    (SDK / "generated" / name).unlink(missing_ok=True)
subprocess.run(["cargo", "fmt", "--manifest-path", str(manifest)], cwd=ROOT, check=True)
