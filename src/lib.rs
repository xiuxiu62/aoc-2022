#![allow(dead_code)]

pub type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6 {
    use std::collections::HashSet;

    fn solve(stream: &str, distinct_chars: usize) -> u32 {
        let chars: Vec<char> = stream.chars().collect();
        for i in 0..chars.len() - distinct_chars {
            let seq: HashSet<_> = chars[i..i + distinct_chars].iter().collect();
            if seq.len() == distinct_chars {
                return (i + distinct_chars) as u32;
            }
        }
        unreachable!("Could not find start of packet");
    }

    pub fn magic_index(packet: &[u8], unique_characters: usize) -> usize {
        for i in 0..(packet.len() - unique_characters + 1) {
            if packet[i..i + unique_characters]
                .iter()
                .collect::<HashSet<&u8>>()
                .len()
                == unique_characters
            {
                return i + unique_characters;
            }
        }

        packet.len()
    }

    #[cfg(test)]
    mod tests {
        fn run(unique_characters: usize) -> usize {
            super::magic_index(
                include_str!("../data/day-6-input.txt").as_bytes(),
                unique_characters,
            )
        }

        #[test]
        fn magic_4_unique() {
            println!("day_6: part_1: {}", run(4));
        }

        #[test]
        fn magic_14_unique() {
            println!("day_6: part_2: {}", run(14));
        }
    }
}
