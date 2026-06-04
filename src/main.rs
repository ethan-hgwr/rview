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
    camera::Camera, cli::Cli, framebuffer::Framebuffer, obj_loader::load, pipeline::Pipeline,
};

mod camera;
mod cli;
mod framebuffer;
mod model;
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

    let mut distance = args.distance;

    let mut last_mouse_pos = (0, 0);
    let mut yaw: f32 = args.yaw.to_radians();
    let mut pitch: f32 = args.pitch.to_radians();

    let mut stdout = std::io::stdout();

    let (width, height) = size().expect("Couldn't get the terminal size.");

    let fov = args.fov.to_radians();
    let aspect_ratio = width as f32 / height as f32;

    let near = args.near;
    let far = args.far;

    let path = args
        .file_path
        .to_str()
        .expect("Cannot convert path to string.");

    let objects = Box::new([load(path, Vec3::splat(1.0), Quat::IDENTITY, Vec3::ZERO)
        .with_context(|| format!("Couldn't load {}", path))?]);

    let camera = Camera::new_with(yaw, pitch, distance);
    let framebuffer = Framebuffer::new_with(BACKGROUND, width.into(), height.into(), BACKGROUND);
    let mut pipeline = Pipeline::new(fov, aspect_ratio, near, far, objects, framebuffer, camera);

    let mut prev = Instant::now();
    let timer = Instant::now();

    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
        SetTitle("rview")
    )
    .context("Couldn't execute crossterm commands.")?;

    enable_raw_mode().context("Couldn't enter crossterm raw mode.")?;
    execute!(stdout, Hide).context("Couldn't hide cursor with crossterm.")?;

    pipeline.update_radius(distance);
    pipeline.rotate_cam_x(pitch);
    pipeline.rotate_cam_y(yaw);
    pipeline.render().context("Failed to render frame.")?;

    loop {
        let now = Instant::now();

        let _dt = now.duration_since(prev).as_secs_f32();
        let _t = timer.elapsed().as_secs_f32();

        let mut dirty = false;

        if poll(Duration::from_secs_f32(REFRESH_RATE))? {
            match read().context("Failed to read event with crossterm.")? {
                Event::Key(key_event) => {
                    if key_event == KeyCode::Char(EXIT_KEY).into() {
                        break;
                    }
                }
                Event::Mouse(mouse_event) => match mouse_event.kind {
                    MouseEventKind::ScrollDown => {
                        distance = (distance + CAM_DISTANCE_STEP).min(MAX_CAM_DISTANCE);
                        dirty = true;
                    }
                    MouseEventKind::ScrollUp => {
                        distance = (distance - CAM_DISTANCE_STEP).max(MIN_CAM_DISTANCE);
                        dirty = true;
                    }
                    MouseEventKind::Down(MouseButton::Left) => {
                        last_mouse_pos = (mouse_event.column as i32, mouse_event.row as i32);
                        dirty = true;
                    }
                    MouseEventKind::Drag(MouseButton::Left) => {
                        let (new_x, new_y) = (mouse_event.column as i32, mouse_event.row as i32);
                        let (old_x, old_y) = last_mouse_pos;
                        let dx = new_x - old_x;
                        let dy = new_y - old_y;

                        yaw += dx as f32 * YAW_SENSITIVITY;

                        if (pitch - dy as f32 * PITCH_SENSITIVITY) < FRAC_PI_2 {
                            pitch -= dy as f32 * PITCH_SENSITIVITY;
                        }

                        last_mouse_pos = (new_x, new_y);
                        dirty = true;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        if dirty {
            pipeline.update_radius(distance);
            pipeline.rotate_cam_x(pitch);
            pipeline.rotate_cam_y(yaw);

            pipeline.render().context("Failed to render frame.")?;
        }

        prev = now;
    }

    execute!(stdout, Show).context("Couldn't show cursor with crossterm.")?;
    disable_raw_mode().context("Couldn't exit crossterm raw mode.")?;

    execute!(std::io::stdout(), LeaveAlternateScreen, DisableMouseCapture)
        .context("Couldn't execute crossterm commands.")?;

    Ok(())
}
