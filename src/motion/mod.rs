use crate::camera::CameraState;

pub(crate) mod turntable;

pub(crate) trait Animation {
    fn update(&self, prev: &mut CameraState, dt: f32);
}
