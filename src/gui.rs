use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};

use crate::{camera::CameraState, constants::*, mode::Mode, motion::Animation, pipeline::Pipeline};

const WIDTH: usize = 1024;
const HEIGHT: usize = 768;
const ROTATION_SPEED: f32 = 1.8;
const MOUSE_SENSITIVITY: f32 = 0.01;

pub(crate) fn run(
    pipeline: &mut Pipeline<u32>,
    camera_state: &mut CameraState,
    mode: &Mode,
) -> Result<()> {
    let mut window = Window::new(
        "rview | drag to rotate, scroll to zoom, Esc to close",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .context("Couldn't create the model viewer window.")?;

    pipeline.apply_camera_state(camera_state);
    pipeline.render_pixels()?;

    let mut previous_mouse = None;
    let mut previous_frame = Instant::now();
    let start = previous_frame;

    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
        let now = Instant::now();
        let dt = now.duration_since(previous_frame).as_secs_f32();
        previous_frame = now;
        let mut dirty = false;

        match mode {
            Mode::Interactive => {
                if window.is_key_down(Key::Left) {
                    camera_state.update_yaw(camera_state.yaw() - ROTATION_SPEED * dt);
                    dirty = true;
                }
                if window.is_key_down(Key::Right) {
                    camera_state.update_yaw(camera_state.yaw() + ROTATION_SPEED * dt);
                    dirty = true;
                }

                let pitch_direction =
                    match (window.is_key_down(Key::Up), window.is_key_down(Key::Down)) {
                        (true, false) => 1.0,
                        (false, true) => -1.0,
                        _ => 0.0,
                    };
                if pitch_direction != 0.0 {
                    let pitch = camera_state.pitch() + pitch_direction * ROTATION_SPEED * dt;
                    if -std::f32::consts::FRAC_PI_2 < pitch && pitch < std::f32::consts::FRAC_PI_2 {
                        camera_state.update_pitch(pitch);
                        dirty = true;
                    }
                }

                if let Some((_, scroll_y)) = window.get_scroll_wheel() {
                    if scroll_y != 0.0 {
                        camera_state.update_radius(
                            (camera_state.radius() - scroll_y * CAM_RADIUS_STEP)
                                .clamp(MIN_CAM_RADIUS, MAX_CAM_RADIUS),
                        );
                        dirty = true;
                    }
                }

                let mouse_position = window.get_mouse_pos(MouseMode::Pass);
                if window.get_mouse_down(MouseButton::Left) {
                    if let (Some((old_x, old_y)), Some((new_x, new_y))) =
                        (previous_mouse, mouse_position)
                    {
                        let dx = new_x - old_x;
                        let dy = new_y - old_y;
                        camera_state.update_yaw(camera_state.yaw() + dx * MOUSE_SENSITIVITY);
                        let pitch = camera_state.pitch() + dy * MOUSE_SENSITIVITY;
                        if -std::f32::consts::FRAC_PI_2 < pitch
                            && pitch < std::f32::consts::FRAC_PI_2
                        {
                            camera_state.update_pitch(pitch);
                        }
                        dirty |= dx != 0.0 || dy != 0.0;
                    }
                    previous_mouse = mouse_position;
                } else {
                    previous_mouse = None;
                }
            }
            Mode::Animation(animation) => {
                animation.update(camera_state, dt);
                dirty = true;
            }
            Mode::Benchmark(benchmark) => {
                if benchmark.is_timedout(start.elapsed().as_secs_f32()) {
                    break;
                }
                benchmark.update(camera_state, dt);
                dirty = true;
            }
        }

        if dirty {
            pipeline.apply_camera_state(camera_state);
            pipeline.render_pixels()?;
        }

        window
            .update_with_buffer(pipeline.pixels(), WIDTH, HEIGHT)
            .context("Couldn't update the model viewer window.")?;

        let frame_time = Duration::from_secs_f32(1.0 / 60.0);
        let elapsed = now.elapsed();
        if elapsed < frame_time {
            std::thread::sleep(frame_time - elapsed);
        }
    }

    Ok(())
}
