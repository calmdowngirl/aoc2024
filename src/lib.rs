use std::env::current_dir;
use std::fs;
use std::io::{self, Write};

mod sol;

pub struct InfiniteIterator {
    pub state: usize,
}
impl Iterator for InfiniteIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.state;
        self.state = (self.state + 1) % 4;
        Some(result)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Position(i32, i32);
impl Position {
    fn add(&self, other: Position) -> Position {
        Position(self.0 + other.0, self.1 + other.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sol::{sol_01, sol_02, sol_03, sol_04, sol_05, sol_06, sol_07, sol_08, sol_09, sol_10};

    #[test]
    fn day_03() -> Result<(), std::io::Error> {
        assert_eq!(
            sol_05("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",),
            161
        );

        assert_eq!(
            sol_06("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        );

        // let contents = fs::read_to_string("data/input_05")?;
        // let result = sol_05(&contents);
        // println!("{:?}", result);

        // let result = sol_06(&contents);
        // println!("{:?}", result);

        Ok(())
    }

    #[test]
    fn day_04() {
        assert_eq!(sol_07("data/test_data_07"), 18);
        assert_eq!(sol_08("data/test_data_07"), 9);

        // let result = sol_08("data/input_07");
        // println!("{}", result)
    }

    #[test]
    fn day_05() {
        assert_eq!(sol_09("data/test_data_09"), (143, 123));

        // let result = sol_09("data/input_09");
        // println!("{:?}", result)
    }

    #[test]
    fn day_06() {
        assert_eq!(sol_10("data/test_data_10"), 41);

        // let result = sol_10("data/input_10");
        // println!("{:?}", result)
    }

    // #[test]
}
