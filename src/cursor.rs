use web_sys::WebGlRenderingContext as GL;
use super::programs;
use super::common_funcs;
use super::lerp_over_time;
use nalgebra as na;
use na::{Vector2, Vector4};

pub struct Cursor {
    animation_started: bool,
    position: Vector2<f32>,
    mouse_position: Vector2<f32>,
    down: bool,
    circle_2d: programs::Circle2D,
    number_2d: programs::Number2D,
    delta: f32,
    lerper: lerp_over_time::LerpOverTime,
    pub selected_number: usize,
    size: f32,
}
impl Cursor {
    pub fn new(gl: &GL) -> Self{
        Self {
            down: false,
            animation_started: false,
            position: Vector2::new(0., 0.),
            circle_2d: programs::Circle2D::new(gl),
            number_2d: programs::Number2D::new(gl),
            delta: -1.,
            lerper: lerp_over_time::LerpOverTime::new(-1., 555.),
            mouse_position: Vector2::new (0.,0.),
            selected_number: 0,
            size:0.,
        }
    }

    
    fn render_number_animation(&self, gl: &GL, canvas_size: Vector2<f32> /*, x:f32, y:f32, _mx:f32, _my:f32*/) {
        let offset_x = 0.; //canvas_width / 2. - width / 2. - 50.*9. + 270.;
        let offset_y = 0.; //canvas_height / 2. - height / 2.- 50.*9. +205.;
        let scale = common_funcs::ease_in_out( self.delta ) * self.size * 1.1;

        for i in 1..10{
            let radians = std::f32::consts::PI * 2.0 / 9.0 * i as f32;
            let position = Vector2::new(
                radians.sin() * scale + self.position.x + offset_x,
                radians.cos() * scale + self.position.y + offset_y
            );
            if self.selected_number == i {
                self.circle_2d.render(gl, 
                    position,
                    self.size * 0.7 * 0.5 * self.delta, 
                    canvas_size,
                    Vector4::new(0.7, 0.7, 0.7, 0.9),
                );
            } else {
                self.circle_2d.render(gl, 
                    position,
                    self.size * 0.7 * 0.5 * self.delta, 
                    canvas_size,
                    Vector4::new(0.3, 0.5, 0.5, 0.5),
                );
            }
            self.number_2d.render( gl, i,
                Vector2::new( 
                    position.x - self.size * 0.32 * self.delta, 
                    position.y - self.size * 0.29 * self.delta
                ), 
                self.size * 0.65 * self.delta,
                canvas_size,
                1.0,
            );
        }        
    }
    fn render_cursor(&self, gl: &GL, canvas_size: Vector2<f32>){
        self.circle_2d.render(
            gl,
            self.mouse_position, 
            3., 
            canvas_size,
            Vector4::new(1.,1.,1.,1.)
        );
    }
    pub fn update(&mut self, time: f32, play_area: &mut super::play_area::PlayArea){
        let app_state = super::app_state::get_curr_state();
        self.mouse_position = app_state.mouse_position;
        self.down = app_state.mouse_down;

        let board_size = 
        if app_state.canvas_size.x < app_state.canvas_size.y
        { app_state.canvas_size.x * 0.7 } 
        else 
        {  app_state.canvas_size.y * 0.7 };
        self.size = board_size / 9.;

        if self.down {
            
            if self.animation_started {
                self.delta = self.lerper.lerp(time, 0., 1.);
                self.selected_number = common_funcs::inside_selection_number(
                    self.position, 
                    self.size * 0.7 * 0.5,
                    self.mouse_position,
                    self.size,
                );

            } else {  
                self.lerper.start = time;
                self.animation_started = true;
                self.position = self.mouse_position;
                self.delta = 0.;
            }
        }else{
            //self.selected_number = 0;
            if self.animation_started {
                //self.selected_number 
                play_area.write_number(self.selected_number as u8);
                self.selected_number = 0;
            }
            self.animation_started = false;
        }

    }
    pub fn render(&self, gl: &GL, canvas_size:Vector2<f32>,){
        if self.down{
            self.render_number_animation( gl, canvas_size );
        }
        self.render_cursor(gl, canvas_size);
    }
}