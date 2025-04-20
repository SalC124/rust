fn main() {
    let inputted_board: [[char; 3]; 3] = [['x', 'o', 'x'], ['o', 'x', ' '], [' ', ' ', 'o']];
    let mut game: GameState = GameState::from_board(inputted_board);
    game.see_the_future();

    println!("{:?}", game);
}

#[derive(Debug, Clone)]
struct GameState {
    board: [[char; 3]; 3],    // the board state
    depth: u8,                // depth in the tree
    wins: bool,               // is did this position result in a win?
    player: char,             // what is the current player?
    children: Vec<GameState>, // its children are a vector of more Game States
}

impl GameState {
    // create a new, empty, game state
    fn from_board(board: [[char; 3]; 3]) -> GameState {
        GameState {
            board,
            depth: 0,
            wins: false,
            player: Self::get_player(board),
            children: Vec::new(),
        }
    }
    fn get_player(board: [[char; 3]; 3]) -> char {
        let mut x_count: u8 = 0;
        let mut o_count: u8 = 0;
        for row in 0..3 {
            for col in 0..3 {
                match board[row][col] {
                    'x' => x_count += 1,
                    'o' => o_count += 1,
                    _ => {}
                }
            }
        }

        if x_count == o_count { return 'x'; } else { return 'o'; }
    }
    // add possible new next moves
    fn see_the_future(&mut self) {
        if check_win(self.board, self.player) {
            println!("this board wins:\n{:?}", self.board);
            return;
        }

        for row in 0..3 {
            for col in 0..3 {
                if self.board[row][col] == ' ' {
                    let mut new_state = self.clone();
                    new_state.board[row][col] = self.player;
                    new_state.player = if self.player == 'o' { 'x' } else { 'o' };

                    new_state.see_the_future();

                    self.children.push(new_state);
                }
            }
        }
    }
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
