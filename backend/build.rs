fn main() {
	// SQLx embeds migrations; newly added files must invalidate incremental builds.
	println!("cargo:rerun-if-changed=migrations");
}
