use rand::Rng;

#[derive(PartialEq)]
enum GridEntry {
    X,
    O,
    EMPTY
}

fn main() {
    let grid: Vec<Vec<GridEntry>> = init_grid();

    show_grid(&grid);

    while !check_grid(&grid) {
        println!("---------------------------------------- X TURN ----------------------------------------------");
        println!("");

        // ask user to make move

        show_grid(&grid);


        println!("---------------------------------------- O TURN ----------------------------------------------");
        println!("");

        // make random move (implement MinMax later)

        show_grid(&grid);
    }
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

    make_a_move(0, 0, GridEntry::X, &mut grid);
    make_a_move(1, 1, GridEntry::X, &mut grid);
    make_a_move(2, 2, GridEntry::X, &mut grid);

    return grid;
}

fn show_grid(grid: &Vec<Vec<GridEntry>>) {

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

fn make_a_move(x: usize, y: usize, grid_move: GridEntry, grid: &mut Vec<Vec<GridEntry>>) -> bool {

    if x > 2 { return false; }
    if y > 2 { return false; }

    if grid[x][y] != GridEntry::EMPTY { return false; }

    grid[x][y] = grid_move;
    return true;
}

fn check_grid(grid: &Vec<Vec<GridEntry>>) -> bool {

    for i in 0..3 {
        if grid[i][0] != GridEntry::EMPTY && grid[i][0] == grid[i][1] && grid[i][0] == grid[i][2] { return true; }
        if grid[0][i] != GridEntry::EMPTY && grid[0][i] == grid[1][i] && grid[0][i] == grid[2][i] { return true; }
    }

    if grid[0][0] != GridEntry::EMPTY && grid[0][0] == grid[1][1] && grid[0][0] == grid[2][2] { return true; }
    if grid[0][2] != GridEntry::EMPTY && grid[0][2] == grid[1][1] && grid[0][2] == grid[2][0] { return true; }

    return false;
}