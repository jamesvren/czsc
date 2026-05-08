# CZSC-RS - Chan Zong San Chuan Rust Implementation

CZSC-RS is a Rust implementation of the CZSC (缠中说禅) technical analysis library. This crate provides automated recognition of Fenxing (分型), Bi (笔), and XianDuan (线段) concepts from Chan Theory, along with multi-level quantitative trading strategies.

## Features

- **Core Objects**: Complete implementation of core CZSC objects including RawBar, NewBar, FX (Fenxing), BI (Bi), ZS (Zhongshu), Signal, and Event
- **Analysis Functions**: Functions for identifying fenxing, bi, and other technical patterns
- **Signal Functions**: Comprehensive signal functions in multiple categories:
  - **bar**: Basic K-line signal functions
  - **pos**: Position-related signals for stop loss, take profit, trend analysis
  - **cxt**: Context-based signals for market state, trend strength, volatility, etc.
- **Bar Generator**: K-line generation and resampling capabilities
- **Enums**: All necessary enumerations for trading operations

## Signal Categories

### Bar Signals (`signals::bar`)
- `bar_single_v230506`: Single K-line trend factor assistance
- `bar_triple_v230506`: Triple K-line acceleration pattern with volume changes
- `bar_end_v221211`: Check if higher frequency K-line ends
- `bar_zdt_v230331`: Limit up/down signal detection

### Position Signals (`signals::pos`)
- `pos_ma_v230414`: Position MA breakout/undercut signals
- `pos_fx_stop_v230414`: Stop loss based on nearby fenxing
- `pos_bar_stop_v230524`: Stop loss based on N recent K-lines extremes
- `pos_holds_v230414`: Close position if N K-line gain is less than M%
- `pos_fix_exit_v230624`: Fixed percentage stop loss/profit taking

### Context Signals (`signals::cxt`)
- `cxt_ma_trend_v230414`: Determine if in bullish/bearish trend based on MA
- `cxt_ma_arrangement_v230414`: Determine trend based on MA arrangement
- `cxt_volatility_state_v230414`: Determine market volatility state
- `cxt_trend_strength_v230414`: Determine trend strength
- `cxt_market_sentiment_v230414`: Determine market sentiment
- `cxt_market_phase_v230414`: Determine market phase (bull/bear market stages)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
czsc-rs = { path = "./rust" }
```

Example usage:

```rust
use czsc_rs::*;

fn main() {
    // Example usage will be added here
}
```

## Architecture

The crate is organized into several modules:

- `enums`: All enumeration types (Operate, Mark, Direction, Freq)
- `objects`: Core data structures (RawBar, FX, BI, ZS, Signal, Event, CZSC)
- `analyze`: Analysis functions for identifying patterns
- `bar_generator`: K-line generation and resampling
- `signals`: Various signal functions grouped by category
  - `bar`: Basic K-line signals
  - `pos`: Position-related signals
  - `cxt`: Context-based signals

## Design Principles

1. **Performance**: Implemented in Rust for optimal performance
2. **Safety**: Memory safety guaranteed by Rust's ownership model
3. **Modularity**: Well-organized modules for different functionality
4. **Extensibility**: Easy to add new signal functions and features
5. **Compatibility**: Maintains compatibility with original CZSC concepts

## Testing

All modules include comprehensive unit tests to ensure correctness:

```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.