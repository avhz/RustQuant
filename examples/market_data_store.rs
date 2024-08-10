// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Market data container.

use std::any::{Any, TypeId};
use std::collections::{BTreeMap, HashMap};
use RustQuant::pricer::MarketData;
use RustQuant::time::Calendar;

fn main() {
    run();
}

/// Market data requirements.
pub trait MarketDataRequirements<C>
where
    C: Calendar,
{
    /// Check if the MarketData contains sufficient data for
    /// pricing the instrument.
    fn is_sufficient(&self, data: MarketData<C>) -> bool;
}

trait MarketDataId: Any {
    fn id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

// type MarketDataKey = (TypeId, String);

struct MarketDataStore {
    // data: HashMap<TypeId, HashMap<Box<str>, Box<dyn Any>>>,
    data: HashMap<Box<str>, Box<dyn Any>>,
}

#[derive(Debug, Default)]
struct Curve {
    nodes: BTreeMap<i32, f64>,
}

#[derive(Debug, Default)]
struct Strike {
    strike: f64,
}

#[derive(Debug, Default)]
struct Volatility {
    volatility: f64,
}

impl MarketDataId for Curve {}
impl MarketDataId for Strike {}
impl MarketDataId for Volatility {}

impl MarketDataStore {
    fn new() -> Self {
        MarketDataStore {
            data: HashMap::new(),
        }
    }

    fn insert<T: MarketDataId + 'static>(&mut self, key: Box<str>, value: T) {
        let type_id = TypeId::of::<T>();

        self.data
            .entry(type_id)
            .or_insert_with(HashMap::new)
            .insert(key, Box::new(value));
    }

    fn contains<T: MarketDataId + 'static>(&self) -> bool {
        self.data.contains_key(&TypeId::of::<T>())
    }

    fn get_all<T: MarketDataId + 'static>(&self) -> Vec<&T> {
        self.data
            .get(&TypeId::of::<T>())
            .map(|map| {
                map.values()
                    .filter_map(|boxed| boxed.downcast_ref::<T>())
                    .collect()
            })
            .unwrap_or_else(Vec::new)
    }

    fn get<T: MarketDataId + 'static>(&self, key: &str) -> Option<&T> {
        self.data
            .get(&TypeId::of::<T>())
            .and_then(|map| map.get(key))
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }
}

fn run() {
    let mut store = MarketDataStore::new();

    let curve1 = Curve::default();
    let curve2 = Curve::default();
    let strike1 = Strike::default();
    let strike2 = Strike::default();

    // Insert multiple items with user-defined keys
    store.insert("spot curve".into(), curve1);
    store.insert("forward curve".into(), curve2);
    store.insert("strike1".into(), strike1);
    store.insert("strike2".into(), strike2);

    if store.contains::<Curve>() {
        println!("Store contains Curve instances.");
    }

    if store.contains::<Strike>() {
        println!("Store contains Strike instances.");
    }

    // Retrieve a specific Curve instance by its key
    if let Some(curve) = store.get::<Curve>("spot curve") {
        println!("Retrieved specific Curve by key: {:?}", curve);
    }

    // Retrieve a specific Strike instance by its key
    if let Some(strike) = store.get::<Strike>("strike1") {
        println!("Retrieved specific Strike by key: {:?}", strike);
    }
}
