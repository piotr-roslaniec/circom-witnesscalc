#![no_std]

use std::collections::HashMap;
use std::hash::Hash;
use std::io::Write;
use crate::build_circuit::Args;
use ruint::aliases::U256;
use serde::Serialize;
use serde_wasm_bindgen::from_value;
use serde_wasm_bindgen::Serializer;
use std::path::PathBuf;
use rust_embed::RustEmbed;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use vfs::{VfsPath, VfsError, MemoryFS, EmbeddedFS, FileSystem};

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
struct BuildCircuitArgs(Args);

#[wasm_bindgen]
impl BuildCircuitArgs {
    #[wasm_bindgen(constructor)]
    pub fn new(
        circuit_file: String,
        inputs_file: Option<String>,
        graph_file: String,
        link_libraries: Vec<String>,
        print_unoptimized: bool,
        print_debug: bool,
    ) -> Self {
        let link_libraries: Vec<PathBuf> = link_libraries.iter().map(PathBuf::from).collect();
        BuildCircuitArgs(Args {
            circuit_file,
            inputs_file,
            graph_file,
            link_libraries,
            print_unoptimized,
            print_debug,
        })
    }

    #[wasm_bindgen]
    pub fn to_js_object(&self) -> JsValue {
        JsValue::from_serde(&self.0).unwrap()
    }
}

impl BuildCircuitArgs {
    pub fn inner(&self) -> Args {
        self.0.clone()
    }
}

#[wasm_bindgen]
pub fn build_circuit(args: &BuildCircuitArgs, version: &str, files_map: &JsValue) -> JsValue {
    set_panic_hook();
    // wasm_logger::init(wasm_logger::Config::default());
    log::info!("Logging to console");
    let files_map: HashMap<String, String> = files_map.into_serde().unwrap();
    log::info!("Received files_map with {} entries", files_map.len());
    let circuit = crate::build_circuit::build_circuit_flow(&args.0, version, &files_map);
    log::info!("Circuit built");
    let circuit = JsValue::from_serde(&circuit).unwrap();
    log::info!("Circuit serialized");
    circuit
}

#[wasm_bindgen]
#[derive(Serialize)]
struct CalcWitnessResult {
    wtns_bytes: Vec<u8>,
    duration_micros: u128,
}

#[wasm_bindgen]
impl CalcWitnessResult {
    #[wasm_bindgen]
    pub fn to_js_object(&self) -> JsValue {
        JsValue::from_serde(&self).unwrap()
    }
}

#[wasm_bindgen]
pub fn calc_witness(inputs: &str, graph: &[u8]) -> CalcWitnessResult {
    set_panic_hook();
    // wasm_logger::init(wasm_logger::Config::default());

    // Now `graph` is a byte slice you can work with directly
    log::info!("Received inputs with {} characters", inputs.len());
    log::info!("Received graph with {} bytes", graph.len());

    let (wtns_bytes, duration_micros) = crate::calc_witness_flow(inputs, graph);
    log::info!("Witness calculated");

    CalcWitnessResult {
        wtns_bytes,
        duration_micros,
    }
}