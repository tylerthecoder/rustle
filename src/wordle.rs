use std::io::{self, Write, BufRead, BufReader};
use std::fs;
use rand::Rng;
use rand::seq::IteratorRandom; // 0.7.3


#[derive(Copy, Clone, PartialEq, Eq)]
enum LetterState {
    NotPresent,
    WrongLocation,
    Correct,
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct LetterResult {
    state: LetterState,
    letter: char
}

type GuessResult = [LetterResult; 5];

type Board = Vec<GuessResult>;


fn print_board(board: Board) {
    let clear_scree_code = "\x1b[2J";
    let move_cursor_to_top_code = "\x1B[1;1H";
    let clear_style_code = "\x1b[0m";

    let lines = board.into_iter().map(|guess| {
        (guess.into_iter().map(|letter_state| {
            if letter_state.state == LetterState::Correct {
                format!("{}{}{}","\x1b[42m",letter_state.letter,clear_style_code)
            } else if letter_state.state == LetterState::WrongLocation {
                format!("{}{}{}","\x1b[30;43m",letter_state.letter,clear_style_code)
            } else {
                format!("{}",letter_state.letter)
            }
        })
        .fold(String::new(), |a, b| a + &b)
        )
        + "\n"
    })
    .fold(String::new(), |a, b| a + &b);

    print!("{}{}{}", clear_scree_code, move_cursor_to_top_code, lines);
    std::io::stdout().flush().unwrap();
}


fn calc_dist(target: String, guess: String) -> GuessResult {
    let mut result = [LetterResult {state: LetterState::Correct, letter: 'a'}; 5];

    for i in 0..5 {
        let target_char = target.chars().nth(i).unwrap();
        let guess_char = guess.chars().nth(i).unwrap();
        let state = if target_char == guess_char {
            LetterState::Correct
        } else if target.contains(guess_char) {
            LetterState::WrongLocation
        } else {
            LetterState::NotPresent
        };
        result[i] = LetterResult {
            letter: guess_char,
            state,
        }
    }
    result
}


fn get_word() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().to_string().to_lowercase()
}

fn get_target_word() -> String {
    let f = fs::File::open("words.txt").unwrap();
    let f = BufReader::new(f);
    let lines = f.lines().map(|l| l.expect("couldn't read line"));
    lines.choose(&mut rand::thread_rng()).expect("No words found")
}

fn main() {
    let mut board: Board = vec![];
    let target_word = get_target_word();
    print_board(board.to_owned());
    loop {
        let word = get_word();

        let result = calc_dist(target_word.to_owned(), word.to_owned());

        board.push(result);

        print_board(board.to_owned());

        if word == target_word {
            println!("\nYou win!");
            break;
        }
    }
}
