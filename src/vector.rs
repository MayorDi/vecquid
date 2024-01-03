use nalgebra::Vector2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use crate::grid::{get_neighbors, get_pos_idx, GetVectors, SIZE_GRID};
use crate::render::Render;
use crate::update::Update;

pub trait GetVector: Clone + Copy {
    type Output: GetVector;
    fn new(x: f32, y: f32) -> Self::Output;
    fn get_position(&self) -> Vector2<f32>;
    fn get_vector(&self) -> Vector2<f32>;
    fn get_mut_vector(&mut self) -> &mut Vector2<f32>;
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Vector {
    inner: Vector2<f32>,
    pub position: Vector2<f32>,
}

impl Into<Vector> for (usize, usize) {
    fn into(self) -> Vector {
        Vector::new(self.0 as f32, self.1 as f32)
    }
}

impl GetVector for Vector {
    type Output = Vector;

    fn new(x: f32, y: f32) -> Self {
        Self {
            inner: Vector2::new(x, y),
            position: Vector2::new(0., 0.)
        }
    }

    fn get_position(&self) -> Vector2<f32> {
        self.position
    }

    fn get_vector(&self) -> Vector2<f32> {
        self.inner
    }

    fn get_mut_vector(&mut self) -> &mut Vector2<f32> {
        &mut self.inner
    }
}

impl Render for Vector {
    fn render(&self, idx: usize, canvas: &mut WindowCanvas) {
        let size = 5;
        let (x, y) = get_pos_idx(idx);
        let rect = Rect::new(x as i32 * size, y as i32 * size, size as u32, size as u32);
        let k = self.inner.norm_squared();

        let (r, g, b) = get_color(370.0 + k*2.);

        canvas.set_draw_color(
            Color::RGB(
                (r * 100.0) as u8,
                (g * 100.0) as u8,
                (b * 100.0) as u8,
            )
        );
        let _ = canvas.fill_rect(rect);
    }
}

impl Update for Vector {
    fn update<G>(idx: usize, grid_read: &G, grid: &mut G) where G: GetVectors {
        let this = grid_read.get_vectors()[idx].get_vector();
        let len = this.norm_squared();
        if len == 0.0 { return; }

        let neighbors = get_neighbors(idx);
        neighbors.iter()
            .for_each(|n| {
                let len_n = grid.get_vectors()[*n].get_vector().norm_squared();
                if len_n > 100.0 { return; }

                *grid.get_mut_vectors()[*n].get_mut_vector() +=
                    grid_read.get_vectors()[idx].get_vector() * 0.18;
                *grid.get_mut_vectors()[idx].get_mut_vector() -=
                    grid_read.get_vectors()[idx].get_vector() * 0.18;
            });
    }
}

fn get_color(w: f32) -> (f32, f32, f32) {
    let (red, green, blue) = match w {
        380.0..=490.0 => (0.0, (w - 440.0) / 50.0, (w - 440.) / 30.0),
        380.0..=510.0 => (0.0, 1.0, (510. - w) / 20.0),
        380.0..=580.0 => ((w - 510.0) / 70., 1.0, 0.0),
        380.0..=645.0 => (1.0, (645.0 - w) / 65.0, 0.0),
        645.0..=781.0 => (1.0, 0.0, 0.0),
        _ => (0.0, 0.0, 0.0)
    };

    let factor = match w {
        380.0..=420.0 => 0.3 + 0.7 * w - 380.0 / 40.0,
        380.0..=701.0 => 1.0,
        701.0..=781.0 => 0.3 + 0.7 * (780.0 - w) / 80.0,
        _ => 0.0
    };

    let gamma = 0.8;
    let r = (red*factor).powf(gamma);
    let g = (green*factor).powf(gamma);
    let b = (blue*factor).powf(gamma);

    (r, g, b)
}