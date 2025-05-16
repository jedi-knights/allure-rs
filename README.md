# allure-rs

> **Rust-powered test result visualizer and static web server for Allure-style reports.**
> Generate, process, and serve test results from `allure-results/` using a fast, modular, and extensible CLI—built for automation, CI pipelines, and local dev workflows.

---

## 🦀 Overview

**`allure-rs`** is a blazing-fast, modular alternative to [Allure2](https://github.com/allure-framework/allure2)—written entirely in Rust. It parses `allure-results` directories, generates clean static HTML reports, and serves them locally with a single command.

This tool is ideal for developers and QA teams who want a lightweight, dependency-free way to inspect and share test execution results.

---

## ✨ Features

* ⚡ Fast CLI for generating and serving reports
* 🔗 Compatible with `allure-results/` directories
* 🏗 Serve static reports via embedded HTTP server (powered by `axum`)
* 🔹 Plugin architecture for extensibility
* ⚙ Clean modular crate-based architecture
* 📊 Ready for CI/CD integration

---

## 💡 Use Cases

* Quickly preview test reports from your local machine
* Publish reports as part of CI/CD pipelines
* Extend reporting functionality via Rust plugins
* Replace Java-based Allure2 CLI with a performant Rust-native CLI

---

## 🚀 Quickstart

### 1. Install (from source)

```bash
cargo install --path .  # or build and run directly
```

### 2. Generate an HTML report

```bash
allure-rs generate --results ./allure-results --output ./allure-report
```

### 3. Serve the report on localhost

```bash
allure-rs serve --report ./allure-report --port 8080
```

Open `http://localhost:8080` in your browser.

---

## 📂 Project Structure

```text
allure-rs/
├── crates/
│   ├── allure-core/        # Core data models and plugin traits
│   ├── allure-generator/   # Report generation logic
│   ├── allure-server/      # HTTP server for report serving
│   └── allure-cli/         # CLI interface
└── plugins/
    ├── allure-plugin-api/      # Plugin interfaces
    └── allure-plugin-example/  # Example plugin
```

---

## 📁 Plugin Architecture

Plugins can hook into the report generation process to:

* Enrich test results
* Filter or transform input data
* Integrate with external systems

Example trait:

```rust
pub trait Plugin {
    fn process(&self, results: &mut Vec<TestResult>);
}
```

---

## 📝 License

This project is licensed under the [MIT License](./LICENSE).

---

## 🤝 Contributing

We welcome contributions! Feel free to open issues, submit PRs, or suggest improvements. To get started:

```bash
git clone https://github.com/your-org/allure-rs
cd allure-rs
cargo build
```

---

## 📊 Roadmap

* [ ] Dynamic plugin loading
* [ ] Improved HTML templating (Tera, Handlebars)
* [ ] Framework adapters (e.g., Rust, Python, JS)
* [ ] Dockerized CLI and web app

---

## 🚨 Disclaimer

`allure-rs` is not affiliated with the official [Allure](https://github.com/allure-framework/allure2) project. It aims to provide a Rust-native alternative for report generation based on the `allure-results/` structure.

---

## 📅 Maintainers

* [Your Name](https://github.com/your-handle) — creator & maintainer

---

## 🌍 Links

* Allure2 Reference: [https://github.com/allure-framework/allure2](https://github.com/allure-framework/allure2)
* Rust Lang: [https://www.rust-lang.org](https://www.rust-lang.org)
* Axum Web Framework: [https://github.com/tokio-rs/axum](https://github.com/tokio-rs/axum)
