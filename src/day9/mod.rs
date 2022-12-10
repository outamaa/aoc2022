use std::collections::HashSet;
use std::ops::{Add, Sub};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Instruction {
    direction: Direction,
    amount: usize
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;

        let mut parts = s.split(' ');
        let direction = match parts.next().unwrap() {
            "L" => Ok(Left),
            "R" => Ok(Right),
            "U" => Ok(Up),
            "D" => Ok(Down),
            other => Err(other.to_string()),
        }?;
        let amount = parts.next().unwrap().parse::<usize>().unwrap();

        Ok(Instruction { direction, amount })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2 { x: i32, y: i32 }
impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output { Self { x: self.x + rhs.x, y: self.y + rhs.y } }
}
impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output { Self { x: self.x - rhs.x, y: self.y - rhs.y } }
}
impl From<Direction> for Vec2 {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Left => Self { x: -1, y: 0 },
            Direction::Right => Self { x: 1, y: 0 },
            Direction::Up => Self { x: 0, y: 1 },
            Direction::Down => Self { x: 0, y: -1 }
        }
    }
}
impl Vec2 {
    fn new() -> Self { Self { x: 0, y: 0 } }
    fn l_inf_norm(&self, other: Vec2) -> i32 {
        let diff = *self - other;
        diff.x.abs().max(diff.y.abs())
    }

    // Assume there always is a largest direction
    fn move_next_to(&self, other: Vec2) -> Vec2 {
        use Direction::*;
        match other - *self {
            Self { x, y } if x.abs() > y.abs() && x > 0 => other + Left.into(),
            Self { x, y } if x.abs() > y.abs() && x < 0 => other + Right.into(),
            Self { x, y } if x.abs() < y.abs() && y > 0 => other + Down.into(),
            Self { x, y } if x.abs() < y.abs() && y < 0 => other + Up.into(),
            Self { x, y } => other + Self { x: -x/(x.abs()), y: -y/(y.abs()) },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Rope<const N: usize> {
    knots: [Vec2; N]
}

impl<const N: usize> Rope<N> {
    fn new() -> Self { Self { knots: [Vec2::new(); N] } }
    fn head(&self) -> Vec2 { self.knots[0] }
    fn last(&self) -> Vec2 { self.knots[N-1] }

    fn move_by_instrcutions(&self, instructions: impl Iterator<Item=Instruction>) -> (Self, HashSet<Vec2>) {
        instructions
            .fold((*self, HashSet::from([self.last()])), |(rope, mut coords), instruction| {
                let (rope, new_coords) = rope.move_by_instruction(instruction);
                coords.extend(new_coords);
                (rope, coords)
            })
    }

    fn move_by_instruction(&self, instruction: Instruction) -> (Self, HashSet<Vec2>) {
        (0..instruction.amount)
            .fold((*self, HashSet::from([self.last()])), |(rope, mut coords), _| {
                let rope = rope.move_knots(instruction.direction);
                coords.insert(rope.last());
                (rope, coords)
            })
    }

    fn move_knots(&self, direction: Direction) -> Self {
        let head = self.head() + direction.into();
        let knots: [Vec2; N] = self.knots.iter().skip(1)
            .fold(vec![head], |mut new_knots, next_old_knot| {
                let new_knot = Self::pull_tail(*next_old_knot, *new_knots.last().unwrap());
                new_knots.push(new_knot);
                new_knots
            })
            .try_into()
            .unwrap();
        Self { knots }
    }

    fn pull_tail(old_tail: Vec2, new_head: Vec2) -> Vec2 {
        if old_tail.l_inf_norm(new_head) < 2 {
            old_tail
        } else {
            old_tail.move_next_to(new_head)
        }
    }
}

fn parse_instructions(s: &str) -> impl Iterator<Item=Instruction> + '_ {
    s.lines()
        .filter_map(|line| line.parse::<Instruction>().ok())
}

#[cfg(test)]
mod tests {
    use test::Bencher;
    use crate::day9::{parse_instructions, Rope};

    #[test]
    fn test_example1() {
        let input = include_str!("example.txt");
        let instructions = parse_instructions(input);

        let rope = Rope::<2>::new();
        let (_, coords) = rope.move_by_instrcutions(instructions);

        assert_eq!(
            coords.len(),
            13
        )
    }

    #[test]
    fn test_input1() {
        let input = include_str!("input.txt");
        let instructions = parse_instructions(input);

        let rope = Rope::<2>::new();
        let (_, coords) = rope.move_by_instrcutions(instructions);

        assert_eq!(
            coords.len(),
            6642
        )
    }

    #[test]
    fn test_example2() {
        let input = include_str!("example.txt");
        let instructions = parse_instructions(input);

        let rope = Rope::<10>::new();
        let (_, coords) = rope.move_by_instrcutions(instructions);

        assert_eq!(
            coords.len(),
            1
        )
    }

    #[test]
    fn test_new_example2() {
        let input = include_str!("example2.txt");
        let instructions = parse_instructions(input);

        let rope = Rope::<10>::new();
        let (_, coords) = rope.move_by_instrcutions(instructions);

        assert_eq!(
            coords.len(),
            36
        )
    }

    #[test]
    fn test_input2() {
        let input = include_str!("input.txt");
        let instructions = parse_instructions(input);

        let rope = Rope::<10>::new();
        let (_, coords) = rope.move_by_instrcutions(instructions);

        assert_eq!(
            coords.len(),
            2602
        )
    }

    #[bench]
    fn bench_input2(b: &mut Bencher) {
        let input = include_str!("input.txt");

        b.iter(|| {
            let instructions = parse_instructions(input);

            let rope = Rope::<10>::new();
            rope.move_by_instrcutions(instructions);
        });
    }
}