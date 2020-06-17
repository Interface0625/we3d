fn lerp(from: f32, to: f32, delta: f32) -> f32 {
    return (to - from) * delta + from;
}
fn min(a: f32, b: f32) -> f32 {
    if a > b { return a; }
    else { return b; }
}
fn max(a: f32, b: f32) -> f32 {
    if a < b { return a; }
    else { return b; }
}


pub struct LerpOverTime {
    pub start: f32,
    pub length: f32,
}

impl LerpOverTime {
    pub fn new(
        start: f32, 
        length: f32
    ) -> Self {        
        Self {
            start,
            length
        }
    }
    pub fn get_delta(&self, time:f32) -> f32 {
        let delta = (time - self.start) / self.length;
        return min(max(delta, 1.0), 0.0);
    }
    pub fn lerp(&self, time:f32, from: f32, to: f32) -> f32 {
        let delta = self.get_delta(time);
        return lerp(from, to, delta);
    }
}