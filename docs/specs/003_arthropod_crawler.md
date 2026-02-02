# 🔭 Product Spec: Arthropod

**Status**: Draft
**Owner**: Vantage
**Priority**: P2

## 1. User Story

> "As a **Data Scientist**, I want to **crawl specific domains with high concurrency but absolute respect for `robots.txt`**, so that I can **turn the open web into structured datasets for model training**."

## 2. Context & Gap Analysis

**The Problem:**
Building a web crawler is deceptively simple until you hit edge cases: infinite loops, rate limits, angry sysadmins blocking your IP, and parsing malformed HTML.
Existing tools are either too simple (`curl`, `wget`) or too complex (`scrapy` requires Python boilerplate). There is no "batteries-included" binary for ethical, high-performance crawling.

**The "So What?":**
Data is the fuel for AI. Arthropod aims to be the most reliable fuel pump. It separates the "fetching" (IO) from the "extraction" (ETL), allowing users to focus on the data, not the network.

## 3. Success Metrics

*   **Throughput**: > 100 pages/second on a single node (Consumer Internet).
*   **Compliance**: 0 violations of `robots.txt`.
*   **Resilience**: Must survive network partitions and resume from the last known state (WAL).

## 4. Acceptance Criteria

### Core Functionality
- [ ] **Politeness Engine**: Built-in parser for `robots.txt` and `Sitemap.xml`. Automatic backoff on 429/503 errors.
- [ ] **Domain Scoping**: Strict whitelisting of domains to prevent "escaping" the target site.
- [ ] **Structure Extraction**: Support for CSS Selectors and XPath to extract JSON fields from HTML.
- [ ] **Output Sink**: Stream results to JSONL files or directly to **GallifreyDB**.

### Usability
- [ ] **Configuration**: Single YAML file defines seed URLs, limits, and extraction rules.
- [ ] **Dashboard**: TUI (Ratatui) showing active connections, queue size, and error rates.

## 5. Out of Scope (Phase 1)

*   🚫 **JavaScript Rendering**: No headless browser (Puppeteer/Playwright). Static HTML only.
*   🚫 **CAPTCHA Solving**: Explicitly out of scope. If we are blocked, we stop.
*   🚫 **Proxy Rotation**: User must provide their own proxy solution if needed.

## 6. Philosophy Alignment
*   **Ethics**: "Be a good citizen of the web."
*   **Performance**: Async Rust (Tokio) for massive IO concurrency.
