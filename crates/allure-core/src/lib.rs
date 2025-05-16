// crates/allure-core/src/lib.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct TestResult {
    pub uuid: String,
    pub name: String,
    pub status: String,
    pub steps: Vec<TestStep>,
    pub attachments: Vec<Attachment>,
    pub labels: HashMap<String, String>,
    pub parameters: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestStep {
    pub name: String,
    pub status: String,
    pub start: u64,
    pub stop: u64,
    pub steps: Vec<TestStep>,
    pub attachments: Vec<Attachment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment {
    pub name: String,
    pub source: String,
    pub r#type: String,
}

pub trait Plugin: Send + Sync {
    fn process(&self, results: &mut Vec<TestResult>);
}
