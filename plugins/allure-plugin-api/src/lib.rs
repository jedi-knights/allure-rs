// plugins/allure-plugin-api/src/lib.rs
use allure_core::{Plugin, TestResult};

pub struct ExamplePlugin;

impl Plugin for ExamplePlugin {
    fn process(&self, results: &mut Vec<TestResult>) {
        for result in results {
            result.labels.insert("processed_by".to_string(), "ExamplePlugin".to_string());
        }
    }
}
