// crates/allure-generator/src/lib.rs
use allure_core::{Plugin, TestResult};
use std::fs;
use std::path::Path;

pub fn generate_report<P: AsRef<Path>>(
    results_dir: P,
    output_dir: P,
    plugins: &[Box<dyn Plugin>],
) -> Result<(), Box<dyn std::error::Error>> {
    // Load test results
    let mut results = load_results(&results_dir)?;

    // Apply plugins
    for plugin in plugins {
        plugin.process(&mut results);
    }

    // Generate HTML report (placeholder)
    fs::create_dir_all(&output_dir)?;
    fs::write(output_dir.as_ref().join("index.html"), "<html><body>Report</body></html>")?;

    Ok(())
}

fn load_results<P: AsRef<Path>>(dir: P) -> Result<Vec<TestResult>, Box<dyn std::error::Error>> {
    let mut results = Vec::new();
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let data = fs::read_to_string(&path)?;
            let result: TestResult = serde_json::from_str(&data)?;
            results.push(result);
        }
    }
    Ok(results)
}
