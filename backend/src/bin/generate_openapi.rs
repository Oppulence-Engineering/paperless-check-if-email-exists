use reacher_backend::http::openapi::build_spec;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let spec = build_spec()?;
	let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
	let output_path = manifest_dir.join("openapi.json");
	let mut output = serde_json::to_vec_pretty(&spec)?;
	output.push(b'\n');
	std::fs::write(output_path, output)?;
	Ok(())
}
