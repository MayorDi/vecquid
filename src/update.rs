use crate::grid::GetVectors;
use crate::vector::GetVector;

pub trait Update {
    fn update<G>(idx: usize, grid_read: &G, grid: &mut G) where G: GetVectors;
}