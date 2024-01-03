use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use crate::grid::{GetVectors, Grid};
use crate::render::Render;
use crate::update::Update;
use crate::vector::{GetVector, Vector};

pub struct App;

impl App {
    pub fn run() -> Result<(), String> {
        const FPS: Duration = Duration::new(0, 1_000_000_000u32 / 30);

        let sdl_context = sdl2::init()?;
        log::info!("SDL2 init");

        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("rust-sdl2 demo: Video", 1024, 512)
            .resizable()
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;
        log::info!("Window init");

        let mut grid = Grid::new();
        grid.init();

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let mut event_pump = sdl_context.event_pump()?;

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                        grid = Grid::new();
                        grid.init();
                    }
                    _ => {}
                }
            }

            canvas.set_draw_color(Color::RGB(0x3c, 0x38, 0x37));
            canvas.clear();

            let grid_read = grid.clone();
            for (idx, vector) in grid_read.get_vectors().iter().enumerate() {
                vector.render(idx, &mut canvas);
                Vector::update(idx, &grid_read, &mut grid);
            }

            canvas.present();
            std::thread::sleep(FPS);
        }

        Ok(())
    }
}