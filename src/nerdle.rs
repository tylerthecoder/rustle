use std::io::{self, BufRead, BufReader, Write}; // 0.7.3
use std::cmp;

#[derive(Copy, Clone, PartialEq, Eq)]
enum LetterState {
    NotPresent,
    WrongLocation,
    Correct,
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct LetterResult {
    state: LetterState,
    letter: char,
}

type GuessResult = [LetterResult; 5];

type Board = Vec<GuessResult>;

fn print_board(board: Board) {
    let clear_scree_code = "\x1b[2J";
    let move_cursor_to_top_code = "\x1B[1;1H";
    let clear_style_code = "\x1b[0m";

    let lines = board
        .into_iter()
        .map(|guess| {
            guess
                .into_iter()
                .map(|letter_state| {
                    if letter_state.state == LetterState::Correct {
                        format!("{}{}{}", "\x1b[42m", letter_state.letter, clear_style_code)
                    } else if letter_state.state == LetterState::WrongLocation {
                        format!(
                            "{}{}{}",
                            "\x1b[30;43m", letter_state.letter, clear_style_code
                        )
                    } else {
                        format!("{}", letter_state.letter)
                    }
                })
                .fold(String::new(), |a, b| a + &b)
                + "\n"
        })
        .fold(String::new(), |a, b| a + &b);

    print!("{}{}{}:", clear_scree_code, move_cursor_to_top_code, lines);
    std::io::stdout().flush().unwrap();
}

// First we need a way of generating valid equations of length 8
// Oops this is hard

#[derive(Debug)]
struct Bounds {
    min: i32,
    max: i32,
}

impl Bounds {
    /// Returns bounds that the target and source bounds share
    fn intersect(&self, b: &Bounds) -> Bounds {
        Bounds {
            min: cmp::max(self.min, b.min),
            max: cmp::min(self.max, b.max),
        }
    }

    fn union(&self, b: &Bounds) -> Bounds {
        Bounds {
            min: cmp::min(self.min, b.min),
            max: cmp::max(self.max, b.max),
        }
    }

    fn is_valid(&self) -> bool {
        self.min < self.max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounds_intersect() {
        let b1 = Bounds { min: 4, max: 6};
        let b2 = Bounds { min: 3, max: 5};
        let b3 = Bounds { min: 6, max: 9};

        let b4 = b1.intersect(&b2);
        let b5 = b2.intersect(&b3);

        assert_eq!(b4.min, 4);
        assert_eq!(b4.max, 5);
        assert_eq!(b5.is_valid(), false);
    }

    #[test]
    fn bounds_union() {
        let b1 = Bounds { min: 4, max: 6};
        let b2 = Bounds { min: 3, max: 5};

        let b4 = b1.union(&b2);

        assert_eq!(b4.min, 3);
        assert_eq!(b4.max, 6);
    }

}

