use crate::DynResult;

enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn score(self, opponent: Move) -> usize {
        match self {
            Self::Rock => {
                1 + match opponent {
                    Self::Rock => 3,
                    Self::Paper => 0,
                    Self::Scissors => 6,
                }
            }
            Self::Paper => {
                2 + match opponent {
                    Self::Rock => 6,
                    Self::Paper => 3,
                    Self::Scissors => 0,
                }
            }
            Self::Scissors => {
                3 + match opponent {
                    Self::Rock => 0,
                    Self::Paper => 6,
                    Self::Scissors => 3,
                }
            }
        }
    }

    pub fn score_decided(self, opponent: Move) -> usize {
        match self {
            // X: Lose
            Self::Rock => match opponent {
                Self::Rock => 3,
                Self::Paper => 1,
                Self::Scissors => 2,
            },
            // Y: Draw
            Self::Paper => {
                3 + match opponent {
                    Self::Rock => 1,
                    Self::Paper => 2,
                    Self::Scissors => 3,
                }
            }
            // Z: Win
            Self::Scissors => {
                6 + match opponent {
                    Self::Rock => 2,
                    Self::Paper => 3,
                    Self::Scissors => 1,
                }
            }
        }
    }
}

impl TryFrom<char> for Move {
    type Error = Box<dyn std::error::Error>;

    fn try_from(char: char) -> DynResult<Self> {
        match char {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
            _ => Err("Invalid character: {char}".into()),
        }
    }
}

fn total_score(data: &str) -> DynResult<usize> {
    let parse_line = |line: &str| -> Vec<char> {
        line.chars()
            .filter(|char| char.is_ascii_alphabetic())
            .collect()
    };
    let score_pair = |acc: usize, pair: Vec<char>| -> DynResult<usize> {
        let left = Move::try_from(pair[0])?;
        let right = Move::try_from(pair[1])?;

        Ok(acc + right.score(left))
    };

    data.lines().map(parse_line).try_fold(0, score_pair)
}

fn total_score_decided(data: &str) -> DynResult<usize> {
    let parse_line = |line: &str| -> Vec<char> {
        line.chars()
            .filter(|char| char.is_ascii_alphabetic())
            .collect()
    };
    let score_pair = |acc: usize, pair: Vec<char>| -> DynResult<usize> {
        let left = Move::try_from(pair[0])?;
        let right = Move::try_from(pair[1])?;

        Ok(acc + right.score_decided(left))
    };

    data.lines().map(parse_line).try_fold(0, score_pair)
}

#[cfg(test)]
mod tests {
    use crate::DynResult;

    #[test]
    fn check_total_score() -> DynResult<()> {
        let data = include_str!("../data/day-2-input.txt");
        let total = super::total_score(data)?;
        println!("day_2: Total score: {total}");

        Ok(())
    }

    #[test]
    fn check_total_score_decided() -> DynResult<()> {
        let data = include_str!("../data/day-2-input.txt");
        let total_decided = super::total_score_decided(data)?;
        println!("day_2: Total score decided: {total_decided}");

        Ok(())
    }
}
