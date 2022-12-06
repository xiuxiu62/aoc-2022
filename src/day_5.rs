use std::fmt::Display;

const WIDTH: usize = 9;

#[derive(Debug)]
pub struct Inventory(Vec<Vec<char>>);

impl Inventory {
    pub fn top_crates(self) -> String {
        self.0.into_iter().fold("".to_owned(), |acc, column| {
            format!("{}{}", acc, column.last().unwrap())
        })
    }
}

impl From<Vec<Vec<Option<char>>>> for Inventory {
    fn from(mut rows: Vec<Vec<Option<char>>>) -> Self {
        rows.reverse();

        Self(rows.into_iter().fold(vec![vec![]; WIDTH], |acc, row| {
            row.into_iter().enumerate().fold(acc, |mut acc, (i, item)| {
                if let Some(item) = item {
                    acc[i].push(item);
                }

                acc
            })
        }))
    }
}

impl Display for Inventory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = self
            .0
            .iter()
            .enumerate()
            .fold("".to_owned(), |acc, (i, column)| {
                format!(
                    "{acc}{} {}\n",
                    i + 1,
                    column
                        .into_iter()
                        .fold("".to_owned(), |acc, item| format!("{acc}{item}"))
                )
            });

        write!(f, "{message}")
    }
}

#[derive(Debug)]
pub struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    pub fn apply(self, inventory: &mut Inventory) {
        (0..self.count).for_each(|_| {
            let item = inventory.0[self.from].pop().unwrap();
            inventory.0[self.to].push(item);
        });
    }

    pub fn multi_apply(self, inventory: &mut Inventory) {
        let post_length = inventory.0[self.from].len() - self.count;
        let xs: Vec<char> = inventory.0[self.from].drain(post_length..).collect();
        xs.into_iter()
            .for_each(|char| inventory.0[self.to].push(char));
    }
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let values: Vec<usize> = value
            .split_whitespace()
            .flat_map(|word| word.parse::<usize>())
            .collect();

        Self {
            count: values[0],
            from: values[1] - 1,
            to: values[2] - 1,
        }
    }
}

pub fn parse_row(data: &str) -> Vec<Option<char>> {
    data.chars()
        .collect::<Vec<char>>()
        .chunks(4)
        .map(|chunk| chunk.contains(&'[').then_some(chunk[1]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{Instruction, Inventory};

    enum Line {
        Row(Vec<Option<char>>),
        Instruction(Instruction),
    }

    fn parse_line(line: &str) -> Option<Line> {
        if line.contains('[') {
            return Some(Line::Row(super::parse_row(line)));
        }

        if line.contains("move") {
            return Some(Line::Instruction(Instruction::from(line)));
        }

        None
    }

    fn parse_data() -> (Inventory, Vec<Instruction>) {
        let (rows, instructions) = include_str!("../data/day-5-input.txt")
            .lines()
            .filter_map(parse_line)
            .fold((vec![], vec![]), |(mut rows, mut instructions), line| {
                match line {
                    Line::Row(row) => rows.push(row),
                    Line::Instruction(instruction) => instructions.push(instruction),
                };

                (rows, instructions)
            });

        (Inventory::from(rows), instructions)
    }

    #[test]
    fn top_crates() {
        let (mut inventory, instructions) = parse_data();
        instructions
            .into_iter()
            .for_each(|instruction| instruction.apply(&mut inventory));
        println!("day_5: top_crates: {}", inventory.top_crates());
    }

    #[test]
    fn top_crates_multi() {
        let (mut inventory, instructions) = parse_data();
        instructions
            .into_iter()
            .for_each(|instruction| instruction.multi_apply(&mut inventory));
        println!("day_5: top_crates_multi: {}", inventory.top_crates());
    }
}
