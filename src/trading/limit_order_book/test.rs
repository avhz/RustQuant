// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
use super::Book;

#[test]
fn add_order_buy() {
    let mut book = Book::new();

    book.add_order(1, true, 2, 10, 1000).unwrap();

    assert!(book.order_map.contains_key(&1));
    assert_eq!(book.order_map.get(&1).unwrap().order_id, 1);
    assert!(book.buy_limits.contains_key(&10));
    assert_eq!(book.buy_limits.get(&10).unwrap().limit_price, 10);
}

#[test]
fn add_order_buy_many() {
    let mut book = Book::new();

    book.add_order(1, true, 2, 10, 1000).unwrap();
    book.add_order(2, true, 2, 20, 1000).unwrap();

    assert!(book.order_map.contains_key(&1));
    assert!(book.order_map.contains_key(&2));
    assert!(book.buy_limits.contains_key(&10));
    assert!(book.buy_limits.contains_key(&20));
    assert_eq!(book.buy_limits.values().next().unwrap().limit_price, 10);
    assert_eq!(book.buy_limits.values().last().unwrap().limit_price, 20);
}

#[test]
fn add_order_sell() {
    let mut book = Book::new();

    book.add_order(2, false, 3, 10, 1000).unwrap();

    assert!(book.order_map.contains_key(&2));
    assert_eq!(book.order_map.get(&2).unwrap().order_id, 2);
    assert!(book.sell_limits.contains_key(&10));
    assert_eq!(book.sell_limits.get(&10).unwrap().limit_price, 10);
}

#[test]
fn add_existing_buy() {
    let mut book = Book::new();

    book.add_order(1, true, 2, 10, 1000).unwrap();

    assert!(book.add_order(1, true, 2, 10, 1000).is_err());
}

#[test]
fn add_existing_sell() {
    let mut book = Book::new();

    book.add_order(1, false, 2, 10, 1000).unwrap();

    assert!(book.add_order(1, false, 2, 10, 1000).is_err());
}

#[test]
fn cancel_order_buy() {
    let mut book = Book::new();

    book.add_order(1, true, 2, 10, 1000).unwrap();

    book.cancel_order(1).unwrap();

    assert!(!book.order_map.contains_key(&1));
    assert!(!book.buy_limits.contains_key(&10));
}

#[test]
fn cancel_order_sell() {
    let mut book = Book::new();

    book.add_order(1, false, 2, 10, 1000).unwrap();

    book.cancel_order(1).unwrap();

    assert!(!book.order_map.contains_key(&1));
    assert!(!book.sell_limits.contains_key(&10));
}

#[test]
fn execute_market_buy() {
    let mut book = Book::new();

    book.add_order(1, false, 3, 10, 1000).unwrap();

    let (is_executed, share_status) = book.execute_market_order(2, true);

    assert!(is_executed);
    assert_eq!(share_status[0].0, 10);
    assert_eq!(share_status[0].1, 2);

    assert!(book.order_map.contains_key(&1));
    assert_eq!(book.order_map.get(&1).unwrap().shares, 1);
}

#[test]
fn unable_execute_market_buy() {
    let mut book = Book::new();

    book.add_order(1, false, 1, 10, 1000).unwrap();

    let (is_executed, share_status) = book.execute_market_order(2, true);

    assert!(!is_executed);
    assert_eq!(share_status[0].0, 10);
    assert_eq!(share_status[0].1, 1);

    assert!(!book.order_map.contains_key(&1));
}
