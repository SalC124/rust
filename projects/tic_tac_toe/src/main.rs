/*
  1 |  2 |  3
----+----+----
  4 |  5 |  6
----+----+----
  7 |  8 |  9

all solutions can be derived from transforming the following sequences:

edges:
->  111000000
    001001001
    000000111
    100100100
center:
->  000111000
    010010010
diagonal:
->  100010001
    001010100


how to rotate:
    wait i can just make it like this and just +2 % 8:
          1 |  2 |  3
        ----+----+----
          8 |  0 |  4
        ----+----+----
          7 |  6 |  5

        sequences would be: (123456789)
            edges:
                11100000_0
                00111000_0
                00001110_0
                10000011_0
            center:
                01000100_1


░▒▓███████▓▒░ ░▒▓██████▓▒░
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░
░▒▓█▓▒░░▒▓█▓▒░░▒▓██████▓▒░


why not just a 2d array?
and then just transpose and mirror for 90 degree turns!

123      147      741
456  ->  258  ->  852
789      369      963

use first solution from first brainstorming sesh

*/
use std::io;

fn main() {
    let mut game_board: [[char; 3]; 3] = [[' '; 3]; 3];
    let mut player: char = 'x';
    println!("tic-tac-toe time!");
    'gameplay_loop: loop {
        println!("\nit is {player}'s turn now");
        println!("\nthe board looks like this:");
        display(game_board);

        'pos_check: loop {
            println!("choose a place");
            println!("row:");
            let mut inputted_row: String = String::new();
            io::stdin()
                .read_line(&mut inputted_row)
                .expect("Failed to read line");
            let inputted_row: usize = match inputted_row.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("you're a stupid idiot, huh");
                    break 'pos_check;
                }
            };

            println!("col: ");
            let mut inputted_col: String = String::new();
            io::stdin()
                .read_line(&mut inputted_col)
                .expect("Failed to read line");
            let inputted_col: usize = match inputted_col.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("you're a stupid idiot, huh");
                    break 'pos_check;
                }
            };
            if !(1..=3).contains(&inputted_row) || !(1..=3).contains(&inputted_col) {
                println!("bro cant count XD");
            } else if game_board[inputted_row - 1][inputted_col - 1] != ' ' {
                println!("invalid, stupid");
                continue;
            } else {
                game_board[inputted_row - 1][inputted_col - 1] = player;
                break 'pos_check;
            }
        }

        if check_win(game_board, player) {
            println!("\n{player} won!");
            break 'gameplay_loop;
        } else {
            player = match player {
                'x' => 'o',
                'o' => 'x',
                _ => ' '
            };
        }
    }
}

fn display(board: [[char; 3]; 3]) {
    println!("  {} |  {} |  {}", board[0][0], board[0][1], board[0][2]);
    println!("----+----+----");
    println!("  {} |  {} |  {}", board[1][0], board[1][1], board[1][2]);
    println!("----+----+----");
    println!("  {} |  {} |  {}", board[2][0], board[2][1], board[2][2]);
}

fn rohtait_cockwise(board: [[char; 3]; 3], mut rot_times: u8) -> [[char; 3]; 3] {
    let mut rotted = [[' '; 3]; 3];

    rot_times %= 4;

    if rot_times == 0 {
        return board;
    }

    if rot_times == 3 {
        // tuah-huhndrehd sehvehndee dehgreez
        for _ in 1..=rot_times {
            for row in 0..=2 {
                for col in 0..=2 {
                    rotted[2 - row][col] = board[col][row];
                }
            }
        }
    } else if rot_times == 2 {
        // wuhn-huhndrehd ate-e dehgreez
        for _ in 1..=rot_times {
            for row in 0..=2 {
                for col in 0..=2 {
                    rotted[2 - row][2 - col] = board[row][col];
                }
            }
        }
    } else if rot_times == 1 {
        // neintie dehgreez
        for _ in 1..=rot_times {
            for row in 0..=2 {
                for col in 0..=2 {
                    rotted[row][2 - col] = board[col][row];
                }
            }
        }
    } else {
        println!("just one question... what the freak happened?")
    }

    rotted
}

fn check_win(board: [[char; 3]; 3], player: char) -> bool {
    // check edges
    for turns in 0..4 {
        let transformed_board = rohtait_cockwise(board, turns);
        if (player == transformed_board[0][0]
            && player == transformed_board[0][1]
            && player == transformed_board[0][2])
            || (player == transformed_board[0][0]
                && player == transformed_board[1][1]
                && player == transformed_board[2][2])
            || (player == transformed_board[1][0]
                && player == transformed_board[1][1]
                && player == transformed_board[1][2])
        {
            return true;
        }
    }
    false
}
