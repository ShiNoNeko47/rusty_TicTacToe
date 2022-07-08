use std::io;

fn main() {

    let (mut x, mut y): (usize, char);
    let mut counter: usize;
    let mut field: [[char; 3]; 3];

    loop {
        counter = 0;
        field = [[' ', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' ', ]];
        draw(&field);
        loop {
            let mut input: String = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Input failed");

            // print!("{} {}", &input[2..], &input[..1]);
            (x, y) = (input[..1].parse::<usize>().unwrap(), input.chars().nth(1).unwrap());
            let y:usize = y as usize - 96;
            let current = ['X', 'O'][counter % 2];
            field[x-1][y-1] = current;

            draw(&field);
            counter += 1;
            if counter == 9 {

                if check(&field, x-1, y-1) {
                    println!("{} won!", current);
                    break
                }
                println!("Draw!");
                break
            }
        }
        break
    }
}

fn check(field: &[[char; 3]; 3], x: usize, y: usize) -> bool {
    let diagonal = if x == y {
        field[0][0] == field[1][1] && field[1][1] == field[2][2]
    }
    else if x + y == 2 {
        field[0][2] == field[1][1] && field[1][1] == field[2][0]
    }
    else {
        false
    };
    ((field[0][y] == field[1][y] && field[1][y] == field[2][y]) | (field[x][0] == field[x][1] && field[x][1] == field[x][2])) | diagonal
}

fn draw(field: &[[char; 3]; 3]) {
            println!("   a   b   c ");
            println!("     |   |   ");
            println!("1  {} | {} | {} ", field[0][0], field[0][1], field[0][2]);
            println!("  ---|---|---");
            println!("2  {} | {} | {} ", field[1][0], field[1][1], field[1][2]);
            println!("  ---|---|---");
            println!("3  {} | {} | {} ", field[2][0], field[2][1], field[2][2]);
            println!("     |   |   ");
}
