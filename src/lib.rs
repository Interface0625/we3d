use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;
use js_sys::Promise;
use wasm_bindgen_futures::{future_to_promise};


//#[macro_use]
//extern crate approx; // For the macro relative_eq!
#[macro_use]
extern crate lazy_static;

mod app_state;
mod common_funcs;
mod gl_setup;
mod shaders;
mod programs;
mod play_area;
mod lerp_over_time;
mod cursor;
mod g3d;
use nalgebra::Vector2;
mod data;
mod gl_helper;
mod camera;
mod content;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, Ohla, {}!", name));
    log("Hi Console!")
}

#[wasm_bindgen]
pub struct RustClient {
    gl: GL,
    play_area: play_area::PlayArea,
    cursor: cursor::Cursor,
    canvas_size: Vector2<f32>,
    ratio: f32,
    g3d_animation: g3d::G3d,
    camera: camera::Camera,
}

#[wasm_bindgen]
impl RustClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let gl = gl_setup::initialize_webgl_context().unwrap();
        let g3d_animation = g3d::G3d::new(&gl, &data::get_butterfly_json());
        log(&format!("{}", g3d_animation.meshes[0].attributes[0]));
        Self {
            play_area: play_area::PlayArea::new(&gl),
            cursor: cursor::Cursor::new(&gl),
            g3d_animation, 
            gl: gl,
            camera: camera::Camera::new(),
            canvas_size: Vector2::new(0.,0.),
            ratio: 0.,   
        }
    }

    pub fn load_all(&mut self, file_list: js_sys::Array) -> Promise {
        future_to_promise(content::content::load_all())
    }
    pub fn update(&mut self, time: f32, width: f32, height: f32) -> Result<(), JsValue> {
        // let curr_state = app_state::get_curr_state();
        self.canvas_size = Vector2::new(width, height);
        self.camera.update(time, self.canvas_size);
        self.ratio = self.canvas_size.x / self.canvas_size.y;
        self.play_area.update(time);
        self.cursor.update(time, &mut self.play_area);
        app_state::update_dynamic_data(time, self.canvas_size);
        let data = content::content::get_progress();
        //for data in _data{
            log(&format!("{}", data));
        //}
        Ok(())
    }

    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT | GL::STENCIL_BUFFER_BIT);
        self.play_area.render(&self.gl, self.canvas_size );
        self.cursor.render(&self.gl, self.canvas_size );
        self.g3d_animation.render(&self.gl, &self.camera);
    }
}

