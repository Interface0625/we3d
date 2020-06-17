use web_sys::WebGlRenderingContext as GL;
use web_sys::*;
use super::super::common_funcs as cf;
use super::super::shaders;
use nalgebra as na;
use na::{Vector2, Vector4};


pub struct Circle2D {
    program: WebGlProgram,
    //vertex_count: usize,
    vertex_buffer: WebGlBuffer,
    u_color: WebGlUniformLocation,
    u_opacity: WebGlUniformLocation,
    u_transform: WebGlUniformLocation,
}

const RESOLUTION: usize = 32;

impl Circle2D {
    pub fn new ( gl: &GL) -> Self {
        let program = cf::link_program(&gl, shaders::vertex::color_2d::SHADER, shaders::fragment::color_2d::SHADER).unwrap();
        let scale = 1.;
        
        let mut vertices =  [0.; RESOLUTION * 2 + 2 as usize];
        vertices[2] = 0.;
        vertices[3] = scale;
        for i in 1..RESOLUTION{
            let radians = std::f32::consts::PI * 2.0 / (RESOLUTION - 1) as f32 * (i-1) as f32;
            vertices[i*2 as usize] = radians.sin() * scale;
            vertices[i*2 + 1 as usize] = radians.cos() * scale;
        }
        vertices[RESOLUTION*2] = 0.;
        vertices[RESOLUTION*2+1] = scale;

        let buffer_vertices = cf::init_vertex_buffer(&gl, &vertices).unwrap();
      
        Self {
            u_color: gl.get_uniform_location(&program, "uColor").unwrap(),
            u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
            u_transform: gl.get_uniform_location(&program, "uTransform").unwrap(),
            vertex_buffer: buffer_vertices,
            //vertex_count: vertices.len(),
            program: program,
        }
    }

    pub fn render( &self, gl: &GL, position: Vector2<f32>, radius:f32, canvas_size: Vector2<f32>, color: Vector4<f32>) {
        let bottom = position.y;
        let top = position.y + radius;
        let left = position.x;
        let right = position.x + radius;
        let transform_mat = cf::calc_matrix(
            bottom, 
            top, 
            left, 
            right, 
            canvas_size.x, 
            canvas_size.y,
        );
        gl.use_program(Some(&self.program));

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertex_buffer));
        gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        gl.uniform4f( Some(&self.u_color), color.x, color.y, color.z, color.w );
        gl.uniform1f(Some(&self.u_opacity), 1.);
 

        gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &transform_mat.as_slice());
        let count = RESOLUTION  as i32 + 1;
        gl.draw_arrays(GL::TRIANGLE_FAN, 0, count);        
    }
}