use std::str::FromStr;

#[derive(Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

impl FromStr for Shape {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Shape::*;
        match s {
            "A" => Ok(Rock),
            "B" => Ok(Paper),
            "C" => Ok(Scissors),
            "X" => Ok(Rock),
            "Y" => Ok(Paper),
            "Z" => Ok(Scissors),
            _ => Err("Oh no".to_string())
        }
    }
}

impl Shape {
    fn score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

struct GameResult(Shape, Shape);

impl FromStr for GameResult {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut shapes = s
            .split(" ")
            .map(Shape::from_str)
            .take(2);

        Ok(
            Self(
                shapes.next().ok_or_else(no_next)??,
                shapes.next().ok_or_else(no_next)??
            )
        )
    }
}

fn no_next() -> String {
    "No next value".to_string()
}

enum Outcome {
    Win,
    Loss,
    Draw
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Loss => 0,
            Outcome::Draw => 3
        }
    }
}

impl GameResult {
    fn outcome(&self) -> Outcome {
        use Shape::*;
        use Outcome::*;

        match (&self.0, &self.1) {
            (Rock, Paper) => Win,
            (Paper, Rock) => Loss,
            (Paper, Scissors) => Win,
            (Scissors, Paper) => Loss,
            (Scissors, Rock) => Win,
            (Rock, Scissors) => Loss,
            _ => Draw
        }
    }

    fn own_shape(&self) -> &Shape {
        &self.1
    }

    fn score(&self) -> u32 {
        self.own_shape().score() + self.outcome().score()
    }
}

fn parse_scores(file: &str) -> impl Iterator<Item=GameResult>  + '_{
    file
        .lines()
        .filter_map(|line| line.parse().ok())
}

fn sum_scores(results: impl Iterator<Item=GameResult>) -> u32 {
    results
        .map(|result| result.score())
        .sum()
}

// part two

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Outcome::*;
        match s {
            "X" => Ok(Loss),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            _ => Err("Oh no".to_string())
        }
    }
}


struct Strategy(Shape, Outcome);

impl FromStr for Strategy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s
            .split(" ")
            .take(2);

        Ok(
            Self(
                line.next().ok_or_else(no_next)?.parse()?,
                line.next().ok_or_else(no_next)?.parse()?
            )
        )
    }

}

fn strategy_to_result(Strategy(shape, outcome): Strategy) -> GameResult {
    use Shape::*;
    use Outcome::*;
    match outcome {
        Win => GameResult(shape, match shape {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }),
        Loss => GameResult(shape, match shape {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }),
        Draw => GameResult(shape, shape)
    }
}

fn parse_strategies(file: &str) -> impl Iterator<Item=Strategy>  + '_{
    file
        .lines()
        .filter_map(|line| line.parse().ok())
}


#[cfg(test)]
mod tests {
    use crate::day2::{parse_scores, parse_strategies, strategy_to_result, sum_scores};

    #[test]
    fn test_example() {
        let example = include_str!("example.txt");
        let scores = parse_scores(example);

        assert_eq!(
            sum_scores(scores),
            15
        );
    }

    #[test]
    fn test_input() {
        let input = include_str!("input.txt");
        let scores = parse_scores(input);

        assert_eq!(
            sum_scores(scores),
            15422
        );

    }

    #[test]
    fn test_example2() {
        let example = include_str!("example.txt");
        let sum_of_scores: u32 = parse_strategies(example)
            .map(strategy_to_result)
            .map(|result| result.score())
            .sum();

        assert_eq!(
            sum_of_scores,
            12
        );
    }

    #[test]
    fn test_input2() {
        let example = include_str!("input.txt");
        let sum_of_scores: u32 = parse_strategies(example)
            .map(strategy_to_result)
            .map(|result| result.score())
            .sum();

        assert_eq!(
            sum_of_scores,
            15442
        );
    }

}