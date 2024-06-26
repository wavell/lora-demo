#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Vec, Symbol, symbol_short, vec};

#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol_short!("Hello"), to]
    }
}

pub struct GeoCoordinate {
    latitude: i64, // Fixed-point representation
    longitude: i64, // Fixed-point representation
}

impl GeoCoordinate {
    pub fn new(latitude_deg: f64, longitude_deg: f64) -> Self {
        let scale_factor = 1_000_000.0; // Scale factor for precision
        let latitude = (latitude_deg * scale_factor) as i64;
        let longitude = (longitude_deg * scale_factor) as i64;

        Self { latitude, longitude }
    }

    // Methods to convert back to decimal degrees
    pub fn to_decimal_degrees(&self) -> (f64, f64) {
        let scale_factor = 1_000_000.0;
        let latitude_deg = self.latitude as f64 / scale_factor;
        let longitude_deg = self.longitude as f64 / scale_factor;

        (latitude_deg, longitude_deg)
    }
}

#[contract]
pub struct GeoContract;

#[contractimpl]
impl GeoContract {
    pub fn store_value(env: Env, key: Symbol, value: Vec<(i64, i64)>) {
        // let datetime = env.ledger().timestamp();
        // let value_and_date = vec![&env, (value, datetime)];
        env.storage().persistent().set(&key, &value);
        // env.storage().persistent().set(&key, &value_and_date);
    }

    // pub fn retrieve_value(env: Env, key: Symbol) -> Option<Vec<(i64, i64, u64)>> {
    pub fn retrieve_value(env: Env, key: Symbol) -> Option<Vec<(i64, i64)>> {
        env.storage().persistent().get(&key)
    }
}

mod test;
