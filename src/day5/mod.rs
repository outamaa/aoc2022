use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Stacks(Vec<Vec<char>>);

#[derive(Clone, Copy, PartialEq)]
enum CraneModel {
    CrateMover9000,
    CrateMover9001
}

impl FromStr for Stacks {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Assume `s` contains both stacks and instructions
        let mut lines: Vec<&str> = s.lines().take_while(|line| !line.is_empty()).collect();

        let num_of_stacks = lines
            .pop().unwrap().trim().chars().rev().next().unwrap() as usize - 48;
        lines.reverse();

        let mut stacks = vec![vec![]; num_of_stacks];

        for line in lines {
            for (idx, chunk) in line.chars().collect::<Vec<char>>().chunks(4).enumerate() {
                let c = chunk[1];
                if c != ' ' {
                    stacks[idx].push(chunk[1]);
                }
            }
        }

        Ok(Self(stacks))
    }
}

impl Stacks {
    fn arrange(&mut self, instructions: &[Instruction], crane_model: CraneModel) {
        for i in instructions {
            self.move_crates(i, crane_model);
        }
    }

    fn move_crates(&mut self, Instruction { amount, from, to }: &Instruction, crane_model: CraneModel) {
        use CraneModel::*;
        let mut to_move = Vec::with_capacity(*amount);
        for _ in 0..*amount {
            let c = self.0[*from].pop().unwrap();
            to_move.push(c);
        }
        if crane_model == CrateMover9001 {
            to_move.reverse();
        }
        self.0[*to].append(&mut to_move);
    }

    fn tops(&self) -> String {
        self.0
            .iter()
            // Assume at least one char in stack
            .map(|stack| stack[stack.len() - 1])
            .collect()
    }
}


#[derive(Debug, PartialEq)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(' ');
        tokens.next();
        let amount = tokens.next().unwrap().parse::<usize>().unwrap();
        tokens.next();
        let from = tokens.next().unwrap().parse::<usize>().unwrap() - 1;
        tokens.next();
        let to = tokens.next().unwrap().parse::<usize>().unwrap() - 1;

        Ok(Self {
            amount,
            from,
            to,
        })
    }
}

fn parse_instructions(file: &str) -> impl Iterator<Item=Instruction> + '_ {
    file
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .filter_map(|line| line.parse().ok())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::day5::{Instruction, parse_instructions, Stacks};
    use crate::day5::CraneModel::*;


    #[test]
    fn test_example1() {
        let input = include_str!("example.txt");
        let mut stacks = Stacks::from_str(input).unwrap();
        let instructions: Vec<Instruction> = parse_instructions(input).collect();

        stacks.arrange(instructions.as_slice(), CrateMover9000);
        assert_eq!(
            &stacks.tops(),
            "CMZ"
        );
    }

    #[test]
    fn test_input1() {
        let input = include_str!("input.txt");
        let mut stacks = Stacks::from_str(input).unwrap();
        let instructions: Vec<Instruction> = parse_instructions(input).collect();

        stacks.arrange(instructions.as_slice(), CrateMover9000);
        assert_eq!(
            &stacks.tops(),
            "TGWSMRBPN"
        );
    }

    #[test]
    fn test_example2() {
        let input = include_str!("example.txt");
        let mut stacks = Stacks::from_str(input).unwrap();
        let instructions: Vec<Instruction> = parse_instructions(input).collect();

        stacks.arrange(instructions.as_slice(), CrateMover9001);
        assert_eq!(
            &stacks.tops(),
            "MCD"
        );
    }

    #[test]
    fn test_input2() {
        let input = include_str!("input.txt");
        let mut stacks = Stacks::from_str(input).unwrap();
        let instructions: Vec<Instruction> = parse_instructions(input).collect();

        stacks.arrange(instructions.as_slice(), CrateMover9001);
        assert_eq!(
            &stacks.tops(),
            "TZLTLWRNF"
        );
    }

}