#![feature(proc_macro, wasm_import_module, wasm_custom_section)]
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub extern "C" fn fact(mut n: u32) -> u64 {
    let mut n = n as u64;
    let mut result = 1;
    while n > 0 {
        result = result * n;
        n = n - 1;
    }
    result
}

#[wasm_bindgen]
pub extern "C" fn fact_str(n: u32) -> *mut c_char {
    let res = fact(n);
    let s = format!("{}", res);
    let s = CString::new(s).unwrap();
    alert(s.into_raw());
    s.into_raw()
}