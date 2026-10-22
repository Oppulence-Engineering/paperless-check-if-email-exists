#[test]
fn embedded_migrations_preserve_the_deployed_october_history() {
	// These checksums were read from staging's applied history on 2026-10-22.
	let deployed = [
        (20261021000000_i64, "b5c8845e23480fec154165b2046d91f2b4f790d92350569504098d7f6df838d24d3b637a61b76f9d9ae892d0008a88a0"),
        (20261021000001_i64, "5705373275898645056c07242e0a254e2a20563552d24628f852d3701a7ba271b6035743151356844eca3111e2a55ee0"),
        (20261021000002_i64, "71ef40d0067545e89b66288e717fac116634cb3f8b70200355c29bfb86b83a66ef25c0e24920f726cc30b0c8717add72"),
        (20261021000003_i64, "7b911679e53852c235cf51098584b6441b99e9379f60bdef691eff477dab15a550563aa74059ee1fe8083215f9480e93"),
        (20261021000004_i64, "ec98aee83b42a2d7963d0ebc1df902ba65439e46dbc70035b5b91569816716efaf467a517669b11d048c785a6e326c7f"),
    ];
	let migrator = sqlx::migrate!("./migrations");
	for (version, checksum) in deployed {
		let migration = migrator
			.iter()
			.find(|migration| {
				migration.version == version && migration.migration_type.is_up_migration()
			})
			.expect("an already-deployed migration must remain embedded");
		assert_eq!(
			hex::encode(&migration.checksum),
			checksum,
			"migration {version} changed"
		);
	}
}
