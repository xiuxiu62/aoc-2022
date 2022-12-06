use crate::DynResult;
use std::num::ParseIntError;

#[derive(Debug, Clone, Copy)]
struct Span<T> {
    start: T,
    end: T,
}

impl<T> Span<T>
where
    T: Ord + Copy,
{
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }

    pub fn full_overlap(&self, other: Span<T>) -> bool {
        self.start <= other.start && self.end >= other.end
            || self.start >= other.start && self.end <= other.end
    }

    pub fn partial_overlap(&self, other: Span<T>) -> bool {
        self.start.max(other.start) <= self.end.min(other.end)
    }
}

impl TryFrom<&str> for Span<i8> {
    type Error = Box<dyn std::error::Error>;

    fn try_from(data: &str) -> DynResult<Self> {
        let xs: Vec<i8> =
            data.split('-')
                .try_fold(vec![], |mut acc, x| -> Result<Vec<i8>, ParseIntError> {
                    acc.push(x.parse::<i8>()?);

                    Ok(acc)
                })?;

        Ok(Self {
            start: xs[0],
            end: xs[1],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Span;
    use crate::DynResult;

    fn parse_pairs() -> DynResult<Vec<Span<i8>>> {
        include_str!("../data/day-4-input.txt")
            .lines()
            .map(|line| line.split(','))
            .flatten()
            .map(Span::try_from)
            .collect()
    }

    fn run(description: &str, f: fn(pair: &&[Span<i8>]) -> bool) -> DynResult<()> {
        let sum = parse_pairs()?.chunks(2).filter(f).count();
        println!("day_4: {description}: {sum}");

        Ok(())
    }

    #[test]
    fn full_overlap() -> DynResult<()> {
        run("Full overlap", |pair| pair[0].full_overlap(pair[1]))
    }

    #[test]
    fn partial_overlap() -> DynResult<()> {
        run("Partial overlap", |pair| pair[0].partial_overlap(pair[1]))
    }
}
