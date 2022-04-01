mod nerdle;
mod wordle;
use clap::{ Arg, Command};
use std::io::{Write};

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

type GuessResult = Vec<LetterResult>;

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
    target.chars()
        .zip(guess.chars())
        .enumerate()
        .map(|(_, (t, g))| {
            LetterResult {
                state: if t == g {
                    LetterState::Correct
                } else if target.contains(g) {
                    LetterState::WrongLocation
                } else {
                    LetterState::NotPresent
                },
                letter: g
            }
        })
        .collect()
}


fn prompt() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().to_string().to_lowercase()
}


fn main() {
    let app = Command::new("Rustle")
        .version("0.1")
        .author("Rustle")
        .about("Play wordle and nerdle in your terminal");

    let num_option = Arg::new("length")
        .short('l')
        .long("length")
        .help("Sets the length of the phrase you are shown")
        .takes_value(true);

    let nerdle_option = Arg::new("nerdle")
        .short('n')
        .long("nerdle")
        .help("Play nerdle")
        .takes_value(false);

    let app = app.arg(num_option).arg(nerdle_option);

    let matches = app.get_matches();

    let num = matches.value_of("length").unwrap_or("8").parse::<u8>().unwrap();
    let nerdle = matches.is_present("nerdle");

    let phrase = if nerdle {
        nerdle::play(num)
    } else {
        wordle::play(num)
    }
    .to_lowercase();

    println!("Phrase: {}", phrase);

    let mut board: Board = vec![];

    print_board(board.to_owned());

    loop {
        let word = prompt();

        if word.len() != num as usize {
            print_board(board.to_owned());
            continue;
        }

        let result = calc_dist(phrase.to_owned(), word.to_owned());

        board.push(result);

        print_board(board.to_owned());

        if word == phrase {
            println!("\nYou win!");
            break;
        }
    }
}





