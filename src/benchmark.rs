use crate::{
    camera::CameraState,
    motion::{Animation, turntable::Turntable},
};

pub(crate) struct Benchmark<A>
where
    A: Animation,
{
    animation: A,
    timeout: f32,
}

impl Benchmark<Turntable> {
    pub(crate) fn new() -> Self {
        Self {
            animation: Turntable::new(Benchmark::SPEED),
            timeout: Benchmark::TIMEOUT,
        }
    }

    pub(crate) fn is_timedout(&self, elapsed: f32) -> bool {
        self.timeout <= elapsed
    }
}

impl<A> Animation for Benchmark<A>
where
    A: Animation,
{
    fn update(&self, prev: &mut CameraState, dt: f32) {
        self.animation.update(prev, dt);
    }
}
