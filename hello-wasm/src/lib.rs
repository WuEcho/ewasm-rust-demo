use wasm_bindgen::prelude::*;
use ewasm_api::types::*;
use ewasm_api::metavm::utils::*;

const COUNTER_KEY: Bytes32 = Bytes32 { bytes: [255; 32] };

fn inc_counter() {
    let old_v = ewasm_api::storage_load(&COUNTER_KEY);
    let old_i = bytes_to_uint(&old_v.bytes[..]);
    let new_i = old_i + 1;
    let val = u32_to_bytes32(new_i as u32);
    let value = Bytes32 { bytes: val };
    ewasm_api::storage_store(&COUNTER_KEY, &value);
}

fn get_counter() {
    let v = ewasm_api::storage_load(&COUNTER_KEY);
    ewasm_api::finish_data(&v.bytes[..]);
}

fn put_data() {
    let input = ewasm_api::calldata_acquire();
    let data = String::from_utf8(input).expect("error_params");
    let sd: Vec<&str> = data.split(":").collect();
    if sd.len() > 1 {
        let sp: Vec<&str> = sd[1].split(",").collect();
        if sp.len() > 1 {
            let k = sp[0].trim();
            let v = sp[1].trim();
            ewasm_api::metavm::storage_store(k.as_bytes(), v.as_bytes());
        }
    }
}

fn get_data() {
    let input = ewasm_api::calldata_acquire();
    let data = String::from_utf8(input).expect("error_params");
    let sd: Vec<&str> = data.split(":").collect();
    if sd.len() > 1 {
        let k = sd[1].trim();
        let v: Vec<u8> = ewasm_api::metavm::storage_load(k.as_bytes());
        ewasm_api::finish_data(&v[..]);
    }
}
 
fn anonymous() {
    
}

#[wasm_bindgen]
pub fn main() {
    inc_counter();
    let input = ewasm_api::calldata_acquire();
    if !input.is_empty() {
        let data = match String::from_utf8(input) {
            Ok(s) => s,
            Err(e) => e.to_string(),
        };
        let sd: Vec<&str> = data.split(":").collect();
        match sd[0].trim().to_uppercase().as_str() {
            "GETCOUNTER" => get_counter(),
            "PUT" => put_data(),
            "GET" => get_data(),
            _ => ewasm_api::finish_data(String::from("METHOD_NOT_FOUND").as_bytes()),
        }
    } else {
        anonymous();
    }
}

