mod test_helpers;
use serial_test::serial;
use test_helpers::{insert_tenant, TestDb};
#[tokio::test]
#[serial]
async fn mixed_version_suppression_keys_are_always_protected() {
	let db = TestDb::start().await;
	let tenant = insert_tenant(db.pool(), "compatibility", None, 0).await;
	let other = insert_tenant(db.pool(), "compatibility-other", None, 0).await;
	for stored in ["Sales+tag@example.com", "sales+tag@example.com"] {
		sqlx::query("DELETE FROM v1_suppression_entries WHERE tenant_id=$1")
			.bind(tenant)
			.execute(db.pool())
			.await
			.unwrap();
		sqlx::query("INSERT INTO v1_suppression_entries (tenant_id,email,canonical_email,reason) VALUES ($1,$2,$2,'manual')").bind(tenant).bind(stored).execute(db.pool()).await.unwrap();
		for input in [
			"Sales+tag@Example.com",
			"sales+tag@example.com",
			"SALES+TAG@EXAMPLE.COM",
		] {
			assert!(
				reacher_backend::suppression::has_active_suppression(db.pool(), tenant, input)
					.await
					.unwrap(),
				"stored {}, input {}",
				stored,
				input
			);
			assert!(
				!reacher_backend::suppression::has_active_suppression(db.pool(), other, input)
					.await
					.unwrap()
			);
		}
		assert!(!reacher_backend::suppression::has_active_suppression(
			db.pool(),
			tenant,
			"sales@example.com"
		)
		.await
		.unwrap());
	}
}
