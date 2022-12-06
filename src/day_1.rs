use crate::DynResult;
use std::num::ParseIntError;

fn get_sorted_calories(data: &str) -> DynResult<Vec<u32>> {
    let mut calories: Vec<u32> = data
        .lines()
        .try_fold(
            (vec![0], 0),
            |(mut acc, i), line| -> Result<(Vec<u32>, usize), ParseIntError> {
                if line.is_empty() {
                    acc.push(0);
                    return Ok((acc, i + 1));
                }

                acc[i] += line.parse::<u32>()?;

                Ok((acc, i))
            },
        )?
        .0;
    calories.sort();

    Ok(calories)
}

fn top_calories(calories: &[u32]) -> u32 {
    calories.last().unwrap().to_owned()
}

fn top_3_calories(calories: &[u32]) -> u32 {
    let len = calories.len();

    calories[len - 3..=len - 1].iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::DynResult;

    #[test]
    fn check_top_calories() -> DynResult<()> {
        let data = include_str!("../data/day-1-input.txt");
        let calories = super::get_sorted_calories(data)?;
        let result = super::top_calories(&calories);
        println!("day_1: Top calories: {result}");

        Ok(())
    }

    #[test]
    fn check_top_3_calories() -> DynResult<()> {
        let data = include_str!("../data/day-1-input.txt");
        let calories = super::get_sorted_calories(data)?;
        let result = super::top_3_calories(&calories);
        println!("day_1: Top 3 calories: {result}");

        Ok(())
    }
}
