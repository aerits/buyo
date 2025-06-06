use wasm_bindgen::prelude::*;
use web_sys::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "performance")]
    #[allow(non_camel_case_types)]
    pub type performance;

    #[wasm_bindgen(static_method_of = performance)]
    pub fn now() -> f64; // Returns time in milliseconds since the page was loaded
}

#[wasm_bindgen]
pub fn get_current_time() -> u64 {
    // Get the current time in milliseconds and convert to nanoseconds
    (performance::now()) as u64
}

// pub fn get_current_time() -> u64 {
//     return 0;
// }
