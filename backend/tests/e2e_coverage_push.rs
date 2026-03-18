mod test_helpers;
#[allow(unused_imports)]
use serial_test::serial;

// =============================================================================
// 1. throttle.rs — per-minute, per-hour, per-day limits
// =============================================================================
#[cfg(test)]
mod throttle_coverage {
    use serial_test::serial;
    use reacher_backend::config::ThrottleConfig;
    use reacher_backend::throttle::{ThrottleLimit, ThrottleManager};

    #[tokio::test]
    #[serial]
    async fn test_throttle_per_minute_limit() {
        let config = ThrottleConfig {
            max_requests_per_second: None,
            max_requests_per_minute: Some(2),
            max_requests_per_hour: None,
            max_requests_per_day: None,
        };
        let manager = ThrottleManager::new(config);

        // First two should be allowed
        assert_eq!(manager.check_throttle().await, None);
        manager.increment_counters().await;
        assert_eq!(manager.check_throttle().await, None);
        manager.increment_counters().await;

        // Third request should hit the per-minute limit
        let result = manager.check_throttle().await;
        assert!(result.is_some());
        assert_eq!(result.unwrap().limit_type, ThrottleLimit::PerMinute);
    }

    #[tokio::test]
    #[serial]
    async fn test_throttle_per_hour_limit() {
        let config = ThrottleConfig {
            max_requests_per_second: None,
            max_requests_per_minute: None,
            max_requests_per_hour: Some(2),
            max_requests_per_day: None,
        };
        let manager = ThrottleManager::new(config);

        assert_eq!(manager.check_throttle().await, None);
        manager.increment_counters().await;
        assert_eq!(manager.check_throttle().await, None);
        manager.increment_counters().await;

        let result = manager.check_throttle().await;
        assert!(result.is_some());
        assert_eq!(result.unwrap().limit_type, ThrottleLimit::PerHour);
    }

    #[tokio::test]
    #[serial]
    async fn test_throttle_per_day_limit() {
        let config = ThrottleConfig {
            max_requests_per_second: None,
            max_requests_per_minute: None,
            max_requests_per_hour: None,
            max_requests_per_day: Some(2),
        };
        let manager = ThrottleManager::new(config);

        assert_eq!(manager.check_throttle().await, None);
        manager.increment_counters().await;
        assert_eq!(manager.check_throttle().await, None);
        manager.increment_counters().await;

        let result = manager.check_throttle().await;
        assert!(result.is_some());
        assert_eq!(result.unwrap().limit_type, ThrottleLimit::PerDay);
    }

    #[tokio::test]
    #[serial]
    async fn test_throttle_limit_display() {
        // Cover the Display impl for all variants
        assert_eq!(format!("{}", ThrottleLimit::PerSecond), "per second");
        assert_eq!(format!("{}", ThrottleLimit::PerMinute), "per minute");
        assert_eq!(format!("{}", ThrottleLimit::PerHour), "per hour");
        assert_eq!(format!("{}", ThrottleLimit::PerDay), "per day");
    }

