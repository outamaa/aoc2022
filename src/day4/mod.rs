use std::collections::HashSet;
use std::str::FromStr;

struct ElfPair(HashSet<u32>, HashSet<u32>);

impl FromStr for ElfPair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elfs = s.split(",")
            .filter_map(|range| {
                let mut range = range
                    .split("-")
                    .filter_map(|n| n.parse::<u32>().ok());
                let start = range.next()?;
                let end = range.next()?;

                Some((start..=end).collect())
            });

        Ok(ElfPair(
            elfs.next().unwrap(),
            elfs.next().unwrap(),
        ))
    }
}

impl ElfPair {
    fn one_contains_other(&self) -> bool {
        self.0.is_subset(&self.1) || self.1.is_subset(&self.0)
    }

    fn is_overlapping(&self) -> bool {
        self.0.intersection(&self.1).next().is_some()
    }
}

fn parse_elfs(file: &str) -> impl Iterator<Item=ElfPair> + '_ {
    file
        .lines()
        .filter_map(|line| line.parse::<ElfPair>().ok())
}

fn pairs_with_fully_contained_assignments(elfs: impl Iterator<Item=ElfPair>) -> impl Iterator<Item=ElfPair> {
    elfs
        .filter(ElfPair::one_contains_other)
}

fn pairs_with_overlap(elfs: impl Iterator<Item=ElfPair>) -> impl Iterator<Item=ElfPair> {
    elfs
        .filter(ElfPair::is_overlapping)
}

#[cfg(test)]
mod tests {
    use crate::day4::{ElfPair, pairs_with_fully_contained_assignments, pairs_with_overlap, parse_elfs};

    #[test]
    fn test_example1() {
        let input = include_str!("example.txt");
        let elfs = parse_elfs(input);
        assert_eq!(
            pairs_with_fully_contained_assignments(elfs).collect::<Vec<ElfPair>>().len(),
            2
        )
    }

    #[test]
    fn test_input1() {
        let input = include_str!("input.txt");
        let elfs = parse_elfs(input);
        assert_eq!(
            pairs_with_fully_contained_assignments(elfs).collect::<Vec<ElfPair>>().len(),
            464
        )
    }

    #[test]
    fn test_example2() {
        let input = include_str!("example.txt");
        let elfs = parse_elfs(input);
        assert_eq!(
            pairs_with_overlap(elfs).collect::<Vec<ElfPair>>().len(),
            4
        )
    }

    #[test]
    fn test_input2() {
        let input = include_str!("input.txt");
        let elfs = parse_elfs(input);
        assert_eq!(
            pairs_with_overlap(elfs).collect::<Vec<ElfPair>>().len(),
            770
        )
    }
}