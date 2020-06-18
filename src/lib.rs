#[macro_use]
extern crate lazy_static;
use wasm_bindgen::prelude::*;

mod app_state;
mod common_funcs;
mod gl_setup;
mod shaders;
mod programs;
mod play_area;
mod lerp_over_time;
mod cursor;
mod g3d;
mod data;
mod gl_helper;
mod camera;
mod content;
pub mod rust_client;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