    #[tokio::test]
    #[serial]
    async fn test_throttle_no_limits_always_allows() {
        let config = ThrottleConfig {
            max_requests_per_second: None,
            max_requests_per_minute: None,
            max_requests_per_hour: None,
            max_requests_per_day: None,
        };
        let manager = ThrottleManager::new(config);

        for _ in 0..100 {
            assert_eq!(manager.check_throttle().await, None);
            manager.increment_counters().await;
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_throttle_per_second_hit_first_when_all_set() {
        // When all limits are set, per-second should fire first
        let config = ThrottleConfig {
            max_requests_per_second: Some(1),
            max_requests_per_minute: Some(100),
            max_requests_per_hour: Some(1000),
            max_requests_per_day: Some(10000),
        };
        let manager = ThrottleManager::new(config);

        assert_eq!(manager.check_throttle().await, None);
        manager.increment_counters().await;

        let result = manager.check_throttle().await;
        assert!(result.is_some());
        assert_eq!(result.unwrap().limit_type, ThrottleLimit::PerSecond);
    }
}

// =============================================================================
// 2. v0/bulk/results/mod.rs — CSV format endpoint + JSON pagination + running job
// =============================================================================
#[cfg(test)]
mod v0_bulk_results_coverage {
    use serial_test::serial;
    use crate::test_helpers::TestDb;
    use reacher_backend::config::{BackendConfig, PostgresConfig, StorageConfig};
    use sqlx::Row;
    use std::sync::Arc;
    use warp::http::StatusCode;
    use warp::test::request;

    /// Build a config with just Postgres (no auth, no worker).
    async fn db_only_config(_pool: sqlx::PgPool) -> Arc<BackendConfig> {
        let mut config = BackendConfig::empty();
        config.header_secret = None;
        // We already have a pool from TestDb; set up config storage so
        // get_pg_pool() works through the storage adapter. We do this by
        // connecting via the real path.
        let db_url = std::env::var("TEST_DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:25432/reacher_test".into()
        });
        config.storage = Some(StorageConfig::Postgres(PostgresConfig {
            db_url,
            extra: None,
        }));
        config.connect().await.unwrap();
        Arc::new(config)
    }

    fn valid_result_json() -> serde_json::Value {
        serde_json::json!({
            "input": "test@example.com",
            "is_reachable": "safe",
            "misc": {
                "is_disposable": false,
                "is_role_account": false,
                "gravatar_url": null
            },
            "mx": {
                "accepts_email": true,
                "records": ["mx1.example.com"]
            },
            "smtp": {
                "can_connect_smtp": true,
                "has_full_inbox": false,
                "is_catch_all": false,
                "is_deliverable": true,
                "is_disabled": false
            },
            "syntax": {
                "is_valid_syntax": true,
                "domain": "example.com",
                "username": "test"
            }
        })
    }

    /// Insert a v0 bulk_job and return its id.
    async fn insert_v0_bulk_job(pool: &sqlx::PgPool, total_records: i32) -> i32 {
        let row = sqlx::query("INSERT INTO bulk_jobs (total_records) VALUES ($1) RETURNING id")
            .bind(total_records)
            .fetch_one(pool)
            .await
            .expect("insert v0 bulk_job");
        row.get("id")
    }

    /// Insert a v0 email_result for a given job_id.
    async fn insert_v0_email_result(pool: &sqlx::PgPool, job_id: i32, result: &serde_json::Value) {
        sqlx::query("INSERT INTO email_results (job_id, result) VALUES ($1, $2)")
            .bind(job_id)
            .bind(result)
            .execute(pool)
            .await
            .expect("insert v0 email_result");
    }

    /// Clean up v0 tables between tests.
    async fn cleanup_v0(pool: &sqlx::PgPool) {
        let _ = sqlx::query("DELETE FROM email_results")
            .execute(pool)
            .await;
        let _ = sqlx::query("DELETE FROM bulk_jobs").execute(pool).await;
    }

    #[tokio::test]
    #[serial]
    async fn test_v0_bulk_results_csv_format() {
        let db = TestDb::start().await;
        let pool = db.pool_owned();
        cleanup_v0(&pool).await;

        let job_id = insert_v0_bulk_job(&pool, 1).await;
        insert_v0_email_result(&pool, job_id, &valid_result_json()).await;

        let config = db_only_config(pool).await;
        let routes = reacher_backend::http::create_routes(config);

        let resp = request()
            .path(&format!("/v0/bulk/{}/results?format=csv", job_id))
            .method("GET")
            .reply(&routes)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
        let content_type = resp
            .headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap();
        assert!(
            content_type.contains("text/csv"),
            "Expected text/csv, got {}",
            content_type
        );
        let body = String::from_utf8_lossy(resp.body());
        assert!(body.contains("test@example.com"), "CSV should contain the email address");
        assert!(body.contains("safe"), "CSV should contain is_reachable value");
    }

    #[tokio::test]
    #[serial]
    async fn test_v0_bulk_results_json_with_pagination() {
        let db = TestDb::start().await;
        let pool = db.pool_owned();
        cleanup_v0(&pool).await;

        let job_id = insert_v0_bulk_job(&pool, 2).await;
        insert_v0_email_result(&pool, job_id, &valid_result_json()).await;
        let mut result2 = valid_result_json();
        result2["input"] = serde_json::json!("other@example.com");
        insert_v0_email_result(&pool, job_id, &result2).await;

        let config = db_only_config(pool).await;
        let routes = reacher_backend::http::create_routes(config);

        // Request with limit=1
        let resp = request()
            .path(&format!(
                "/v0/bulk/{}/results?format=json&limit=1",
                job_id
            ))
            .method("GET")
            .reply(&routes)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
        let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
        let results = body["results"].as_array().unwrap();
        assert_eq!(results.len(), 1, "Should return only 1 result when limit=1");
    }

    #[tokio::test]
    #[serial]
    async fn test_v0_bulk_running_job_rejects_results() {
        let db = TestDb::start().await;
        let pool = db.pool_owned();
        cleanup_v0(&pool).await;

        // Job expects 5 records but only has 2
        let job_id = insert_v0_bulk_job(&pool, 5).await;
        insert_v0_email_result(&pool, job_id, &valid_result_json()).await;
        insert_v0_email_result(&pool, job_id, &valid_result_json()).await;

        let config = db_only_config(pool).await;
        let routes = reacher_backend::http::create_routes(config);

        let resp = request()
            .path(&format!("/v0/bulk/{}/results", job_id))
            .method("GET")
            .reply(&routes)
            .await;

        // Should reject because the job is still in progress
        assert_ne!(
            resp.status(),
            StatusCode::OK,
            "Running job should not return 200"
        );
    }

    #[tokio::test]
    #[serial]
    async fn test_v0_bulk_results_json_default_format() {
        let db = TestDb::start().await;
        let pool = db.pool_owned();
        cleanup_v0(&pool).await;

        let job_id = insert_v0_bulk_job(&pool, 1).await;
        insert_v0_email_result(&pool, job_id, &valid_result_json()).await;

        let config = db_only_config(pool).await;
        let routes = reacher_backend::http::create_routes(config);

        // No format query param -> defaults to JSON
        let resp = request()
            .path(&format!("/v0/bulk/{}/results", job_id))
            .method("GET")
            .reply(&routes)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
        let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
        assert!(body["results"].is_array());
    }

    #[tokio::test]
    #[serial]
    async fn test_v0_bulk_results_csv_deprecation_headers() {
        let db = TestDb::start().await;
        let pool = db.pool_owned();
        cleanup_v0(&pool).await;

        let job_id = insert_v0_bulk_job(&pool, 1).await;
        insert_v0_email_result(&pool, job_id, &valid_result_json()).await;

        let config = db_only_config(pool).await;
        let routes = reacher_backend::http::create_routes(config);

        let resp = request()
            .path(&format!("/v0/bulk/{}/results?format=csv", job_id))
            .method("GET")
            .reply(&routes)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
        // v0 endpoints should have deprecation headers
        assert_eq!(
            resp.headers()
                .get("Deprecation")
                .map(|v| v.to_str().unwrap()),
            Some("true")
        );
    }
}

// =============================================================================
// 3. http/error.rs — From implementations for csv::Error, InvalidStatusCode
// =============================================================================
#[cfg(test)]
mod error_from_coverage {
    use serial_test::serial;
    use reacher_backend::config::BackendConfig;
    use reacher_backend::http::ReacherResponseError;
    use std::sync::Arc;
    use warp::http::StatusCode;

