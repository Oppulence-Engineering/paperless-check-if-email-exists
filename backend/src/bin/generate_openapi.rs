use reacher_backend::http::openapi::build_spec;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let spec = build_spec()?;
	let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
	let output = manifest_dir.join("openapi.json");
	std::fs::write(output, serde_json::to_vec_pretty(&spec)?)?;
	Ok(())
}
