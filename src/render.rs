use sdl2::render::WindowCanvas;
use crate::vector::GetVector;

pub trait Render {
    fn render(&self, idx: usize, canvas: &mut WindowCanvas) where Self: GetVector;
}