use wasm_bindgen::prelude::*;
use ruint::aliases::U256;
use serde::Serialize;
use serde_wasm_bindgen::Serializer;
use wasm_bindgen::JsValue;
use serde_wasm_bindgen::from_value;

#[wasm_bindgen]
pub fn calc_witness(inputs: &str, graph_data: &[u8]) -> JsValue {
    let witness: Vec<u8> = crate::calc_witness(inputs, graph_data)
        .iter()
        .flat_map(|u256_vec| u256_vec.iter().flat_map(|u256| u256.to_be_bytes_vec()))
        .collect();
    witness.serialize(&Serializer::new().serialize_maps_as_objects(true)).unwrap()
}

#[wasm_bindgen]
pub fn wtns_from_witness(witness: JsValue) -> JsValue {
    let witness: Vec<u8> = from_value(witness).unwrap();
    let witness: Vec<U256> = witness.chunks(32).map(|chunk| U256::from_be_bytes::<32>(chunk.try_into().unwrap())).collect();
    let result: Vec<u8> = crate::wtns_from_witness(witness);
    result.serialize(&Serializer::new().serialize_maps_as_objects(true)).unwrap()
}