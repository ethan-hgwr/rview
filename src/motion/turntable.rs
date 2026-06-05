use crate::{camera::CameraState, motion::animation::Animation};

pub(crate) struct TurntableConfig {
    pub(crate) speed: f32,
    pub(crate) pitch: f32,
    pub(crate) distance: f32,
}

pub(crate) struct Turntable {
    config: TurntableConfig,
}

impl TurntableConfig {
    pub(crate) fn new(speed: f32, pitch: f32, distance: f32) -> Self {
        Self {
            speed,
            pitch: pitch.to_radians(),
            distance,
        }
    }
}

impl Turntable {
    pub(crate) fn new(config: TurntableConfig) -> Self {
        Self { config }
    }
}

impl Animation for Turntable {
    fn update(&self, prev: &mut CameraState, dt: f32) {
        prev.update(
            prev.yaw() + self.config.speed * dt,
            self.config.pitch,
            self.config.distance,
        )
    }
}
