use std::io;
use rand::Rng;

#[derive(PartialEq, Copy, Clone)]
enum GridEntry {
    X,
    O,
    EMPTY
}

fn main() {
    let mut grid: Vec<Vec<GridEntry>> = init_grid();

    show_grid(&grid);

    while !check_grid(&grid).0 {
        println!("---------------------------------------- X TURN ----------------------------------------------");

        let mut x_move: String = String::new();
        let mut y_move: String = String::new();
        let mut valid_move: bool = false;

        while !valid_move {
            println!("Enter X of your move : ");

            io::stdin()
                .read_line(&mut x_move)
                .expect("Failed to read X of the move");

            println!("Enter Y of your move : ");

            io::stdin()
                .read_line(&mut y_move)
                .expect("Failed to read Y of the move");

            let x_move: usize = match x_move.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let y_move: usize = match y_move.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            valid_move = make_a_move(x_move, y_move, GridEntry::X, &mut grid);
        }

        println!("");
        show_grid(&grid);

        if !check_grid(&grid).0 {
            println!("---------------------------------------- O TURN ----------------------------------------------");

            random_move(GridEntry::O, &mut grid);

            println!("");
            show_grid(&grid);
        }
    }

    match check_grid(&grid).1.expect("Should be Some winner") {
        GridEntry::X => println!("End of the game. The winner is X !"),
        GridEntry::O => println!("End of the game. The winner is O !"),
        _ => panic!()
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

    return grid;
}

fn show_grid(grid: &Vec<Vec<GridEntry>>) {

    println!("-------------");
    for y in 0..3 {
        for x in 0..3 {
            match grid[x][y] {
                GridEntry::O => print!("| O "),
                GridEntry::X => print!("| X "),
                GridEntry::EMPTY => print!("|   ")
            }

            if x == 2 {
                println!("|");
            }
        }
        println!("-------------");
    }
    println!("");

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

fn check_grid(grid: &Vec<Vec<GridEntry>>) -> (bool, Option<GridEntry>) {

    for i in 0..3 {
        if grid[i][0] != GridEntry::EMPTY && grid[i][0] == grid[i][1] && grid[i][0] == grid[i][2] { return (true, Some(grid[i][0])); }
        if grid[0][i] != GridEntry::EMPTY && grid[0][i] == grid[1][i] && grid[0][i] == grid[2][i] { return (true, Some(grid[0][i])); }
    }

    if grid[0][0] != GridEntry::EMPTY && grid[0][0] == grid[1][1] && grid[0][0] == grid[2][2] { return (true, Some(grid[0][0])); }
    if grid[0][2] != GridEntry::EMPTY && grid[0][2] == grid[1][1] && grid[0][2] == grid[2][0] { return (true, Some(grid[0][2])); }

    return (false, None);
}