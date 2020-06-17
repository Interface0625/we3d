use nalgebra as na; 
use na::{Vector2, Vector3, Point3, Matrix4, Perspective3, Isometry3};


pub struct Camera {
    fovy: f32,
    aspect: f32,
    znear: f32,
    zfar: f32,
    pub position: Point3<f32>,
    pub v_matrix: Matrix4<f32>,
    pub p_matrix: Matrix4<f32>,
}
impl Camera {
    pub fn new() -> Self {
        let position = Point3::new( 0., 0.1, 50.,);
        Self{
            fovy: 1.63,
            aspect: 1280./840.,
            znear: 0.1,
            zfar: 1000.,
            v_matrix: Matrix4::new_translation( &Vector3::new( 0., 1., -20.,) ),
            p_matrix: Matrix4::new_translation( &Vector3::new( position.x, position.y, position.z) ),
            position,
        }
    }
    pub fn update(&mut self, time: f32, canvas_size: Vector2<f32>){
        let delta = time % 18000. / 18000.;
        let radians = std::f32::consts::PI * 2.0 * delta;
        let eye = Point3::new(radians.sin() * 50., radians.cos() * 36.8, radians.cos() * 50.);
        let target = Point3::new(0.,0.,0.);

        self.aspect = canvas_size.x / canvas_size.y;
        self.fovy = std::f32::consts::PI/180. * 80.;
        self.p_matrix = Perspective3::new(self.aspect, self.fovy, self.znear, self.zfar).to_homogeneous();
 
        self.v_matrix = Isometry3::look_at_rh( 
            &eye,
            &target,
            &Vector3::new( 0., 1., 0.,),
        ).to_homogeneous();
    }
}