    #[test]
    #[serial]
    fn test_from_csv_error() {
        // csv::Error can be created by deserializing invalid CSV
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader("not,valid\ncsv".as_bytes());
        // Force a type mismatch error
        let result: Result<(i32, i32), csv::Error> = rdr.deserialize().next().unwrap();
        let csv_err = result.unwrap_err();
        let err: ReacherResponseError = csv_err.into();
        assert_eq!(err.code, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    #[serial]
    fn test_from_csv_into_inner_error() {
        // Create a csv writer, write to it, then cause an IntoInnerError
        // by using a writer that is already flushed. We can't easily cause a
        // real IntoInnerError, but we can test the From<csv::Error> path
        // directly by creating a csv Error from a bad record.
        let mut wtr = csv::WriterBuilder::new().from_writer(vec![]);
        // Write a valid record first
        wtr.write_record(&["a", "b"]).unwrap();
        // into_inner should succeed here, so test the From path via csv::Error
        let _data = wtr.into_inner().unwrap();

        // Test that the From impl exists by converting a csv error
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader("x".as_bytes());
        let result: Result<(i32,), _> = rdr.deserialize().next().unwrap();
        let csv_err = result.unwrap_err();
        let err: ReacherResponseError = csv_err.into();
        assert_eq!(err.code, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    #[serial]
    fn test_from_invalid_status_code() {
        // StatusCode::from_u16 with an invalid value returns InvalidStatusCode
        let invalid = StatusCode::from_u16(9999).unwrap_err();
        let err: ReacherResponseError = invalid.into();
        assert_eq!(err.code, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    #[serial]
    async fn test_handle_rejection_via_create_routes() {
        // Test that the rejection handler in create_routes correctly formats
        // ReacherResponseError as JSON. We trigger it by sending an empty
        // to_email which produces a BAD_REQUEST ReacherResponseError.
        let config = Arc::new(BackendConfig::empty());
        let routes = reacher_backend::http::create_routes(config);

        let resp = warp::test::request()
            .path("/v1/check_email")
            .method("POST")
            .json(&serde_json::json!({"to_email": ""}))
            .reply(&routes)
            .await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
        assert!(body["error"].is_string());
    }
}

// =============================================================================
// 4. deprecation.rs — deprecation_wrapper function
// =============================================================================
#[cfg(test)]
mod deprecation_wrapper_coverage {
    use serial_test::serial;
    use warp::hyper::body::Bytes;

    #[test]
    #[serial]
    fn test_deprecation_wrapper_adds_headers() {
        let wrapper =
            reacher_backend::http::deprecation::deprecation_wrapper("2026-09-16", "/v1/new");

        // Build a minimal response
        let resp = warp::http::Response::builder()
            .status(200)
            .body(Bytes::from_static(b"hello"))
            .unwrap();

        let modified = wrapper(resp);

        assert_eq!(
            modified.headers().get("Deprecation").unwrap().to_str().unwrap(),
            "true"
        );
        assert_eq!(
            modified.headers().get("Sunset").unwrap().to_str().unwrap(),
            "2026-09-16"
        );
        let link = modified.headers().get("Link").unwrap().to_str().unwrap();
        assert!(link.contains("/v1/new"));
        assert!(link.contains("successor-version"));
    }

    #[test]
    #[serial]
    fn test_deprecation_wrapper_is_clone() {
        let wrapper =
            reacher_backend::http::deprecation::deprecation_wrapper("2027-01-01", "/v2/api");
        let wrapper2 = wrapper.clone();

        let resp = warp::http::Response::builder()
            .status(200)
            .body(Bytes::from_static(b""))
            .unwrap();
        let modified = wrapper2(resp);
        assert_eq!(
            modified.headers().get("Deprecation").unwrap().to_str().unwrap(),
            "true"
        );
    }

    #[test]
    #[serial]
    fn test_deprecation_wrapper_preserves_body_and_status() {
        let wrapper =
            reacher_backend::http::deprecation::deprecation_wrapper("2026-12-31", "/v3/x");

        let resp = warp::http::Response::builder()
            .status(404)
            .body(Bytes::from_static(b"not found"))
            .unwrap();

        let modified = wrapper(resp);
        assert_eq!(modified.status(), 404);
        assert_eq!(modified.body().as_ref(), b"not found");
    }
}

// =============================================================================
// 5. readiness.rs — degraded status (pg ok but no rmq)
// =============================================================================
#[cfg(test)]
mod readiness_coverage {
    use serial_test::serial;
    use crate::test_helpers::TestDb;
    use reacher_backend::config::{BackendConfig, PostgresConfig, StorageConfig, WorkerConfig};
    use std::sync::Arc;
    use warp::http::StatusCode;
    use warp::test::request;

    #[tokio::test]
    #[serial]
    async fn test_readyz_postgres_ok_no_worker() {
        // Postgres connected, no worker -> pg=ok, rmq=not_configured -> overall "ok"
        let db = TestDb::start().await;
        let _ = db.pool();

        let db_url = std::env::var("TEST_DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:25432/reacher_test".into()
        });
        let mut config = BackendConfig::empty();
        config.storage = Some(StorageConfig::Postgres(PostgresConfig {
            db_url,
            extra: None,
        }));
        config.connect().await.unwrap();
        let routes = reacher_backend::http::create_routes(Arc::new(config));

        let resp = request()
            .path("/readyz")
            .method("GET")
            .reply(&routes)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
        let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
        assert_eq!(body["status"], "ok");
        assert_eq!(body["checks"]["postgres"]["status"], "ok");
        assert_eq!(body["checks"]["rabbitmq"]["status"], "not_configured");
        // Postgres should have latency_ms
        assert!(body["checks"]["postgres"]["latency_ms"].is_number());
    }

    #[tokio::test]
    #[serial]
    async fn test_readyz_no_postgres_no_worker() {
        // No postgres, no worker -> both not_configured -> "ok"
        let config = BackendConfig::empty();
        let routes = reacher_backend::http::create_routes(Arc::new(config));

        let resp = request()
            .path("/readyz")
            .method("GET")
            .reply(&routes)
            .await;

        let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
        assert_eq!(body["checks"]["postgres"]["status"], "not_configured");
        assert_eq!(body["checks"]["rabbitmq"]["status"], "not_configured");
        // When both are not_configured, overall should be "ok"
        assert_eq!(body["status"], "ok");
    }

    #[tokio::test]
    #[serial]
    async fn test_readyz_worker_enabled_no_rabbitmq_connection() {
        // Worker enabled but no RabbitMQ channel -> rmq=unavailable
        let mut config = BackendConfig::empty();
        config.worker = WorkerConfig {
            enable: true,
            rabbitmq: None,
            webhook: None,
        };
        // Don't call connect() since we don't have a real RabbitMQ.
        // The must_worker_config() will fail, triggering the "unavailable" path.
        let routes = reacher_backend::http::create_routes(Arc::new(config));

        let resp = request()
            .path("/readyz")
            .method("GET")
            .reply(&routes)
            .await;

        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
        let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
        assert_eq!(body["checks"]["rabbitmq"]["status"], "unavailable");
    }

    #[tokio::test]
    #[serial]
    async fn test_readyz_degraded_state() {
        // Postgres ok + RabbitMQ unavailable -> should be "unavailable"
        // (not degraded, since unavailable + ok = unavailable per the match)
        let db = TestDb::start().await;
        let _ = db.pool();

        let db_url = std::env::var("TEST_DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:25432/reacher_test".into()
        });
        let mut config = BackendConfig::empty();
        config.storage = Some(StorageConfig::Postgres(PostgresConfig {
            db_url,
            extra: None,
        }));
        config.worker = WorkerConfig {
            enable: true,
            rabbitmq: None,
            webhook: None,
        };
        // Connect postgres but not rabbitmq
        // We can't call config.connect() because it would try to set up rabbitmq.
        // Instead, just set the storage adapter manually by re-creating with
        // a partial connect. We'll create a new config with storage connected
        // but worker channel missing.
        let db_url2 = std::env::var("TEST_DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:25432/reacher_test".into()
        });
        let mut base_config = BackendConfig::empty();
        base_config.storage = Some(StorageConfig::Postgres(PostgresConfig {
            db_url: db_url2,
            extra: None,
        }));
        // First connect storage only (worker disabled)
        base_config.connect().await.unwrap();

        // Now copy the storage adapter but enable worker without channel
        // We need to construct the final config with the pg_pool but worker.enable=true
        // and no channel. We achieve this by building a config that has Postgres storage
        // connected and worker enabled but no rabbitmq config (no channel).
        // The readiness check will see worker.enable=true, call must_worker_config(),
        // which returns Err, and set rmq to "unavailable".
        let mut final_config = BackendConfig::empty();
        final_config.worker = WorkerConfig {
            enable: true,
            rabbitmq: None,
            webhook: None,
        };
        final_config.storage = base_config.storage.clone();
        // We need the storage adapter connected. Since BackendConfig::empty() sets Noop,
        // we'll use a trick: connect a non-worker config first, then flip the flag.
        let db_url3 = std::env::var("TEST_DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:25432/reacher_test".into()
        });
        let mut trick_config = BackendConfig::empty();
        trick_config.storage = Some(StorageConfig::Postgres(PostgresConfig {
            db_url: db_url3,
            extra: None,
        }));
        trick_config.worker.enable = false;
        trick_config.connect().await.unwrap();
        // Now enable worker after connect (no channel was set up)
        trick_config.worker.enable = true;

        let routes = reacher_backend::http::create_routes(Arc::new(trick_config));

        let resp = request()
            .path("/readyz")
            .method("GET")
            .reply(&routes)
            .await;

        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
        let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
        // pg should be ok, rmq should be unavailable
        assert_eq!(body["checks"]["postgres"]["status"], "ok");
        assert_eq!(body["checks"]["rabbitmq"]["status"], "unavailable");
        // overall should be "unavailable" because ("ok", "unavailable") matches that arm
        assert_eq!(body["status"], "unavailable");
    }
}

// =============================================================================
// 6. v0/check_email/post.rs — to_check_email_input branches
// =============================================================================
#[cfg(test)]
mod v0_check_email_input_coverage {
    use serial_test::serial;
    use reacher_backend::config::BackendConfig;
    use reacher_backend::http::CheckEmailRequest;
    use std::sync::Arc;

    #[test]
    #[serial]
    fn test_to_check_email_input_with_proxy() {
        let config = Arc::new(BackendConfig::empty());
        let req: CheckEmailRequest = serde_json::from_value(serde_json::json!({
            "to_email": "user@example.com",
            "proxy": {
                "host": "proxy.example.com",
                "port": 1080
            }
        }))
        .unwrap();

        let input = req.to_check_email_input(config);
        assert_eq!(input.to_email, "user@example.com");
        // The verif method should be constructed with the proxy
        // We can verify by checking that the proxy was set in the verif method
        // The VerifMethod fields are provider-specific; the key thing is it
        // doesn't panic and constructs successfully with a proxy.
    }

    #[test]
    #[serial]
    fn test_to_check_email_input_with_yahoo_verif_method() {
        let config = Arc::new(BackendConfig::empty());
        let req: CheckEmailRequest = serde_json::from_value(serde_json::json!({
            "to_email": "user@yahoo.com",
            "yahoo_verif_method": "Smtp"
        }))
        .unwrap();

        let input = req.to_check_email_input(config);
        assert_eq!(input.to_email, "user@yahoo.com");
    }

    #[test]
    #[serial]
    fn test_to_check_email_input_with_hotmailb2c_verif_method() {
        let config = Arc::new(BackendConfig::empty());
        let req: CheckEmailRequest = serde_json::from_value(serde_json::json!({
            "to_email": "user@hotmail.com",
            "hotmailb2c_verif_method": "Smtp"
        }))
        .unwrap();

        let input = req.to_check_email_input(config);
        assert_eq!(input.to_email, "user@hotmail.com");
    }

    #[test]
    #[serial]
    fn test_to_check_email_input_with_proxy_and_yahoo() {
        let config = Arc::new(BackendConfig::empty());
        let req: CheckEmailRequest = serde_json::from_value(serde_json::json!({
            "to_email": "user@yahoo.com",
            "proxy": {
                "host": "proxy.test",
                "port": 8080
            },
            "yahoo_verif_method": "Smtp"
        }))
        .unwrap();

        let input = req.to_check_email_input(config);
        assert_eq!(input.to_email, "user@yahoo.com");
    }

    #[test]
    #[serial]
    fn test_to_check_email_input_with_proxy_and_hotmail() {
        let config = Arc::new(BackendConfig::empty());
        let req: CheckEmailRequest = serde_json::from_value(serde_json::json!({
            "to_email": "user@outlook.com",
            "proxy": {
                "host": "proxy.test",
                "port": 8080
            },
            "hotmailb2c_verif_method": "Smtp"
        }))
        .unwrap();

        let input = req.to_check_email_input(config);
        assert_eq!(input.to_email, "user@outlook.com");
    }

    #[test]
    #[serial]
    fn test_to_check_email_input_custom_from_and_hello() {
        let config = Arc::new(BackendConfig::empty());
        let req: CheckEmailRequest = serde_json::from_value(serde_json::json!({
            "to_email": "dest@example.com",
            "from_email": "custom@sender.com",
            "hello_name": "sender.com"
        }))
        .unwrap();

        let input = req.to_check_email_input(config);
        assert_eq!(input.to_email, "dest@example.com");
    }

    #[test]
    #[serial]
    fn test_to_check_email_input_with_smtp_timeout_and_port() {
        let config = Arc::new(BackendConfig::empty());
        let req: CheckEmailRequest = serde_json::from_value(serde_json::json!({
            "to_email": "dest@example.com",
            "smtp_timeout": { "secs": 30, "nanos": 0 },
            "smtp_port": 587
        }))
        .unwrap();

        let input = req.to_check_email_input(config);
        assert_eq!(input.to_email, "dest@example.com");
    }

    #[test]
    #[serial]
    fn test_to_check_email_input_yahoo_headless() {
        let config = Arc::new(BackendConfig::empty());
        let req: CheckEmailRequest = serde_json::from_value(serde_json::json!({
            "to_email": "user@yahoo.com",
            "yahoo_verif_method": "Headless"
        }))
        .unwrap();

        let input = req.to_check_email_input(config);
        assert_eq!(input.to_email, "user@yahoo.com");
    }

    #[test]
    #[serial]
    fn test_to_check_email_input_hotmail_headless() {
        let config = Arc::new(BackendConfig::empty());
        let req: CheckEmailRequest = serde_json::from_value(serde_json::json!({
            "to_email": "user@hotmail.com",
            "hotmailb2c_verif_method": "Headless"
        }))
        .unwrap();

        let input = req.to_check_email_input(config);
        assert_eq!(input.to_email, "user@hotmail.com");
    }
}

// =============================================================================
// 7. v0/bulk/get.rs — running job status path
// =============================================================================
#[cfg(test)]
mod v0_bulk_get_running_coverage {
    use serial_test::serial;
    use crate::test_helpers::TestDb;
    use reacher_backend::config::{BackendConfig, PostgresConfig, StorageConfig};
    use sqlx::Row;
    use std::sync::Arc;
    use warp::http::StatusCode;
    use warp::test::request;

    async fn db_config() -> (TestDb, Arc<BackendConfig>) {
        let db = TestDb::start().await;
        let db_url = std::env::var("TEST_DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:25432/reacher_test".into()
        });
        let mut config = BackendConfig::empty();
        config.header_secret = None;
        config.storage = Some(StorageConfig::Postgres(PostgresConfig {
            db_url,
            extra: None,
        }));
        config.connect().await.unwrap();
        (db, Arc::new(config))
    }

    async fn cleanup(pool: &sqlx::PgPool) {
        let _ = sqlx::query("DELETE FROM email_results")
            .execute(pool)
            .await;
        let _ = sqlx::query("DELETE FROM bulk_jobs").execute(pool).await;
    }

    #[tokio::test]
    #[serial]
    async fn test_v0_bulk_get_running_status() {
        let (db, config) = db_config().await;
        let pool = db.pool_owned();
        cleanup(&pool).await;

        // Create a job with 5 records but only 2 completed
        let row = sqlx::query("INSERT INTO bulk_jobs (total_records) VALUES ($1) RETURNING id")
            .bind(5i32)
            .fetch_one(&pool)
            .await
            .unwrap();
        let job_id: i32 = row.get("id");

        let result_json = serde_json::json!({
            "input": "test@example.com",
            "is_reachable": "safe",
            "misc": {"is_disposable": false, "is_role_account": false},
            "mx": {"accepts_email": true, "records": []},
            "smtp": {"can_connect_smtp": true, "has_full_inbox": false, "is_catch_all": false, "is_deliverable": true, "is_disabled": false},
            "syntax": {"is_valid_syntax": true, "domain": "example.com", "username": "test"}
        });
        sqlx::query("INSERT INTO email_results (job_id, result) VALUES ($1, $2)")
            .bind(job_id)
            .bind(&result_json)
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO email_results (job_id, result) VALUES ($1, $2)")
            .bind(job_id)
            .bind(&result_json)
            .execute(&pool)
            .await
            .unwrap();

        let routes = reacher_backend::http::create_routes(config);

        let resp = request()
            .path(&format!("/v0/bulk/{}", job_id))
            .method("GET")
            .reply(&routes)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
        let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
        assert_eq!(body["job_status"], "Running");
        assert_eq!(body["total_records"], 5);
        assert_eq!(body["total_processed"], 2);
        assert!(body["finished_at"].is_null());
    }

    #[tokio::test]
    #[serial]
    async fn test_v0_bulk_get_completed_status() {
        let (db, config) = db_config().await;
        let pool = db.pool_owned();
        cleanup(&pool).await;

        // Create a completed job: 2 records, 2 results
        let row = sqlx::query("INSERT INTO bulk_jobs (total_records) VALUES ($1) RETURNING id")
            .bind(2i32)
            .fetch_one(&pool)
            .await
            .unwrap();
        let job_id: i32 = row.get("id");

        let result_json = serde_json::json!({
            "input": "a@b.com",
            "is_reachable": "safe",
            "misc": {"is_disposable": false, "is_role_account": false},
            "mx": {"accepts_email": true, "records": []},
            "smtp": {"can_connect_smtp": true, "has_full_inbox": false, "is_catch_all": false, "is_deliverable": true, "is_disabled": false},
            "syntax": {"is_valid_syntax": true, "domain": "b.com", "username": "a"}
        });
        for _ in 0..2 {
            sqlx::query("INSERT INTO email_results (job_id, result) VALUES ($1, $2)")
                .bind(job_id)
                .bind(&result_json)
                .execute(&pool)
                .await
                .unwrap();
        }

        let routes = reacher_backend::http::create_routes(config);

        let resp = request()
            .path(&format!("/v0/bulk/{}", job_id))
            .method("GET")
            .reply(&routes)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
        let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
        assert_eq!(body["job_status"], "Completed");
        assert_eq!(body["total_records"], 2);
        assert_eq!(body["total_processed"], 2);
        assert!(!body["finished_at"].is_null());

        // Check summary fields
        assert!(body["summary"]["total_safe"].is_number());
    }

    #[tokio::test]
    #[serial]
    async fn test_v0_bulk_get_deprecation_headers() {
        let (db, config) = db_config().await;
        let pool = db.pool_owned();
        cleanup(&pool).await;

        let row = sqlx::query("INSERT INTO bulk_jobs (total_records) VALUES ($1) RETURNING id")
            .bind(1i32)
            .fetch_one(&pool)
            .await
            .unwrap();
        let job_id: i32 = row.get("id");

        let result_json = serde_json::json!({
            "input": "a@b.com", "is_reachable": "safe",
            "misc": {"is_disposable": false, "is_role_account": false},
            "mx": {"accepts_email": true, "records": []},
            "smtp": {"can_connect_smtp": true, "has_full_inbox": false, "is_catch_all": false, "is_deliverable": true, "is_disabled": false},
            "syntax": {"is_valid_syntax": true, "domain": "b.com", "username": "a"}
        });
        sqlx::query("INSERT INTO email_results (job_id, result) VALUES ($1, $2)")
            .bind(job_id)
            .bind(&result_json)
            .execute(&pool)
            .await
            .unwrap();

        let routes = reacher_backend::http::create_routes(config);

        let resp = request()
            .path(&format!("/v0/bulk/{}", job_id))
            .method("GET")
            .reply(&routes)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers()
                .get("Deprecation")
                .map(|v| v.to_str().unwrap()),
            Some("true")
        );
    }
}

// =============================================================================
// 8. v1/bulk/get_results/mod.rs — CSV format for v1
// =============================================================================
#[cfg(test)]
mod v1_bulk_results_csv_coverage {
    use serial_test::serial;
    use crate::test_helpers::{TestDb, insert_tenant, insert_api_key, insert_job, insert_task, safe_result};
    use reacher_backend::config::{
        BackendConfig, PostgresConfig, StorageConfig,
    };
    use std::sync::Arc;
    use warp::http::StatusCode;
    use warp::test::request;

