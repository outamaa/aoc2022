use std::collections::HashSet;

fn parse_calories(file: &str) -> Vec<HashSet<u32>> {
    let (mut vec, set) = file
        .lines()
        .map(|line| {
            if line.is_empty() {
                None
            } else {
                line.parse().ok()
            }
        })
        .fold((Vec::new(), HashSet::new()), |(mut vec, mut set), value| {
            match value {
                Some(calories) => {
                    set.insert(calories);
                    (vec, set)
                }
                None => {
                    vec.push(set);
                    (vec, HashSet::new())
                }
            }
        });
    if !set.is_empty() {
        vec.push(set);
    }

    vec
}

fn most_calories(elves: &Vec<HashSet<u32>>) -> u32 {
    elves
        .iter()
        .map(|calories| calories.iter().sum())
        .max()
        .unwrap()
}

fn top_three(elves: &Vec<HashSet<u32>>) -> u32 {
    let mut elves = elves
        .iter()
        .map(|calories| calories.iter().sum())
        .collect::<Vec<u32>>();

    elves.sort_by(|a, b| b.cmp(&a));

    elves.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::day1::{most_calories, parse_calories, top_three};

    #[test]
    fn test_parse_example() {
        let example = include_str!("example.txt");
        let calories = parse_calories(example);
        assert_eq!(
            calories,
            vec![
                HashSet::from([
                    1000,
                    2000,
                    3000
                ]),
                HashSet::from([
                    4000
                ]),
                HashSet::from([
                    5000,
                    6000
                ]),
                HashSet::from([
                    7000,
                    8000,
                    9000
                ]),
                HashSet::from([
                    10000
                ])
            ]
        )
    }

    #[test]
    fn test_example_most_calories() {
        let example = include_str!("example.txt");
        let calories = parse_calories(example);

        assert_eq!(
            most_calories(&calories),
            24000
        )
    }

    #[test]
    fn test_input_most_calories() {
        let input = include_str!("input.txt");
        let calories = parse_calories(input);

        assert_eq!(
            most_calories(&calories),
            68775
        )
    }

    #[test]
    fn test_example_top_three() {
        let example = include_str!("example.txt");
        let calories = parse_calories(example);

        assert_eq!(
            top_three(&calories),
            45000
        )
    }

    #[test]
    fn test_input_top_three() {
        let input = include_str!("input.txt");
        let calories = parse_calories(input);

        assert_eq!(
            top_three(&calories),
            202585
        )
    }
}