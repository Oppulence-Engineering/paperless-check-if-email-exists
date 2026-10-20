#![recursion_limit = "256"]

pub mod bounce_risk;
pub mod config;
pub mod decision;
pub mod delayed_recheck;
pub mod finder;
pub mod http;
pub mod list_intelligence;
pub mod outcomes;
pub mod pipelines;
pub mod reputation;
pub mod research;
pub mod reverification;
pub mod sandbox;
pub mod scoring;
pub mod smtp_egress;
pub mod storage;
pub mod suppression;
pub mod tenant;
pub mod throttle;
pub mod verification;
pub mod worker;

const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
