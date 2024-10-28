// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Trading related items.

/// Contains limit order book implementation
pub mod limit_order_book;

/// Order definition.
pub mod order;

/// Contains a limit orderbook (LOB) implementation.
pub mod order_book;

/// Order lifespan definitions.
pub mod order_lifespan;

/// Order side definitions.
pub mod order_side;

/// Order types definitions.
pub mod order_type;
