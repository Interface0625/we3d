use web_sys::WebGlRenderingContext as GL;
use web_sys::*;
use super::super::common_funcs as cf;
use super::super::shaders;
use super::super::gl_helper as glh;
use nalgebra as na;
use na::{Vector1, Vector4};




pub struct Rect2D {
    program: WebGlProgram,
    index_count: i32,
    index_buffer: WebGlBuffer,
    //vertex_count: usize,
    vertex_buffer: WebGlBuffer,
    u_color: WebGlUniformLocation,
    u_opacity: WebGlUniformLocation,
    u_transform: WebGlUniformLocation,
}


impl Rect2D {

    pub fn new ( gl: &GL) -> Self {
        let program = cf::link_program(&gl, shaders::vertex::color_2d::SHADER, shaders::fragment::color_2d::SHADER).unwrap();
        let vertices: [f32; 8] = [ 0.,0., 1.,0., 1.,1., 0.,1., ];
        let buffer_vertices = cf::init_vertex_buffer(&gl, &vertices).unwrap();
        let indices: [u16; 6] = [0,2,1,0,3,2];
        let buffer_indices = cf::init_index_buffer(&gl, &indices).unwrap();
        Self {
            u_color: gl.get_uniform_location(&program, "uColor").unwrap(),
            u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
            u_transform: gl.get_uniform_location(&program, "uTransform").unwrap(),
            index_count: indices.len() as i32,
            index_buffer: buffer_indices,
            vertex_buffer: buffer_vertices,
            //vertex_count: vertices.len(),
            program: program,
        }
    }

    pub fn render( &self, gl: &GL, bottom: f32, top: f32, left: f32, right: f32, canvas_width: f32, canvas_height: f32, opacity: f32 ) {

        let transform_mat = cf::calc_matrix(bottom, top, left, right, canvas_width, canvas_height);
        gl.use_program(Some(&self.program));

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertex_buffer));
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.index_buffer));

        let attrib_ptr = gl.get_attrib_location(&self.program, "aPosition") as u32;
        gl.vertex_attrib_pointer_with_i32(attrib_ptr, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(attrib_ptr);

        glh::uniform_v4(gl, &self.u_color, Vector4::new( 0., 0.5, 0.5, 1.0) );
        glh::uniform_v1(gl, &self.u_opacity, Vector1::new( opacity ) );
        glh::uniform_matrix4(gl, &self.u_transform, transform_mat);

        gl.draw_elements_with_i32(GL::TRIANGLES, self.index_count, GL::UNSIGNED_SHORT, 0);        
    }
}