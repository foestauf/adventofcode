# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Advent of Code solutions implemented in Rust.

## Build and Run Commands

```bash
# Build the project
cargo build

# Run the project (interactive - prompts for year and problem number)
cargo run

# Build release version
cargo build --release
```

## Architecture

The codebase follows a hierarchical module structure organized by year and problem:

```
src/
├── main.rs              # Entry point - prompts for year selection
└── problems/
    ├── mod.rs           # Year dispatcher (select_year function)
    └── year_YYYY/
        ├── mod.rs       # Problem dispatcher for that year
        └── problem_N/
            ├── mod.rs   # Solution with solve() function
            └── input.txt # Puzzle input
```

**Key patterns:**
- Each problem module exports a `solve()` function
- Input files are read from `src/problems/year_YYYY/problem_N/input.txt`
- Adding a new year requires: new year module, entry in `problems/mod.rs` dispatcher
- Adding a new problem requires: new problem module, mod declaration and match arm in year's `mod.rs`
