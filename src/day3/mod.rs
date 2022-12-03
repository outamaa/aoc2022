use std::collections::HashSet;
use std::hash::Hash;
use std::str::FromStr;

struct Rucksack {
    left: HashSet<char>,
    right: HashSet<char>,
}

impl Rucksack {
    // Assumes zero to one
    fn common_item_in_compartments(&self) -> char {
        // Unwrap ok due to invariant
        *self.left.intersection(&self.right).take(1).next().unwrap()
    }

    fn all_items(&self) -> HashSet<char> {
        self.left.union(&self.right).cloned().collect()
    }
}

impl FromStr for Rucksack {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            left: s.chars().take(s.len()/2).collect(),
            right: s.chars().rev().take(s.len()/2).collect(),
        })
    }
}

fn priority(c: char) -> u32 {
    if c.is_uppercase() {
        c as u32 - 'A' as u32 + 27
    } else {
        c as u32 - 'a' as u32 + 1
    }
}

fn sum_priorities(file: &str) -> u32 {
    file
        .lines()
        .filter_map(|line| line.parse::<Rucksack>().ok())
        .map(|r| priority(r.common_item_in_compartments()))
        .sum()
}

// Part two

fn common_in_three(a: &Rucksack, b: &Rucksack, c: &Rucksack) -> char  {
    let ab: HashSet<char> = a.all_items().intersection(&b.all_items()).cloned().collect();
    *ab.intersection(&c.all_items()).take(1).next().unwrap()
}

fn sum_group_priorities(file: &str) -> u32 {
    let mut rucksacks = file
        .lines()
        .filter_map(|line| line.parse::<Rucksack>().ok())
        .peekable();

    let mut sum: u32 = 0;

    while rucksacks.peek().is_some() {
        sum += priority(common_in_three(
        &rucksacks.next().unwrap(),
        &rucksacks.next().unwrap(),
        &rucksacks.next().unwrap(),
        ));
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::day3::{sum_group_priorities, sum_priorities};

    #[test]
    fn test_example1() {
        let input = include_str!("example.txt");

        assert_eq!(
            sum_priorities(input),
            157
        )
    }

    #[test]
    fn test_input1() {
        let input = include_str!("input.txt");

        assert_eq!(
            sum_priorities(input),
            7831
        )
    }

    #[test]
    fn test_example2() {
        let input = include_str!("example.txt");

        assert_eq!(
            sum_group_priorities(input),
            70
        )
    }

    #[test]
    fn test_input2() {
        let input = include_str!("input.txt");

        assert_eq!(
            sum_group_priorities(input),
            7831
        )
    }
}