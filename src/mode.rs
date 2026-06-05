use crate::{
    benchmark::Benchmark,
    motion::{animation::Animation, turntable::Turntable},
};

pub(crate) enum Mode {
    Interactive,
    Animation(Box<dyn Animation>),
    Benchmark(Benchmark<Turntable>),
}
