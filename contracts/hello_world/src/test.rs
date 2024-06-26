#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, HelloContract);
    let client = HelloContractClient::new(&env, &contract_id);

    let words = client.hello(&symbol_short!("Dev"));
    assert_eq!(
        words,
        vec![&env, symbol_short!("Hello"), symbol_short!("Dev"),]
    );
}

#[test]
fn test_geo_coordinate_conversion() {
	// Test data: Known latitude and longitude in decimal degrees
	let lat_deg = 40.7128; // Example latitude
	let lon_deg = -74.0060; // Example longitude

	// Create a GeoCoordinate instance from decimal degrees
	let coord = GeoCoordinate::new(lat_deg, lon_deg);

	assert_eq!(coord.latitude, 40712800); // Expected scaled latitude
	assert_eq!(coord.longitude, -74006000); // Expected scaled longitude

	// Convert back to decimal degrees and assert accuracy
	let (lat_back, lon_back) = coord.to_decimal_degrees();
	assert_eq!(lat_back, lat_deg);
	assert_eq!(lon_back, lon_deg);
}

#[test]
fn test_store_and_retrieve_value() {
    let env = Env::default();
	let contract_id = env.register_contract(None, GeoContract);

	let client = GeoContractClient::new(&env, &contract_id);

	let key = symbol_short!("LoRaKey");
	let lat_deg = 40.7128; // Example latitude
	let lon_deg = -74.0060; // Example longitude
    let scale_factor = 1_000_000.0; // Scale factor for precision
    let latitude = (lat_deg * scale_factor) as i64;
    let longitude = (lon_deg * scale_factor) as i64;
	// let datetime = env.ledger().timestamp();
    let value = vec![&env, (latitude, longitude)];

	// Store the value
	client.store_value(&key, &value);

	// Retrieve the value and verify it
	let retrieved_value = client.retrieve_value(&key);
	assert_eq!(retrieved_value, Some(value));
	// assert_eq!(retrieved_value, Some((value, datetime)));
}
