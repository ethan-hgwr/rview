use crate::{camera::CameraState, motion::Animation};

pub(crate) struct Turntable {
    speed: f32,
}

impl Turntable {
    pub(crate) fn new(speed: f32) -> Self {
        Self { speed }
    }
}

impl Animation for Turntable {
    fn update(&self, prev: &mut CameraState, dt: f32) {
        prev.update_yaw(prev.yaw() + self.speed * dt)
    }
}
