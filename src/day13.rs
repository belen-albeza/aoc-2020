use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day13)]
pub fn parse_input(input: &str) -> (u64, Vec<u64>) {
  let mut input_iter = input.lines();

  let departure = input_iter.next().unwrap().parse().unwrap();
  let buses = input_iter
    .next()
    .unwrap()
    .split(',')
    .map(|x| x.parse())
    .filter(|x| x.is_ok())
    .map(|x| x.unwrap())
    .collect();

  (departure, buses)
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &(u64, Vec<u64>)) -> u64 {
  let departure = input.0;
  let buses = &input.1;

  let earliest_departures = buses.iter().map(|&bus| {
    let timestamp = (departure as f64 / bus as f64).ceil() as u64 * bus;
    (bus, timestamp)
  });

  let (bus, timestamp) = earliest_departures
    .min_by(|a, b| a.1.cmp(&b.1))
    .expect("No bus was found");
  let waiting_time = timestamp - departure;

  bus * waiting_time
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_input() {
    let input = "939\n7,13,x,x,59,x,31,19";

    assert_eq!(parse_input(input), (939, vec![7, 13, 59, 31, 19]));
  }

  #[test]
  fn test_solve_part1() {
    let input = (939, vec![7, 13, 59, 31, 19]);

    assert_eq!(solve_part1(&input), 295);
  }
}
