use rand::Rng;

enum GridEntry {
    X,
    O,
    EMPTY
}

fn main() {
    let grid: Vec<Vec<GridEntry>> = init_grid();
    show_grid(grid);
}

fn init_grid() -> Vec<Vec<GridEntry>> {
    let mut grid: Vec<Vec<GridEntry>> = Vec::new();

    for _y in 0..3 {
        let mut r: Vec<GridEntry> = Vec::new();
        for _x in 0..3 {
            r.push(GridEntry::EMPTY);
        }
        grid.push(r);
    }

    random_move(GridEntry::X, &mut grid);
    random_move(GridEntry::O, &mut grid);
    random_move(GridEntry::X, &mut grid);
    random_move(GridEntry::O, &mut grid);
    random_move(GridEntry::X, &mut grid);

    return grid;
}

fn show_grid(grid: Vec<Vec<GridEntry>>) {

    for y in 0..3 {
        for x in 0..3 {
            match grid[x][y] {
                GridEntry::O => print!("O "),
                GridEntry::X => print!("X "),
                GridEntry::EMPTY => print!("  ")
            }
        }
        println!("");
    }

}

fn random_move(grid_move: GridEntry, grid: &mut Vec<Vec<GridEntry>>) {

    let mut rng = rand::thread_rng();

    loop {
        let rand_x = rng.gen_range(0, 3);
        let rand_y = rng.gen_range(0, 3);

        match grid[rand_x][rand_y] {
            GridEntry::EMPTY => {
                grid[rand_x][rand_y] = grid_move;
                break;
            },
            _ => {}
        }
    }

}