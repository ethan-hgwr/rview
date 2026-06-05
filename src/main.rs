use anyhow::{Context, Result};
use clap::Parser;
use crossterm::{
    cursor::{Hide, Show},
    event::{
        DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseButton, MouseEventKind, poll,
        read,
    },
    execute,
    terminal::{
        EnterAlternateScreen, LeaveAlternateScreen, SetTitle, disable_raw_mode, enable_raw_mode,
        size,
    },
};
use glam::{Quat, Vec3};
use std::{
    f32::consts::FRAC_PI_2,
    time::{Duration, Instant},
};

use crate::{
    camera::{Camera, CameraState},
    cli::{Cli, build_mode},
    framebuffer::Framebuffer,
    mode::Mode,
    motion::animation::Animation,
    obj_loader::load,
    pipeline::Pipeline,
};

mod benchmark;
mod camera;
mod cli;
mod framebuffer;
mod mode;
mod model;
mod motion;
mod obj_loader;
mod pipeline;
mod raster;
mod types;

const TARGET_FPS: u32 = 165;
const REFRESH_RATE: f32 = 1.0 / TARGET_FPS as f32;
const BACKGROUND: char = ' ';

const EXIT_KEY: char = 'q';

const MAX_CAM_DISTANCE: f32 = 10.0;
const MIN_CAM_DISTANCE: f32 = 1.0;
const CAM_DISTANCE_STEP: f32 = 0.1;

const YAW_SENSITIVITY: f32 = 0.05;
const PITCH_SENSITIVITY: f32 = -YAW_SENSITIVITY;

fn main() -> Result<()> {
    let args = Cli::parse();
    let mode = build_mode(&args).context("Couldn't build execution mode.")?;

    let mut stdout = std::io::stdout();
    let (width, height) = size().context("Couldn't get the terminal size.")?;

    let path = args
        .file_path
        .to_str()
        .context("Cannot convert path to string.")?;
    let objects = Box::new([load(path, Vec3::splat(1.0), Quat::IDENTITY, Vec3::ZERO)
        .with_context(|| format!("Couldn't load {}", path))?]);

    let mut camera_state = CameraState::new(
        args.yaw.to_radians(),
        args.pitch.to_radians(),
        args.distance,
    );

    let mut pipeline = Pipeline::new(
        args.fov.to_radians(),
        width as f32 / height as f32,
        args.near,
        args.far,
        objects,
        Framebuffer::new_with(BACKGROUND, width.into(), height.into(), BACKGROUND),
        Camera::new_with(
            camera_state.yaw(),
            camera_state.pitch(),
            camera_state.radius(),
        ),
    );

    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
        SetTitle("rview"),
        Hide
    )
    .context("Couldn't setup terminal.")?;
    enable_raw_mode().context("Couldn't enter raw mode.")?;

    pipeline.apply_camera_state(&camera_state);
    pipeline
        .render()
        .context("Failed to render initial frame.")?;

    let result = run(&mut pipeline, &mut camera_state, &mode);

    let _ = execute!(stdout, Show, LeaveAlternateScreen, DisableMouseCapture);
    let _ = disable_raw_mode();

    result
}

fn run(pipeline: &mut Pipeline<char>, camera_state: &mut CameraState, mode: &Mode) -> Result<()> {
    let mut last_mouse_pos = (0i32, 0i32);
    let mut prev = Instant::now();
    let start = Instant::now();

    loop {
        let now = Instant::now();
        let dt = now.duration_since(prev).as_secs_f32().min(REFRESH_RATE);

        let mut dirty = false;

        match mode {
            Mode::Interactive => {
                if poll(Duration::ZERO)? {
                    match read().context("Failed to read event.")? {
                        Event::Key(key) if key == KeyCode::Char(EXIT_KEY).into() => break,
                        Event::Mouse(mouse) => match mouse.kind {
                            MouseEventKind::ScrollDown => {
                                camera_state.update_radius(
                                    (camera_state.radius() + CAM_DISTANCE_STEP)
                                        .min(MAX_CAM_DISTANCE),
                                );
                                dirty = true;
                            }
                            MouseEventKind::ScrollUp => {
                                camera_state.update_radius(
                                    (camera_state.radius() - CAM_DISTANCE_STEP)
                                        .max(MIN_CAM_DISTANCE),
                                );
                                dirty = true;
                            }
                            MouseEventKind::Down(MouseButton::Left) => {
                                last_mouse_pos = (mouse.column as i32, mouse.row as i32);
                            }
                            MouseEventKind::Drag(MouseButton::Left) => {
                                let (new_x, new_y) = (mouse.column as i32, mouse.row as i32);
                                let (dx, dy) = (new_x - last_mouse_pos.0, new_y - last_mouse_pos.1);

                                camera_state
                                    .update_yaw(camera_state.yaw() + dx as f32 * YAW_SENSITIVITY);

                                let new_pitch =
                                    camera_state.pitch() - dy as f32 * PITCH_SENSITIVITY;
                                if new_pitch < FRAC_PI_2 {
                                    camera_state.update_pitch(new_pitch);
                                }

                                last_mouse_pos = (new_x, new_y);
                                dirty = true;
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
            Mode::Animation(anim) => {
                if poll(Duration::ZERO)? {
                    if let Event::Key(key) = read().context("Failed to read event.")? {
                        if key == KeyCode::Char(EXIT_KEY).into() {
                            break;
                        }
                    }
                }

                anim.update(camera_state, dt);
                dirty = true;
            }
            Mode::Benchmark(bench) => {
                if bench.is_timedout(start.elapsed().as_secs_f32()) {
                    break;
                }

                bench.update(camera_state, dt);
                dirty = true;
            }
        }

        if dirty {
            pipeline.apply_camera_state(camera_state);
            pipeline.render().context("Failed to render frame.")?;
        }

        let frame_time = Duration::from_secs_f32(REFRESH_RATE);
        let elapsed = now.elapsed();
        if elapsed < frame_time {
            std::thread::sleep(frame_time - elapsed);
        }

        prev = now;
    }

    Ok(())
}
