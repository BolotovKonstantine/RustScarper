# Overview

This project aims to deepen my understanding of Rust by creating a web scraper that collects and organizes the latest documentation from a target website. The extracted data can be used to support AI-based Retrieval-Augmented Generation (RAG) workflows. By building this from the ground up, I gained insights into Rust’s concurrency features, error handling patterns, and overall language design.

The software demonstrates how to:
- Recursively crawl a website to gather text-based information.
- Parse HTML content.
- Store and structure the scraped data for reuse.

[Software Demo Video](http://youtube.link.goes.here)  
(This is a short demonstration of the scraper at work and a walkthrough of what I learned in terms of language syntax and best practices.)

# Development Environment

- Operating System: macOS Sonoma (Apple Silicon)
- Editor: JetBrains RustRover 2024.3.7
- Programming Language: Rust 1.85.1 (or above)
- Libraries:
    - reqwest (for HTTP requests)
    - tokio (for async runtime)
    - scraper (for HTML parsing)
    - serde / serde_json (for serialization)

# Useful Websites

- [Rust Official Documentation](https://doc.rust-lang.org)
- [Docs.rs - Library Documentation](https://docs.rs)

# Future Work

- Integrate a filtering mechanism to skip irrelevant pages during recursion.
- Build a front-end viewer for the scraped documentation.
- Develop a customized pipeline for feeding documentation into an AI model more seamlessly.  