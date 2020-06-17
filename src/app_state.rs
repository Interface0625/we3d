use std::sync::Arc;
use std::sync::Mutex;
use nalgebra::Vector2;

lazy_static! {
    static ref APP_STATE: Mutex< Arc < AppState > > = Mutex::new( Arc::new( AppState::new() ) );
}

pub fn update_dynamic_data(
    time: f32, 
    canvas_size: Vector2<f32>, 
) {
    let mut data = APP_STATE.lock().unwrap();

    *data = Arc::new(AppState {
        canvas_size,
        time: time,
        ..*data.clone()
    });
}
pub fn get_curr_state() -> Arc<AppState> {
    APP_STATE.lock().unwrap().clone()
}

pub struct AppState {
    pub canvas_size: Vector2<f32>,
    pub time: f32,
    pub mouse_down: bool,
    pub mouse_position: Vector2<f32>,
    pub rotation: Vector2<f32>,
}
impl AppState {
    fn new() -> Self {
        Self {
            canvas_size: Vector2::new(0.,0.),
            time: 0.,
            mouse_down: false,
            mouse_position: Vector2::new(0.,0.),
            rotation: Vector2::new(0.5, 0.5),
        }
    }
}
pub fn update_mouse_down(x:f32, y:f32, is_down:bool){
    let mut data = APP_STATE.lock().unwrap();
    let inverted_y = data.canvas_size.y - y;
    *data = Arc::new( AppState {
        mouse_down:is_down,
        mouse_position: Vector2::new( x, inverted_y ),
        ..*data.clone()
    })
}
pub fn update_mouse_position(x:f32,y:f32){
    let mut data = APP_STATE.lock().unwrap();
    let inverted_y = data.canvas_size.y - y;
    let delta_x = x - data.mouse_position.x;
    let delta_y = inverted_y - data.mouse_position.y;
    let rotation_x_delta = if data.mouse_down {
        std::f32::consts::PI * delta_x / data.canvas_size.x
    } else {
        0.
    };
    let rotation_y_delta = if data.mouse_down {
        std::f32::consts::PI * delta_y / data.canvas_size.y
    } else {
        0.
    };
    *data = Arc::new( AppState {
        mouse_position: Vector2::new( x, inverted_y ),
        rotation: Vector2::new( rotation_x_delta, rotation_y_delta),
        ..*data.clone()
    })
}