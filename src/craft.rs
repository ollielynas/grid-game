use grid::Grid;

use crate::map::Pixel;

pub fn craft(grid: Grid::<Pixel>) -> (bool, Grid::<Pixel>) {


    let mut g_vec = grid.clone().into_vec();
    let length = g_vec.len();
    g_vec.retain(|x| *x!=Pixel::Air);
    let mut changed = true;
    let mut new_vec = match g_vec.as_slice() {
        [Pixel::Stone, Pixel::Lava] | [Pixel::Lava, Pixel::Stone] => {
            vec![Pixel::Lava, Pixel::Lava]
        }
        a => {
            changed = false;
            a.to_vec()
        }
    };
    let len2 = new_vec.len();

    new_vec.append(&mut vec![Pixel::Air;length-len2]);
    
    return (changed, if changed {Grid::from_vec(new_vec, grid.cols())} else {grid});
}