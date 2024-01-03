use nalgebra::Vector2;
use crate::vector::{GetVector, Vector};

pub const SIZE_GRID: [usize; 2] = [100, 100];

pub trait GetVectors: Clone {
    type Output: GetVector;
    fn get_vectors(&self) -> &[Self::Output];
    fn get_mut_vectors(&mut self) -> &mut [Self::Output];
}

#[derive(Debug, Clone)]
pub struct Grid<V: GetVector> {
    inner: Vec<V>
}

impl GetVectors for Grid<Vector> {
    type Output = Vector;

    fn get_vectors(&self) -> &[Self::Output] {
        &self.inner[..]
    }

    fn get_mut_vectors(&mut self) -> &mut [Self::Output] {
        &mut self.inner[..]
    }
}

pub const fn get_pos_idx(idx: usize) -> (usize, usize) {
    (idx % SIZE_GRID[0], idx / SIZE_GRID[0])
}

pub const fn get_idx_pos(x: usize, y: usize) -> usize {
    y * SIZE_GRID[0] + x
}

const fn len_grid() -> usize {
    SIZE_GRID[0] * SIZE_GRID[1]
}

impl Grid<Vector> {
    pub fn new() -> Self {
        let mut grid = Self {
            inner: vec![Vector::default(); len_grid()],
        };

        for (idx, vector) in grid.inner.iter_mut().enumerate() {
            let (x, y) = get_pos_idx(idx);
            vector.position = Vector2::new(x as f32, y as f32);
        }

        grid
    }

    pub fn init(&mut self) {
        *self.get_mut_vectors()[get_idx_pos(50, 50)].get_mut_vector() += *Vector::new(0.1, 0.1).get_mut_vector();
    }
}

pub fn get_neighbors(idx: usize) -> [usize; 8] {
    let pos = get_pos_idx(idx);
    let pos = (pos.0 as i32, pos.1 as i32);

    [
        get_idx_pos(limit(pos.0 - 1, 0, SIZE_GRID[0] as i32 - 1), limit(pos.1, 0, SIZE_GRID[1] as i32 - 1)),
        get_idx_pos(limit(pos.0 + 1, 0, SIZE_GRID[0] as i32 - 1), limit(pos.1, 0, SIZE_GRID[1] as i32 - 1)),
        get_idx_pos(limit(pos.0, 0, SIZE_GRID[0] as i32 - 1), limit(pos.1 + 1, 0, SIZE_GRID[1] as i32 - 1)),
        get_idx_pos(limit(pos.0, 0, SIZE_GRID[0] as i32 - 1), limit(pos.1 - 1, 0, SIZE_GRID[1] as i32 - 1)),
        get_idx_pos(limit(pos.0 - 1, 0, SIZE_GRID[0] as i32 - 1), limit(pos.1 - 1, 0, SIZE_GRID[1] as i32 - 1)),
        get_idx_pos(limit(pos.0 + 1, 0, SIZE_GRID[0] as i32 - 1), limit(pos.1 - 1, 0, SIZE_GRID[1] as i32 - 1)),
        get_idx_pos(limit(pos.0 - 1, 0, SIZE_GRID[0] as i32 - 1), limit(pos.1 + 1, 0, SIZE_GRID[1] as i32 - 1)),
        get_idx_pos(limit(pos.0 + 1, 0, SIZE_GRID[0] as i32 - 1), limit(pos.1 + 1, 0, SIZE_GRID[1] as i32 - 1)),
    ]
}

fn limit(n: i32, min: i32, max: i32) -> usize {
    if n < min {
        min as usize
    } else if n > max {
        max as usize
    } else {
        n as usize
    }
}