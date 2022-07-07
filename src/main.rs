use std::{io, collections::HashMap};

fn main() {
    let (mut x, mut y): (usize, &str);
    let dict_y: HashMap<&str, usize> = HashMap::from([
        ("a", 1),
        ("b", 2),
        ("c", 3)
    ]);
    let mut counter: usize;
    let mut xo: [[char; 3]; 3];
    loop {
        counter = 0;
        xo = [[' ', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' ', ]];
        draw(&xo);
        loop {
            let mut input: String = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Input failed");

            // print!("{} {}", &input[2..], &input[..1]);
            (x, y) = (input[..1].parse::<usize>().unwrap(), &input[1..2]);
            let current = ['X', 'O'][counter % 2];
            xo[x-1][dict_y[y]-1] = current;

            draw(&xo);
            counter += 1;
            if (counter == 9) | check(&xo, x-1, dict_y[y]-1) {
                println!("{} won!", current);
                break
            }
        }
        break
    }
}

fn check(xo: &[[char; 3]; 3], x: usize, y: usize) -> bool {
    let diagonal = if x == y {
        xo[0][0] == xo[1][1] && xo[1][1] == xo[2][2]
    }
    else if x + y == 2 {
        xo[0][2] == xo[1][1] && xo[1][1] == xo[2][0]
    }
    else {
        false
    };
    ((xo[0][y] == xo[1][y] && xo[1][y] == xo[2][y]) | (xo[x][0] == xo[x][1] && xo[x][1] == xo[x][2])) | diagonal
}

fn draw(xo: &[[char; 3]; 3]) {
            println!("   |   |   ");
            println!(" {} | {} | {} ", xo[0][0], xo[0][1], xo[0][2]);
            println!("---|---|---");
            println!(" {} | {} | {} ", xo[1][0], xo[1][1], xo[1][2]);
            println!("---|---|---");
            println!(" {} | {} | {} ", xo[2][0], xo[2][1], xo[2][2]);
            println!("   |   |   ");
}