    /// Build a config with Postgres connected and worker.enable=true
    /// but no actual RabbitMQ channel (sufficient for the results endpoint).
    async fn worker_db_config() -> (TestDb, Arc<BackendConfig>) {
        let db = TestDb::start().await;
        let db_url = std::env::var("TEST_DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:25432/reacher_test".into()
        });
        let mut config = BackendConfig::empty();
        config.header_secret = None;
        config.storage = Some(StorageConfig::Postgres(PostgresConfig {
            db_url,
            extra: None,
        }));
        config.worker.enable = false;
        config.connect().await.unwrap();
        // Now enable worker mode after connect so we have pg_pool but no channel
        config.worker.enable = true;
        (db, Arc::new(config))
    }

    #[tokio::test]
    #[serial]
    async fn test_v1_bulk_results_csv_format() {
        let (db, config) = worker_db_config().await;
        let pool = db.pool_owned();

        // Insert a completed job with results
        let job_id = insert_job(&pool, None, 1, "completed").await;
        insert_task(&pool, job_id, "completed", None, Some(safe_result()), None).await;

        let routes = reacher_backend::http::create_routes(config);

        let resp = request()
            .path(&format!("/v1/bulk/{}/results?format=csv", job_id))
            .method("GET")
            .reply(&routes)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
        let content_type = resp
            .headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap();
        assert!(
            content_type.contains("text/csv"),
            "Expected text/csv, got {}",
            content_type
        );
        let body = String::from_utf8_lossy(resp.body());
        assert!(body.contains("test@example.com"));
        assert!(body.contains("safe"));
    }

