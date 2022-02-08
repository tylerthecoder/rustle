
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
    // let clear_scree_code = "\x1b[2J";
    let clear_style_code = "\x1b[0m";

    let lines = board.into_iter().map(|guess| {
        guess.into_iter().map(|letter_state| {
            if letter_state.state == LetterState::Correct {
                format!("{}{}{}","\x1b[42m",letter_state.letter,clear_style_code)
            } else if letter_state.state == LetterState::WrongLocation {
                format!("{}{}{}","\x1b[30;43m",letter_state.letter,clear_style_code)
            } else {
                format!("{}",letter_state.letter)
            }
        })
        .fold(String::new(), |a, b| a + &b)
        + "\n"
    })
    .fold(String::new(), |a, b| a + &b);

    print!("{}[2J", 27 as char);
    print!("{}", lines);
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

fn main() {
    let mut board: Board = vec![];
    let target_word = "print";
    print!("{}[2J", 27 as char);
    print_board(board.to_owned());
    loop {
        let word = get_word();

        let result = calc_dist(String::from(target_word), word.to_owned());

        board.push(result);

        print_board(board.to_owned());

        if word == target_word {
            println!("\nYou win!");
            break;
        }
    }
}
