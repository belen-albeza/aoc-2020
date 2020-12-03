use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Cell {
  Empty,
  Tree,
}

pub struct Map {
  cells: Vec<Cell>,
  width: usize,
  height: usize,
}

impl Map {
  pub fn new(input: &str) -> Self {
    let mut cells: Vec<Cell> = vec![];
    let mut height: usize = 1;
    let mut width: usize = 0;

    for character in input.chars() {
      width += 1;

      match character {
        '.' => cells.push(Cell::Empty),
        '#' => cells.push(Cell::Tree),
        '\n' => {
          width = 0;
          height += 1;
        }
        _ => {}
      }
    }

    Map {
      width: width,
      height: height,
      cells: cells,
    }
  }

  pub fn get_cell(&self, x: usize, y: usize) -> Option<Cell> {
    let index: usize = x % self.width + y * self.width;
    self.cells.get(index).cloned()
  }
}

#[derive(Clone, Copy)]
pub struct SlopeIterator {
  slope: (usize, usize),
  current: (usize, usize),
  limit: (usize, usize),
}

impl SlopeIterator {
  fn new(slope: (usize, usize), map: &Map) -> Self {
    SlopeIterator {
      slope: slope,
      current: (0, 0),
      limit: (usize::MAX, map.height - 1),
    }
  }
}

impl Iterator for SlopeIterator {
  type Item = (usize, usize);

  fn next(&mut self) -> Option<(usize, usize)> {
    if self.current.0 > self.limit.0 || self.current.1 > self.limit.1 {
      return None;
    }

    let res = self.current;
    self.current.0 += self.slope.0;
    self.current.1 += self.slope.1;

    return Some(res);
  }
}

fn count_trees_in_map(map: &Map, slope: (usize, usize)) -> u64 {
  let iterator = SlopeIterator::new(slope, map);

  iterator.fold(0, |total, (x, y)| {
    let inc = match map.get_cell(x, y) {
      Some(Cell::Tree) => 1,
      _ => 0,
    };

    total + inc
  })
}

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> Map {
  Map::new(input)
}

#[aoc(day3, part1)]
pub fn solve_part1(map: &Map) -> u64 {
  const SLOPE: (usize, usize) = (3, 1);
  count_trees_in_map(map, SLOPE)
}

#[aoc(day3, part2)]
pub fn solve_part2(map: &Map) -> u64 {
  const SLOPES: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

  SLOPES
    .iter()
    .fold(1, |total, slope| total * count_trees_in_map(map, *slope))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_map_constructor() {
    let input = ".#..\n..#.\n...#";
    let map = Map::new(input);

    assert_eq!(map.width, 4);
    assert_eq!(map.height, 3);
    assert_eq!(map.cells.len(), 12);
    assert_eq!(map.cells[0], Cell::Empty);
    assert_eq!(map.cells[1], Cell::Tree);
    assert_eq!(map.cells[11], Cell::Tree);
  }

  #[test]
  fn test_map_get_cell() {
    let input = "##..\n..#.\n...#";
    let map = Map::new(input);

    // test getting direct coordinates
    assert_eq!(map.get_cell(0, 0), Some(Cell::Tree));
    assert_eq!(map.get_cell(1, 0), Some(Cell::Tree));
    assert_eq!(map.get_cell(2, 2), Some(Cell::Empty));
    // test wrapping in X axis
    assert_eq!(map.get_cell(0, 0), map.get_cell(4, 0));
    assert_eq!(map.get_cell(2, 0), map.get_cell(6, 0));
    // test out of bounds coords
    assert_eq!(map.get_cell(0, 3), None);
  }

  #[test]
  fn test_solve_part1() {
    let input = "..##.......\n\
                 #...#...#..\n\
                 .#....#..#.\n\
                 ..#.#...#.#\n\
                 .#...##..#.\n\
                 ..#.##.....\n\
                 .#.#.#....#\n\
                 .#........#\n\
                 #.##...#...\n\
                 #...##....#\n\
                 .#..#...#.#";
    let map = Map::new(input);
    assert_eq!(solve_part1(&map), 7);
  }

  #[test]
  fn test_solve_part2() {
    let input = "..##.......\n\
                 #...#...#..\n\
                 .#....#..#.\n\
                 ..#.#...#.#\n\
                 .#...##..#.\n\
                 ..#.##.....\n\
                 .#.#.#....#\n\
                 .#........#\n\
                 #.##...#...\n\
                 #...##....#\n\
                 .#..#...#.#";
    let map = Map::new(input);
    assert_eq!(solve_part2(&map), 336);
  }
}