    #[tokio::test]
    #[serial]
    async fn test_v1_bulk_results_json_format() {
        let (db, config) = worker_db_config().await;
        let pool = db.pool_owned();

        let job_id = insert_job(&pool, None, 1, "completed").await;
        insert_task(&pool, job_id, "completed", None, Some(safe_result()), None).await;

        let routes = reacher_backend::http::create_routes(config);

        let resp = request()
            .path(&format!("/v1/bulk/{}/results?format=json", job_id))
            .method("GET")
            .reply(&routes)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
        let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
        assert!(body["results"].is_array());
        assert_eq!(body["results"].as_array().unwrap().len(), 1);
    }

    #[tokio::test]
    #[serial]
    async fn test_v1_bulk_results_running_job_rejects() {
        let (db, config) = worker_db_config().await;
        let pool = db.pool_owned();

        // Job expects 5 records but only 1 completed
        let job_id = insert_job(&pool, None, 5, "running").await;
        insert_task(&pool, job_id, "completed", None, Some(safe_result()), None).await;

        let routes = reacher_backend::http::create_routes(config);

        let resp = request()
            .path(&format!("/v1/bulk/{}/results", job_id))
            .method("GET")
            .reply(&routes)
            .await;

        assert_ne!(
            resp.status(),
            StatusCode::OK,
            "Running job should not return results"
        );
    }

