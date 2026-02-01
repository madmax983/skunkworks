# 🔭 Product Spec: Thorp Backtester

**Status**: Draft
**Owner**: Vantage
**Priority**: P1

## 1. User Story

> "As a **Quantitative Researcher**, I want to **simulate trading strategies against high-resolution historical data**, so that I can **validate their profitability and risk profile before deploying capital**."

## 2. Context & Gap Analysis

**The Problem:**
Currently, strategy research is often done in Python (Pandas/Backtrader). While flexible, these solutions suffer from:
1.  **Performance bottlenecks** when processing tick-level data or performing multi-parameter optimization loops.
2.  **Type safety issues** leading to runtime errors during long backtests.
3.  **Look-ahead bias** risks in vectorized backtesting approaches.

**The "So What?":**
Speed of iteration is the primary competitive advantage in quantitative finance. If a backtest takes 1 hour, a researcher can only try 8 ideas a day. If it takes 1 minute, they can try 480. **Thorp** aims to reduce the feedback loop from hours to seconds.

## 3. Success Metrics

*   **Throughput**: The engine must process > **1,000,000 events per second** on standard commodity hardware.
*   **Safety**: Zero panics on malformed data (NaNs, missing fields).
*   **Accuracy**: 100% deterministic results for the same input data and strategy parameters.

## 4. Acceptance Criteria

### Core Functionality
- [ ] **Data Ingestion**: Must support reading historical data from CSV and Parquet formats.
- [ ] **Event-Driven Architecture**: The engine must simulate a clock, feeding events (Ticks, Candles) to the strategy one by one to prevent look-ahead bias.
- [ ] **Robust Math**: Must handle floating-point anomalies (NaN, Inf) gracefully using `Option` or specialized wrappers (e.g., `decimal` or checked float wrappers). **Panicking on bad data is unacceptable.**
- [ ] **Reporting**: Must generate a summary report including:
    - Total Return
    - Sharpe Ratio
    - Max Drawdown
    - Win/Loss Ratio
- [ ] **Output**: Must optionally export a trade log (CSV) of all executions.

### Usability
- [ ] **Simple API**: Defining a strategy should require implementing a single Trait (e.g., `Strategy::on_tick(&mut self, event: Event)`).

## 5. Out of Scope (Phase 1)

*   🚫 **Live Execution**: Connection to real broker APIs (IBKR, Binance) is explicitly out of scope.
*   🚫 **GUI**: No web interface or complex GUI. A simple TUI (Ratatui) for progress monitoring is acceptable but not required.
*   🚫 **Machine Learning Integration**: No native tensor bindings in v1.

## 6. Philosophy Alignment
*   **Utility over Complexity**: Do not over-engineer the event bus. A simple loop is faster and easier to debug than a complex async actor system for backtesting.
*   **Metric-Driven**: If it's not faster than Python, we don't build it.
