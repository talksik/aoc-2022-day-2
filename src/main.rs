#[derive(Debug, Copy, Clone, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn from_str(s: &str) -> Move {
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

fn main() {
    let input = get_input();
    let mut score = 0;
    input.lines().for_each(|line| {
        let opponent_move = Move::from_str(line[0..1].trim());
        let my_move = Move::from_str(line[2..3].trim());
        let round_score = my_move.get_score(opponent_move);
        score += my_move.get_score(opponent_move);

        println!(
            "Opponent: {:?}, Me: {:?}, Roundscore: {:?}, Score: {:?}",
            opponent_move, my_move, round_score, score
        );
    });

    println!("Final score: {:?}", score);
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