    #[tokio::test]
    #[serial]
    async fn test_v1_bulk_results_csv_with_multiple_records() {
        let (db, config) = worker_db_config().await;
        let pool = db.pool_owned();

        let job_id = insert_job(&pool, None, 2, "completed").await;
        insert_task(&pool, job_id, "completed", None, Some(safe_result()), None).await;

        let mut result2 = safe_result();
        result2["input"] = serde_json::json!("other@test.com");
        result2["syntax"]["address"] = serde_json::json!("other@test.com");
        result2["syntax"]["username"] = serde_json::json!("other");
        result2["syntax"]["domain"] = serde_json::json!("test.com");
        insert_task(&pool, job_id, "completed", None, Some(result2), None).await;

        let routes = reacher_backend::http::create_routes(config);

        let resp = request()
            .path(&format!("/v1/bulk/{}/results?format=csv", job_id))
            .method("GET")
            .reply(&routes)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
        let body = String::from_utf8_lossy(resp.body());
        assert!(body.contains("test@example.com"));
        assert!(body.contains("other@test.com"));
    }

    #[tokio::test]
    #[serial]
    async fn test_v1_bulk_results_with_api_key_auth() {
        let (db, config) = worker_db_config().await;
        let pool = db.pool_owned();

        // Create tenant and API key
        let tenant_id = insert_tenant(&pool, "csv-test", None, 0).await;
        let (api_key, _) = insert_api_key(&pool, tenant_id).await;

        let job_id = insert_job(&pool, Some(tenant_id), 1, "completed").await;
        insert_task(
            &pool,
            job_id,
            "completed",
            Some(tenant_id),
            Some(safe_result()),
            None,
        )
        .await;

        let routes = reacher_backend::http::create_routes(config);

        let resp = request()
            .path(&format!("/v1/bulk/{}/results?format=csv", job_id))
            .method("GET")
            .header("Authorization", format!("Bearer {}", api_key))
            .reply(&routes)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
        let body = String::from_utf8_lossy(resp.body());
        assert!(body.contains("test@example.com"));
    }

    #[tokio::test]
    #[serial]
    async fn test_v1_bulk_results_pagination() {
        let (db, config) = worker_db_config().await;
        let pool = db.pool_owned();

        let job_id = insert_job(&pool, None, 3, "completed").await;
        for _ in 0..3 {
            insert_task(&pool, job_id, "completed", None, Some(safe_result()), None).await;
        }

        let routes = reacher_backend::http::create_routes(config);

        let resp = request()
            .path(&format!(
                "/v1/bulk/{}/results?format=json&limit=1&offset=0",
                job_id
            ))
            .method("GET")
            .reply(&routes)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);
        let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
        assert_eq!(body["results"].as_array().unwrap().len(), 1);
    }
}
