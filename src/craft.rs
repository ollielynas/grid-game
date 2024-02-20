use grid::Grid;
use macroquad::shapes::draw_rectangle;

use crate::map::Pixel;

pub fn craft(mut grid: Grid::<Pixel>) -> (bool, Grid::<bool>, Grid::<Pixel>) {

    let mut changed = false;

    let grid_clone = grid.clone();

    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            match grid[(row,col)] {
                Pixel::Lava if row > 0 && grid[(row-1,col)] == Pixel::Stone => {
                    grid[(row-1,col)] = Pixel::Lava;
                }
                Pixel::Sand if row > 0 && grid[(row-1,col)] == Pixel::Oil => {
                    grid[(row-1,col)] = Pixel::Air;
                    grid[(row,col)] = Pixel::Explosive;
                }
                Pixel::Wood if row > 0 && grid[(row-1,col)] == Pixel::Oil => {
                    grid[(row-1,col)] = Pixel::Air;
                    grid[(row,col)] = Pixel::Candle;
                }
                Pixel::Lava if row > 0 && col < grid.size().0 -1 && col > 0 && row < grid.size().0 -1 
                && grid[(row-1,col)] == Pixel::Glass
                && grid[(row+1,col)] == Pixel::Glass
                && grid[(row,col+1)] == Pixel::Glass
                && grid[(row,col-1)] == Pixel::Glass => {
                    grid[(row-1,col)] = Pixel::Air;
                    grid[(row+1,col)] = Pixel::Air;
                    grid[(row,col -1)] = Pixel::Air;
                    grid[(row,col +1)] = Pixel::Air;
                    grid[(row,col)] = Pixel::Lamp;
                }

                Pixel::LiveWood => {
                    grid[(row,col)] = Pixel::Wood;
                }
                _ => {}
            }
        }
    }
    let mut changed_grid: Grid<_> = Grid::from_vec(vec![false; grid.cols() * grid.rows()], grid.cols());
    for (a, i) in changed_grid.indexed_iter_mut() {
        *i = grid[a] != grid_clone[a]; 
        changed = *i || changed;
    }
    
    return (changed , changed_grid ,  grid);
}