use aoc_runner_derive::aoc;
use std::cmp;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
  Seat(bool), // Seat(is_occupied)
  Floor,
}

pub struct Grid {
  cells: Vec<Cell>,
  previous_cells: Vec<Cell>,
  width: usize,
  height: usize,
}

impl Grid {
  pub fn new(map: &str) -> Self {
    let mut width = 0;
    let cells: Vec<Cell> = map
      .lines()
      .map(|line| {
        width = line.len();
        line
          .chars()
          .map(|c| match c {
            'L' => Cell::Seat(false),
            '.' => Cell::Floor,
            '#' => Cell::Seat(true),
            _ => {
              println!("wrong char {:?}", c);
              unreachable!();
            }
          })
          .collect::<Vec<Cell>>()
      })
      .collect::<Vec<Vec<Cell>>>()
      .concat();

    let height = if width > 0 { cells.len() / width } else { 0 };

    Grid {
      cells: cells,
      previous_cells: vec![],
      width: width,
      height: height,
    }
  }

  fn get_cell_at(&self, x: i64, y: i64) -> Option<Cell> {
    if x >= self.width as i64 || y >= self.height as i64 || x < 0 || y < 0 {
      return None;
    }

    let cell = self.cells[(x + y * self.width as i64) as usize];
    Some(cell)
  }

  fn is_neighbor_on_dir(&self, cell_index: usize, distance: usize, direction: (i64, i64)) -> bool {
    let cell_x = (cell_index % self.width) as i64;
    let cell_y = (cell_index / self.width) as i64;

    for i in 1..=(distance as i64) {
      let x = cell_x + i * direction.0;
      let y = cell_y + i * direction.1;
      match self.get_cell_at(x, y) {
        Some(Cell::Seat(true)) => {
          return true;
        }
        None | Some(Cell::Seat(false)) => {
          return false;
        }
        _ => {}
      }
    }

    false
  }

  pub fn is_estable(&self) -> bool {
    self.previous_cells == self.cells
  }

  pub fn get_occupied_amount(&self) -> usize {
    let occupied: Vec<&Cell> = self
      .cells
      .iter()
      .filter(|&cell| *cell == Cell::Seat(true))
      .collect();
    occupied.len()
  }

  pub fn step(&mut self, distance: usize, tolerance: usize) {
    const NEIGHBOR_COORDS: [(i64, i64); 8] = [
      (-1, -1),
      (0, -1),
      (1, -1),
      (-1, 0),
      (1, 0),
      (-1, 1),
      (0, 1),
      (1, 1),
    ];

    self.previous_cells = self.cells.clone();
    self.cells = self
      .cells
      .iter()
      .enumerate()
      .map(|(index, cell)| {
        let occupied: Vec<bool> = NEIGHBOR_COORDS
          .iter()
          .map(|dir| self.is_neighbor_on_dir(index, distance, *dir))
          .filter(|x| *x)
          .collect();

        if *cell == Cell::Seat(false) && occupied.len() == 0 {
          Cell::Seat(true)
        } else if *cell == Cell::Seat(true) && occupied.len() >= tolerance {
          Cell::Seat(false)
        } else {
          *cell
        }
      })
      .collect();
  }
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &str) -> u64 {
  let mut grid = Grid::new(input);

  while !grid.is_estable() {
    grid.step(1, 4);
  }

  grid.get_occupied_amount() as u64
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &str) -> u64 {
  let mut grid = Grid::new(input);

  let distance = cmp::max(grid.width, grid.height);

  while !grid.is_estable() {
    grid.step(distance, 5);
  }

  grid.get_occupied_amount() as u64
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_grid_constructor() {
    let input = ".L.\n#L.\n...\nL#.";
    let grid = Grid::new(input);

    assert_eq!(grid.width, 3);
    assert_eq!(grid.height, 4);
    assert_eq!(
      grid.cells,
      vec![
        Cell::Floor,
        Cell::Seat(false),
        Cell::Floor,
        Cell::Seat(true),
        Cell::Seat(false),
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
        Cell::Floor,
        Cell::Seat(false),
        Cell::Seat(true),
        Cell::Floor
      ]
    );
  }

  #[test]
  fn test_get_cell_at() {
    let input = "L.#";
    let grid = Grid::new(input);

    assert_eq!(grid.get_cell_at(-1, 0), None);
    assert_eq!(grid.get_cell_at(4, 0), None);
    assert_eq!(grid.get_cell_at(0, -1), None);
    assert_eq!(grid.get_cell_at(0, 2), None);
    assert_eq!(grid.get_cell_at(0, 0), Some(Cell::Seat(false)));
    assert_eq!(grid.get_cell_at(1, 0), Some(Cell::Floor));
    assert_eq!(grid.get_cell_at(2, 0), Some(Cell::Seat(true)));
  }

  #[test]
  fn test_solve_part1() {
    let input: String = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
"
    .chars()
    .filter(|&c| c != ' ')
    .collect();
    assert_eq!(solve_part1(&input), 37);
  }

  #[test]
  fn test_solve_part2() {
    let input: String = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
"
    .chars()
    .filter(|&c| c != ' ')
    .collect();
    assert_eq!(solve_part2(&input), 26);
  }
}
