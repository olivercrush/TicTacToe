fn main() {
    let grid: Vec<Vec<i32>> = init_grid();
    show_grid(grid);
}

fn init_grid() -> Vec<Vec<i32>> {
    let mut grid: Vec<Vec<i32>> = Vec::new();

    for _y in 0..3 {
        let mut r: Vec<i32> = Vec::new();
        for _x in 0..3 {
            r.push(0);
        }
        grid.push(r);
    }

    return grid;
}

fn show_grid(grid: Vec<Vec<i32>>) {

    for y in 0..3 {
        for x in 0..3 {
            print!("{}  ", grid[x][y]);
        }
        println!("");
    }

}
