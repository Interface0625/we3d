use web_sys::WebGlRenderingContext as GL;
use super::programs;
use nalgebra as na;
use na::{Vector2};

pub struct PlayArea {
    current_board: [u8; 81],
    solution_board: [u8; 81],
    initial_board: [u8; 81],
    square_size: f32,
    minor_width: f32,
    major_width: f32,
    board_size: f32,
    rect_2d: programs::Rect2D,
    number_2d: programs::Number2D,
    current_cell_rect: [f32; 4],
    current_cell_index: i32,
    last_cell_index: i32,
    mouse_position: Vector2<f32>,
}

impl PlayArea{
    pub fn new(gl: &GL)-> Self{
        Self {
            current_board: [1; 81],
            solution_board: [1; 81],
            initial_board: [1; 81],
            rect_2d: programs::Rect2D::new(gl),
            number_2d: programs::Number2D::new(gl),
            current_cell_index: -1,
            square_size: 50.,
            minor_width: 1.0,
            major_width: 6.5,
            board_size: 9. * 50.,
            current_cell_rect: [-1.; 4],
            mouse_position: Vector2::new(0.,0.),
            last_cell_index: -1,
        }
    }
    fn render_current_background(&self, gl: &GL, canvas_size: Vector2<f32>){
        let size = self.square_size;
        let program = &self.rect_2d;
        let r = self.current_cell_rect; // get_current_rect(canvas_width, canvas_height, self.mx, self.my);
        if r[0] != -1.0 {
            program.render(
                &gl, 
                r[3], 
                r[3] + size, 
                r[2], 
                r[2] + size, 
                canvas_size.x,
                canvas_size.y, 
                0.5,
            ); 
        }
    }
    fn render_grid(&self, gl: &GL, canvas_size: Vector2<f32>){
        let size = self.square_size;
        let minor = self.minor_width;
        let major = self.major_width;
        let width = self.board_size;
        let height = self.board_size;
        let opacity = 0.5;
        let offset_x = canvas_size.x / 2. - width / 2.;
        let offset_y = canvas_size.y / 2. - height / 2.;
        for x in 1..9 {
            if x % 3 != 0 {
                let x = x as f32;
                self.rect_2d.render(
                    gl, 
                    offset_y, // bottom
                    offset_y + height, // top
                    offset_x + x * size, // left
                    offset_x + x * size + minor, // right
                    canvas_size.x,
                    canvas_size.y, 
                    opacity,
                );
            }else{
                let x = x as f32;
                self.rect_2d.render(
                    gl, 
                    offset_y,
                    offset_y + height,
                    offset_x + x * size, 
                    offset_x + x * size + major,
                    canvas_size.x,
                    canvas_size.y, 
                    opacity,
                );
            }
        }
        for y in 1..9 {
            if y % 3 != 0 {
                let y = y as f32;
                self.rect_2d.render(
                    gl, 
                    offset_y + y * size, 
                    offset_y + y * size + minor, 
                    offset_x, 
                    offset_x + width, 
                    canvas_size.x,
                    canvas_size.y, 
                    opacity,
                );
            }else{
                let y = y as f32;
                self.rect_2d.render(
                    gl, 
                    offset_y + y * size, 
                    offset_y + y * size + major, 
                    offset_x, 
                    offset_x + width, 
                    canvas_size.x,
                    canvas_size.y,
                    opacity,
                );
            }
           
        }

    }

    fn render_numbers(&self, gl: &GL,canvas_size: Vector2<f32>){
        let offset_x = self.square_size * 0.3 / 2.;
        let offset_y = self.square_size * 0.22;
        let size = self.square_size * 0.7;

        let board_offset_x = canvas_size.x / 2. - self.board_size / 2.;
        let board_offset_y = canvas_size.y / 2. - self.board_size / 2.;

        let current_board = self.current_board;
        let solution_board = self.solution_board;
        let number_2d = &self.number_2d;

        for i in 0..81 {
            let number = current_board[i as usize];
            let x = i % 9;
            let y = (i - x) / 9;
            let x = x as f32 * (self.square_size) + board_offset_x + offset_x;
            let y = y as f32 * (self.square_size) + board_offset_y + offset_y;
            let position = Vector2::new(x, y);
            if number == 0 {
                number_2d.render(
                    &gl, 
                    solution_board[i] as usize, 
                    position, 
                    size, 
                    canvas_size, 
                    0.0
                );
            }else{
                if number != solution_board[i]{
                    number_2d.render(
                        &gl, 
                        number as usize, 
                        position, 
                        size, 
                        canvas_size, 
                        0.1,
                    );
                }else{
                    number_2d.render(
                        &gl, 
                        number as usize, 
                        position,  
                        size, 
                        canvas_size,  
                        0.8,
                    );
                }
            }
        }
    }
    pub fn update(&mut self, _time:f32){
        let curr_state = super::app_state::get_curr_state();
        let canvas_size = curr_state.canvas_size;
        let down = curr_state.mouse_down;
        let mouse_position = curr_state.mouse_position;
        if canvas_size.x < canvas_size.y 
        { self.board_size = canvas_size.x * 0.7 } 
        else 
        { self.board_size = canvas_size.y  * 0.7 };
        self.square_size = self.board_size / 9.;
        
        if !down {
            
            let r = get_current_rect(self.square_size, canvas_size, mouse_position);
            self.current_cell_rect = r;
            if r[0] != -1.0 {
                self.current_cell_index = r[0] as i32 + r[1] as i32 * 9;
            }
        }else{
            self.last_cell_index = self.current_cell_index;
        }
        self.mouse_position = mouse_position;

    }
    pub fn write_number(&mut self, number: u8){
        if self.last_cell_index != -1 && number != 0 {
            let index = self.last_cell_index as usize;
            if self.initial_board[index] == 0 {
                self.current_board[index] = number;
            }
        }
    }
    pub fn render(&self, gl: &GL, canvas_size: Vector2<f32>){
        self.render_grid(gl, canvas_size);
        self.render_current_background(gl, canvas_size);
        self.render_numbers(gl, canvas_size);
    }

}

fn inside_rect(point:Vector2<f32>, top:f32, bottom:f32, left:f32, right:f32) -> bool {
    point.y < bottom && point.y > top && point.x > left && point.x < right
}


fn get_current_rect(square_size:f32, canvas_size: Vector2<f32>, mouse_postion: Vector2<f32>) -> [f32; 4]{
    let size = square_size;
    let width = size * 9.;
    let height = size * 9.;
    let offset_x = canvas_size.x / 2. - width / 2.;
    let offset_y = canvas_size.y / 2. - height / 2.;
    for x in 0..9 {
        for y in 0..9 {
            let _y = offset_y + y as f32 * size;
            let _x = offset_x + x as f32 * size;
            if inside_rect(
                mouse_postion, 
                _y, _y + size, 
                _x, _x + size
            ) {
                return [x as f32 , y as f32, _x, _y]
            }
        }
    }
    return [-1.0,0.0,0.0,0.0]
}




