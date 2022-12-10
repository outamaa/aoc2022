use std::str::FromStr;

struct Forest(Vec<Vec<u32>>);

impl FromStr for Forest {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s
                .lines()
                .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
                .collect()
        ))
    }
}

impl Forest {
    fn flipped(&self) -> Self {
        let flipped = self.0
            .iter()
            .cloned()
            .map(|mut row| {
                row.reverse();
                row
            })
            .collect();

        Self(flipped)
    }

    fn transposed(&self) -> Self {
        // Assume at least 1
        let rows = self.0.len();
        let cols = self.0[0].len();

        let mut t = vec![vec![self.0[0][0].clone(); rows]; cols];

        for r in 0..rows {
            for c in 0..cols {
                t[c][r] = self.0[r][c].clone();
            }
        }

        Self(t)
    }


    fn grid(&self) -> &Vec<Vec<u32>> {
        &self.0
    }

    fn left_max(&self) -> Self {
        let maxs = self.0
            .iter()
            .map(|row| row.iter().scan(0u32, |state, &tree_height| {
                let previous_max = *state;
                if tree_height > *state {
                    *state = tree_height;
                }

                Some(previous_max)
            })
                .collect())
            .collect();

        Forest(maxs)
    }

    fn right_max(&self) -> Self {
        self
            .flipped()
            .left_max()
            .flipped()
    }

    fn up_max(&self) -> Self {
        self
            .transposed()
            .left_max()
            .transposed()
    }

    fn down_max(&self) -> Self {
        self
            .transposed()
            .right_max()
            .transposed()
    }

    fn visible_trees(&self) -> Vec<Vec<bool>> {
        let lmax = self.left_max();
        let rmax = self.right_max();
        let umax = self.up_max();
        let dmax = self.down_max();

        let rows = self.0.len();
        let cols = self.0[0].len();

        let mut t = vec![vec![true; cols]; rows];

        for r in 1..(rows-1) {
            for c in 1..(cols-1) {
                let height = self.0[r][c];
                let lmax = lmax.grid()[r][c];
                let rmax = rmax.grid()[r][c];
                let umax = umax.grid()[r][c];
                let dmax = dmax.grid()[r][c];
                let is_visible = height > lmax || height > rmax || height > umax || height > dmax;

                t[r][c] = is_visible;
            }
        }

        t
    }

    fn number_of_visible_trees(&self) -> usize {
        self
            .visible_trees()
            .iter()
            .map(|row| row.iter().map(|&visible| if visible { 1 } else { 0 }).sum::<usize>())
            .sum()
    }

    fn scenic_scores(&self) -> Vec<Vec<u32>> {
        let rows = self.0.len();
        let cols = self.0[0].len();

        let mut t = vec![vec![0; cols]; rows];

        for r in 0..rows {
            for c in 0..cols {
                let row = self.row(r);
                let col = self.col(c);
                let height = self.0[r][c];
                let up = Self::num_of_trees_visible(col.iter().take(r).rev(), height);
                let down = Self::num_of_trees_visible(col.iter().skip(r + 1), height);
                let left = Self::num_of_trees_visible(row.iter().take(c).rev(), height);
                let right = Self::num_of_trees_visible(row.iter().skip(c + 1), height);

                t[r][c] = up * down * left * right;
            }
        }

        t
    }

    fn num_of_trees_visible<'a, I: Iterator<Item=&'a u32>>(trees: I, tree_height: u32) -> u32 {
        let v: Vec<u32> = trees.cloned().collect();
        let max_trees = v.len() as u32;
        if v.is_empty() {
            1
        } else {
            let number_of_smaller_trees = v
                .iter()
                .take_while(|&h| tree_height > *h)
                .count() as u32;
            if number_of_smaller_trees < max_trees { number_of_smaller_trees + 1 } else { max_trees }
        }
    }

    fn max_scenic_score(&self) -> u32 {
        *self
            .scenic_scores()
            .iter()
            .map(|row| row.iter().max().unwrap())
            .max()
            .unwrap()
    }

    fn row(&self, r: usize) -> Vec<u32> {
        self.0[r].clone()
    }

    fn col(&self, c: usize) -> Vec<u32> {
        self.0.iter()
            .map(|row| row[c])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;
    use crate::day8::Forest;

    #[test]
    fn test_example1() {
        let input = include_str!("example.txt");
        let forest: Forest = input.parse().unwrap();

        assert_eq!(
            forest.number_of_visible_trees(),
            21
        )
    }

    #[test]
    fn test_input1() {
        let input = include_str!("input.txt");
        let forest: Forest = input.parse().unwrap();

        assert_eq!(
            forest.number_of_visible_trees(),
            1543
        )
    }

    #[test]
    fn test_example2() {
        let input = include_str!("example.txt");
        let forest: Forest = input.parse().unwrap();

        assert_eq!(
            forest.max_scenic_score(),
            16,
        )
    }

    #[test]
    fn test_input2() {
        let input = include_str!("input.txt");
        let forest: Forest = input.parse().unwrap();

        assert_eq!(
            forest.max_scenic_score(),
            595080,
        )
    }

    #[bench]
    fn bench_input1(b: &mut Bencher) {
        let input = include_str!("input.txt");

        b.iter(|| {
            let forest: Forest = input.parse().unwrap();
            forest.number_of_visible_trees();
        })
    }

    #[bench]
    fn bench_input2(b: &mut Bencher) {
        let input = include_str!("input.txt");

        b.iter(|| {
            let forest: Forest = input.parse().unwrap();
            forest.max_scenic_score();
        })
    }
}
