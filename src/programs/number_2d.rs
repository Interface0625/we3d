use web_sys::WebGlRenderingContext as GL;
use web_sys::*;
use super::super::common_funcs as cf;
use super::super::shaders;
use super::numbers;
use nalgebra as na;
use na::{Vector2};


pub struct Number2D {
    program: WebGlProgram,
    index_counts: [i32; 9],
    index_buffers: [ WebGlBuffer; 9 ],
    //vertex_count: usize,
    vertex_buffer: WebGlBuffer,
    u_color: WebGlUniformLocation,
    u_opacity: WebGlUniformLocation,
    u_transform: WebGlUniformLocation,
}


impl Number2D {

    pub fn new ( gl: &GL) -> Self {
        let program = cf::link_program(&gl, shaders::vertex::color_2d::SHADER, shaders::fragment::color_2d::SHADER).unwrap();
        let vertices = numbers::VERTICES;
        let buffer_vertices = cf::init_vertex_buffer(&gl, &vertices).unwrap();
        let ibs = [
            cf::init_index_buffer(&gl, &numbers::NUMBER_1).unwrap(),
            cf::init_index_buffer(&gl, &numbers::NUMBER_2).unwrap(),
            cf::init_index_buffer(&gl, &numbers::NUMBER_3).unwrap(),
            cf::init_index_buffer(&gl, &numbers::NUMBER_4).unwrap(),
            cf::init_index_buffer(&gl, &numbers::NUMBER_5).unwrap(),
            cf::init_index_buffer(&gl, &numbers::NUMBER_6).unwrap(),
            cf::init_index_buffer(&gl, &numbers::NUMBER_7).unwrap(),
            cf::init_index_buffer(&gl, &numbers::NUMBER_8).unwrap(),
            cf::init_index_buffer(&gl, &numbers::NUMBER_9).unwrap(),
        ];
        let ics = [
            numbers::NUMBER_1.len() as i32,
            numbers::NUMBER_2.len() as i32,
            numbers::NUMBER_3.len() as i32,
            numbers::NUMBER_4.len() as i32,
            numbers::NUMBER_5.len() as i32,
            numbers::NUMBER_6.len() as i32,
            numbers::NUMBER_7.len() as i32,
            numbers::NUMBER_8.len() as i32,
            numbers::NUMBER_9.len() as i32,
        ];

        Self {
            u_color: gl.get_uniform_location(&program, "uColor").unwrap(),
            u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
            u_transform: gl.get_uniform_location(&program, "uTransform").unwrap(),
            index_buffers: ibs,
            index_counts: ics,
            vertex_buffer: buffer_vertices,
            program: program,
        }
    }
    pub fn render(
        &self,
        gl: &GL, 
        number: usize,
        position: Vector2<f32>,
        size: f32,
        canvas_size: Vector2<f32>,
        opacity: f32,
    ){
        let offset_x = size * 0.17;
        let size = size * 1.7;
        let bottom = position.y;
        let top = position.y + size;
        let left = position.x + offset_x;
        let right = position.x + size + offset_x;


        self._render( 
            gl, 
            number, 
            bottom, 
            top, 
            left, 
            right, 
            canvas_size.x,
            canvas_size.y,
            opacity,
        );
    }
    fn _render(
        &self, 
        gl: &GL, 
        number: usize, 
        bottom: f32, 
        top: f32, 
        left: f32, 
        right: f32, 
        canvas_width: f32,
        canvas_height: f32,
        opacity: f32,
    ) {

        let transform_mat = cf::calc_matrix(bottom, top, left, right, canvas_width, canvas_height);
        gl.use_program(Some(&self.program));

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertex_buffer));

        gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        gl.uniform4f( Some(&self.u_color), 0.,0.5,0.5,1.0 );
        gl.uniform1f(Some(&self.u_opacity), opacity);
 

        gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &transform_mat.as_slice());
        match number {
            1..=9 =>gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.index_buffers[number-1])),
            _=> println!("not a number")
        };
        match number {
            1..=9 =>gl.draw_elements_with_i32(GL::TRIANGLES, self.index_counts[number-1], GL::UNSIGNED_SHORT, 0),
            _=> println!("not a number")
        };

              
    }
}