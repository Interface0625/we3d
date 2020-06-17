use web_sys::WebGlRenderingContext as GL;
use web_sys::*;
use wasm_bindgen::JsCast;
use js_sys::WebAssembly;

extern crate nalgebra as na;
use na::{Vector2, Vector3, Matrix4};

//    GL
pub fn link_program(
    gl: &GL,
    vert_source: &str, 
    frag_source: &str, 
) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or_else(|| String::from("Error creating program"))?;

    let vert_shader = compile_shader(&gl, GL::VERTEX_SHADER, vert_source).unwrap();
    let frag_shader = compile_shader(&gl, GL::FRAGMENT_SHADER, frag_source).unwrap();

    gl.attach_shader(&program, &vert_shader);
    gl.attach_shader(&program, &frag_shader);
    gl.link_program(&program);

    if gl.get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false) 
    {
        Ok(program)
    } else {
        Err(gl.get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unable to get shader info log")))
    }
}

pub fn init_vertex_buffer(gl: &GL, vertices: &[f32] ) -> Result<WebGlBuffer, String>{
    let memory_buffer = wasm_bindgen::memory().dyn_into::<WebAssembly::Memory>().unwrap().buffer();
    let vertices_location = vertices.as_ptr() as u32 / 4;
    let vertices_array = js_sys::Float32Array::new(&memory_buffer).subarray(
        vertices_location,
        vertices_location + vertices.len() as u32,
    );
    let buffer_vertices = gl.create_buffer().ok_or("failed to create buffer").unwrap();
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer_vertices));
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertices_array, GL::STATIC_DRAW);
    Ok(buffer_vertices)
}
pub fn init_index_buffer(gl: &GL, indices: &[u16]) -> Result<WebGlBuffer, String>{
    let indices_memory_buffer = wasm_bindgen::memory()
    .dyn_into::<WebAssembly::Memory>()
    .unwrap()
    .buffer();
    let indices_location = indices.as_ptr() as u32 / 2;
    let indices_array = js_sys::Uint16Array::new(&indices_memory_buffer).subarray(
        indices_location,
        indices_location + indices.len() as u32
    );
    let buffer_indices = gl.create_buffer().unwrap();
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&buffer_indices));
    gl.buffer_data_with_array_buffer_view(
        GL::ELEMENT_ARRAY_BUFFER,
        &indices_array,
        GL::STATIC_DRAW,
    );
    Ok(buffer_indices)
    
}

pub fn compile_shader(
    gl: &GL,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Error Creating shader"))?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl.get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false) 
        {
            Ok(shader)
        } else {
            Err(gl.get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unable to get shader info log")))
        }
}

//    MATH
pub fn calc_matrix (bottom: f32, top: f32, left: f32, right: f32, canvas_width: f32, canvas_height: f32)-> Matrix4<f32>{
    let translation_matrix = Matrix4::new_translation( &Vector3::new(
        2. * left / canvas_width - 1., 
        2. * bottom / canvas_height - 1., 
        0.
    ));

    let scaling_matrix = Matrix4::new_nonuniform_scaling( &Vector3::new(
        2. * (right - left) / canvas_width,
        2. * (top - bottom) / canvas_height, 
        0.
    ));
    translation_matrix * scaling_matrix
}

pub fn ease_in_out(t: f32) -> f32 {
    let t = t * 2.0;
    if t < 1.0 {
        0.5 * (t * t * t * t * t)
    } else {
        let t = t - 2.0;
        0.5 * ((t * t * t * t * t) + 2.0)
    }
}

pub fn inside_circle(x:f32, y:f32, cx:f32, cy:f32, radius:f32) -> bool {
    let _x = x - cx;
    let _y = y - cy;
    let len = (_x * _x + _y *_y).sqrt();
    len < radius
}
pub fn inside_selection_number(position: Vector2<f32>, radius:f32, point: Vector2<f32>, scale: f32)-> usize {
    for i in 1..10 {
        let radians = std::f32::consts::PI * 2.0 / 9.0 * i as f32;
        let _x = radians.sin() * scale + position.x;
        let _y = radians.cos() * scale + position.y;
        if inside_circle(point.x, point.y, _x, _y, radius){
            return i;
        }
    }
    return 0;
}
