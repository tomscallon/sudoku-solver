use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Position {
  row: u8,
  col: u8,
}

impl fmt::Display for Position {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "(r: {}, c: {})", self.row, self.col)
  }
}

#[derive(Copy, Clone)]
pub enum CellPosition {
  START,
  MIDDLE,
  END,
}

impl CellPosition {
  fn offset(&self) -> u8 {
    match self {
      CellPosition::START => 0,
      CellPosition::MIDDLE => 3,
      CellPosition::END => 6
    }
  }

  fn for_coord(coord: u8) -> CellPosition {
    match coord {
      0..3 => CellPosition::START,
      3..6 => CellPosition::MIDDLE,
      6..9 => CellPosition::END,
      _ => panic!("Invalid coordinate!"),
    }
  }

  fn for_position(pos: &Position) -> (CellPosition, CellPosition) {
    (Self::for_coord(pos.row), Self::for_coord(pos.col))
  }
}

pub struct Group {
  positions: Vec<Position>,
}

impl Group {
  pub fn row(row: u8) -> Group {
    let mut positions = vec![];

    for col in 0..9 {
      positions.push(Position {
        row,
        col,
      });
    }

    Group {
      positions,
    }
  }

  pub fn col(col: u8) -> Group {
    let mut positions = vec![];

    for row in 0..9 {
      positions.push(Position {
        row,
        col,
      });
    }

    Group {
      positions,
    }
  }

  pub fn cell(row_pos: CellPosition, col_pos: CellPosition) -> Group {
    let mut positions = vec![];

    for row in 0..3 {
      for col in 0..3 {
        positions.push(Position {
          row: row_pos.offset() + row,
          col: col_pos.offset() + col,
        });
      }
    }

    Group {
      positions
    }
  }

  pub fn diag(top: bool) -> Group {
    let offset: i8 = if top { 0 } else { 8 };
    let dir: i8 = if top { 1 } else { -1 };
    let mut positions = vec![];

    for col in 0..9 {
      positions.push(Position {
        row: (offset + dir * col).try_into().unwrap(),
        col: col.try_into().unwrap()
      });
    }

    Group {
      positions
    }
  }
}

impl fmt::Display for Group {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f, "[{}]", 
      self.positions.iter().map(|p| format!("{}", p))
        .collect::<Vec<String>>().join(", ")
    )
  }
}

pub trait Constraint {
  fn apply(&self, value: u8, pos: &Position) -> HashMap<Position, Vec<u8>>;
}

pub trait GroupConstraint {
  fn get_group(pos: &Position) -> Group;
}

impl <T: GroupConstraint> Constraint for T {
  fn apply(&self, value: u8, pos: &Position) -> HashMap<Position, Vec<u8>> {
    let group = Self::get_group(pos);
    let mut map = HashMap::new();
    for group_pos in group.positions.iter() {
      if group_pos != pos {
        map.insert(Position::clone(group_pos), vec![value]);
      }
    }
    map
  }
}

struct RowConstraint {}
impl GroupConstraint for RowConstraint {
  fn get_group(pos: &Position) -> Group {
    Group::row(pos.row)
  }
}

struct ColumnConstraint {}
impl GroupConstraint for ColumnConstraint {
  fn get_group(pos: &Position) -> Group {
    Group::col(pos.col)
  }
}

struct CellConstraint {}
impl GroupConstraint for CellConstraint {
  fn get_group(pos: &Position) -> Group {
    let (row_pos, cell_pos) = CellPosition::for_position(pos);
    Group::cell(row_pos, cell_pos)
  }
}

struct DiagonalConstraint {}
impl GroupConstraint for DiagonalConstraint {
  fn get_group(pos: &Position) -> Group {
    let mut positions = vec![];

    if pos.row == pos.col {
      positions.append(&mut Group::diag(true).positions);
    }

    if pos.row == 8 - pos.col {
      positions.append(&mut Group::diag(false).positions);
    }

    Group { positions }
  }
}

struct PuzzleBuilder {
  groups: Vec<Group>,
  constraints: Vec<Box<dyn Constraint>>,
  clues: HashMap<Position, u8>,
}

impl PuzzleBuilder {
  pub fn new() -> PuzzleBuilder {
    PuzzleBuilder {
      groups: vec![],
      constraints: vec![],
      clues: HashMap::new(),
    }
  }

  pub fn add_group(&mut self, g: Group) -> &mut Self {
    self.groups.push(g);
    self
  }

  pub fn add_row_groups(&mut self) -> &mut Self {
    for row in 0..9 {
      self.add_group(Group::row(row));
    }
    self
  }

  pub fn add_col_groups(&mut self) -> &mut Self {
    for col in 0..9 {
      self.add_group(Group::col(col));
    }
    self
  }

  pub fn add_cell_groups(&mut self) -> &mut Self {
    let positions = vec![
      CellPosition::START,
      CellPosition::MIDDLE,
      CellPosition::END,
    ];
    for row_pos in positions.iter() {
      for col_pos in positions.iter() {
        self.add_group(Group::cell(*row_pos, *col_pos));
      }
    }
    self
  }

  pub fn add_diag_groups(&mut self) -> &mut Self {
    self.add_group(Group::diag(true));
    self.add_group(Group::diag(false));
    self
  }

  pub fn add_constraint(&mut self, c: Box<dyn Constraint>) -> &mut Self {
    self.constraints.push(c);
    self
  }

  pub fn add_clue(&mut self, pos: Position, value: u8) -> &mut Self {
    self.clues.insert(pos, value);
    self
  }

  pub fn add_clues(&mut self, clues: HashMap<Position, u8>) -> &mut Self {
    self.clues.extend(clues);
    self
  }

  pub fn build(self) -> Puzzle {
    Puzzle {
      groups: self.groups,
      constraints: self.constraints,
      clues: self.clues,
    }
  }
}

pub struct Puzzle {
  groups: Vec<Group>,
  constraints: Vec<Box<dyn Constraint>>,
  clues: HashMap<Position, u8>,
}

impl Puzzle {
  pub fn standard(clues: HashMap<Position, u8>) -> Puzzle {
    let mut builder = PuzzleBuilder::new();
    builder
      .add_row_groups()
      .add_col_groups()
      .add_cell_groups()
      .add_clues(clues);
    builder.build()
  }
}

pub struct Solution {
  pub puzzle: Puzzle,
  pub elements: HashMap<Position, u8>,
}