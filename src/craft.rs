use grid::Grid;

use crate::map::Pixel;

pub fn craft(mut grid: Grid::<Pixel>) -> (bool, Grid::<Pixel>) {

    let mut changed = false;

    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            match grid[(row,col)] {
                Pixel::Lava if row > 0 && grid[(row-1,col)] == Pixel::Stone => {
                    grid[(row-1,col)] = Pixel::Lava;
                    changed = true;
                }
                Pixel::Sand if row > 0 && grid[(row-1,col)] == Pixel::Oil => {
                    grid[(row-1,col)] = Pixel::Air;
                    grid[(row,col)] = Pixel::Explosive;
                    changed = true;
                }
                Pixel::Wood if row > 0 && grid[(row-1,col)] == Pixel::Oil => {
                    grid[(row-1,col)] = Pixel::Air;
                    grid[(row,col)] = Pixel::Candle;
                    changed = true;
                }
                Pixel::LiveWood => {
                    grid[(row,col)] = Pixel::Wood;
                    changed = true;
                }
                _ => {}
            }
        }
    }
    
    return (changed , grid);
}