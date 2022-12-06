use std::{collections::HashSet, fmt::Display};

macro_rules! hashset_from_iter {
    ($iter: expr) => {{
        let mut set = HashSet::new();
        $iter.for_each(|item| {
            set.insert(item);
        });

        set
    }};
}

pub struct Rucksack(HashSet<char>, HashSet<char>);

impl Rucksack {
    pub fn sum_duplicate_priorities(self) -> u16 {
        let duplicates = self.0.intersection(&self.1);

        duplicates
            .into_iter()
            .map(|item| get_priority(*item) as u16)
            .sum()
    }
}

impl From<&str> for Rucksack {
    fn from(inventory: &str) -> Self {
        let create_comparments = |(lhs, rhs): (&str, &str)| {
            (
                hashset_from_iter!(lhs.chars()),
                hashset_from_iter!(rhs.chars()),
            )
        };
        let (lhs, rhs) = create_comparments(inventory.split_at(inventory.len() / 2));

        Self(lhs, rhs)
    }
}

impl Display for Rucksack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let duplicates = self.0.intersection(&self.1);
        let sum: u8 = duplicates
            .clone()
            .into_iter()
            .map(|item| get_priority(*item))
            .sum();

        write!(f, "{duplicates:?} => {sum}")
    }
}

#[derive(Clone)]
pub struct Inventory(HashSet<char>);

impl Inventory {
    // fn duplicate_of_three(self, xs: [Self; 2]) -> char {
    //     Self::find_duplicate(
    //         self.0
    //             .intersection(&xs[0].0)
    //             .into_iter()
    //             .chain(self.0.intersection(&xs[0].0))
    //             .collect(),
    //     )
    //     .unwrap()
    // }

    // fn find_duplicate(mut items: Vec<&char>) -> Option<char> {
    //     items.sort_unstable_by_key(|char| *char);
    //     let (_, duplicates) = items.partition_dedup_by_key(|char| *char);
    //     println!("{duplicates:?}");

    //     duplicates.first().map(|char| **char)
    // }

    fn duplicate_of_three(self, xs: [Self; 2]) -> char {
        xs[1]
            .0
            .intersection(&hashset_from_iter!(self
                .0
                .intersection(&xs[0].0)
                .into_iter()
                .cloned()))
            .cloned()
            .collect::<Vec<char>>()
            .first()
            .unwrap()
            .to_owned()
    }
}

impl From<&str> for Inventory {
    fn from(data: &str) -> Self {
        Self(hashset_from_iter!(data.chars()))
    }
}

fn get_priority(char: char) -> u8 {
    if char.is_lowercase() {
        return char as u8 - b'a' + 1;
    }

    char as u8 - b'A' + 27
}

#[cfg(test)]
mod tests {
    use super::{Inventory, Rucksack};

    #[test]
    fn part_1() {
        let data = include_str!("../data/day-3-input.txt");
        let priority_sums: u16 = data
            .lines()
            .map(Rucksack::from)
            .map(|sack| sack.sum_duplicate_priorities())
            .sum();

        println!("day_3: Priority sums: {priority_sums}");
    }

    #[test]
    fn part_2() {
        let data = include_str!("../data/day-3-input.txt");
        let sum: usize = data
            .lines()
            .map(Inventory::from)
            .collect::<Vec<Inventory>>()
            .chunks(3)
            .map(|chunk| {
                chunk[0]
                    .to_owned()
                    .duplicate_of_three([chunk[1].to_owned(), chunk[2].to_owned()])
            })
            .map(super::get_priority)
            .map(|priority| priority as usize)
            .sum();

        println!("day_3: part_2: {sum}");
    }
}
