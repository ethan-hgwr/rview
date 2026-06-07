use glam::{Mat4, Vec3, Vec3A};

const UP: Vec3 = Vec3::Y;
const TARGET: Vec3 = Vec3::ZERO;

pub(crate) struct Camera {
    state: CameraState,
    view_matrix: Mat4,
}

pub(crate) struct CameraState {
    yaw: f32,
    pitch: f32,
    radius: f32,
}

impl Camera {
    pub fn new_with(yaw: f32, pitch: f32, radius: f32) -> Self {
        Self {
            state: CameraState::new(yaw, pitch, radius),
            view_matrix: Mat4::look_at_rh(Vec3::new(0.0, 0.0, -radius), TARGET, UP),
        }
    }

    #[inline(always)]
    pub fn get_view_matrix(&self) -> &Mat4 {
        &self.view_matrix
    }

    fn update_view_matrix(&mut self) {
        let eye = self.get_position();
        let target = Vec3::ZERO;

        self.view_matrix = Mat4::look_at_rh(eye.into(), target, UP);
    }

    pub fn get_position(&self) -> Vec3A {
        let x = self.state.radius * self.state.pitch.cos() * self.state.yaw.cos();
        let y = self.state.radius * self.state.pitch.sin();
        let z = self.state.radius * self.state.pitch.cos() * self.state.yaw.sin();
        Vec3A::new(x, y, z)
    }

    pub fn update_radius(&mut self, radius: f32) {
        self.state.update_radius(radius);
        self.update_view_matrix();
    }

    pub fn rotate_x(&mut self, pitch: f32) {
        self.state.update_pitch(pitch);
        self.update_view_matrix();
    }

    pub fn rotate_y(&mut self, yaw: f32) {
        self.state.update_yaw(yaw);
        self.update_view_matrix();
    }
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            yaw: CameraState::DEFAULT_YAW,
            pitch: CameraState::DEFAULT_PITCH,
            radius: CameraState::DEFAULT_RADIUS,
        }
    }
}

impl CameraState {
    pub fn new(yaw: f32, pitch: f32, radius: f32) -> Self {
        Self { yaw, pitch, radius }
    }

    pub fn _update(&mut self, yaw: f32, pitch: f32, radius: f32) {
        self.update_yaw(yaw);
        self.update_pitch(pitch);
        self.update_radius(radius);
    }

    pub fn update_yaw(&mut self, yaw: f32) {
        self.yaw = yaw;
    }

    pub fn update_pitch(&mut self, pitch: f32) {
        self.pitch = pitch;
    }

    pub fn update_radius(&mut self, radius: f32) {
        self.radius = radius;
    }

    pub fn yaw(&self) -> f32 {
        self.yaw
    }

    pub fn pitch(&self) -> f32 {
        self.pitch
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}
