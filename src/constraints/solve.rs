use std::collections::HashMap;

use super::typedefs::{Puzzle, Solution};

pub fn solve(puzzle: Puzzle) -> Solution {
  Solution {
    puzzle,
    elements: HashMap::new(),
  }
}