#[derive(Debug, Copy, Clone, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum ElfSuggestion {
    Lose,
    Draw,
    Win,
}

// based on what the elf suggests, choose
// the move based on the other players move
fn choose_move(opponent_move: Move, elf_suggestion: ElfSuggestion) -> Move {
    match elf_suggestion {
        ElfSuggestion::Lose => match opponent_move {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        },
        ElfSuggestion::Draw => opponent_move,
        ElfSuggestion::Win => match opponent_move {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        },
    }
}

impl ElfSuggestion {
    fn from_str(s: &str) -> ElfSuggestion {
        match s {
            "X" => ElfSuggestion::Lose,
            "Y" => ElfSuggestion::Draw,
            "Z" => ElfSuggestion::Win,
            _ => panic!("Invalid elf suggestion"),
        }
    }
}

impl Move {
    // this is based on part one of the problem
    fn from_str_chosen_move_scheme(s: &str) -> Move {
        match s {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => panic!("Invalid move"),
        }
    }

    // input what you put and what the other player put
    fn get_score(self, other: Move) -> i32 {
        match self {
            Move::Rock => {
                1 + match other {
                    Move::Rock => 3,
                    Move::Paper => 0,
                    Move::Scissors => 6,
                }
            }
            Move::Paper => {
                2 + match other {
                    Move::Rock => 6,
                    Move::Paper => 3,
                    Move::Scissors => 0,
                }
            }
            Move::Scissors => {
                3 + match other {
                    Move::Rock => 0,
                    Move::Paper => 6,
                    Move::Scissors => 3,
                }
            }
        }
    }
}

// the second column tells us the suggestion from the elf
fn part_two() {
    let input = get_input();
    let mut score = 0;
    input.lines().for_each(|line| {
        let opponent_move = Move::from_str_chosen_move_scheme(line[0..1].trim());
        let elf_suggestion = ElfSuggestion::from_str(line[2..3].trim());
        let elf_suggested_move = choose_move(opponent_move, elf_suggestion);

        let round_score = elf_suggested_move.get_score(opponent_move);
        score += round_score;

        println!(
            "Opponent: {:?}, elf_suggested: {:?}, Me: {:?}, Roundscore: {:?}, Score: {:?}",
            opponent_move, elf_suggestion, elf_suggested_move, round_score, score
        );
    });

    println!("Final score: {:?}", score);
}

fn part_one() {
    let input = get_input();
    let mut score = 0;
    input.lines().for_each(|line| {
        let opponent_move = Move::from_str_chosen_move_scheme(line[0..1].trim());
        let my_move = Move::from_str_chosen_move_scheme(line[2..3].trim());
        let round_score = my_move.get_score(opponent_move);
        score += round_score;

        println!(
            "Opponent: {:?}, Me: {:?}, Roundscore: {:?}, Score: {:?}",
            opponent_move, my_move, round_score, score
        );
    });

    println!("Final score: {:?}", score);
}

fn main() {
    part_two();
}

fn get_input() -> String {
    std::fs::read_to_string("input.txt").expect("Something went wrong reading the file")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_score() {
        assert_eq!(Move::Rock.get_score(Move::Rock), 4);
        assert_eq!(Move::Rock.get_score(Move::Paper), 1);
        assert_eq!(Move::Rock.get_score(Move::Scissors), 7);

        assert_eq!(Move::Paper.get_score(Move::Rock), 8);
        assert_eq!(Move::Paper.get_score(Move::Paper), 5);
        assert_eq!(Move::Paper.get_score(Move::Scissors), 2);

        assert_eq!(Move::Scissors.get_score(Move::Rock), 3);
        assert_eq!(Move::Scissors.get_score(Move::Paper), 9);
        assert_eq!(Move::Scissors.get_score(Move::Scissors), 6);
    }
}
