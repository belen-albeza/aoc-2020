use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day13)]
pub fn parse_input(input: &str) -> (u64, Vec<Option<u64>>) {
  let mut input_iter = input.lines();

  let departure = input_iter.next().unwrap().parse().unwrap();
  let buses: Vec<Option<u64>> = input_iter
    .next()
    .unwrap()
    .split(',')
    .map(|x| x.parse().ok())
    .collect();

  (departure, buses)
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &(u64, Vec<Option<u64>>)) -> u64 {
  let departure = input.0;
  let buses: Vec<u64> = input
    .1
    .iter()
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .collect();

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

fn find_common_timestamp_in_list(buses: Vec<(u64, u64)>) -> Result<u64, String> {
  if buses.len() == 0 {
    return Err("Empty bus list".to_string());
  }
  if buses.len() == 1 {
    return Ok(buses[0].1);
  }

  let mut t = 0;
  let mut coeff = 1;
  for i in 1..buses.len() {
    let (curr_gap, curr_bus_id) = buses[i];
    let (_, prev_bus_id) = buses[i - 1];

    // NOTE: this should be minimum common multiple of every ID up to this point,
    // but since all the numbers in the input are primes, just a plain
    // multiplication works
    coeff = coeff * prev_bus_id;

    loop {
      if (t + curr_gap) % curr_bus_id == 0 {
        break;
      }
      t += coeff;
    }
  }

  Ok(t)
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &(u64, Vec<Option<u64>>)) -> Result<u64, String> {
  let buses = &input.1;

  find_common_timestamp_in_list(
    buses
      .iter()
      .enumerate()
      .filter(|(_, bus)| bus.is_some())
      .map(|(gap, bus)| (gap as u64, bus.unwrap()))
      .collect(),
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_input() {
    let input = "939\n7,13,x,x,59,x,31,19";

    assert_eq!(
      parse_input(input),
      (
        939,
        vec![
          Some(7),
          Some(13),
          None,
          None,
          Some(59),
          None,
          Some(31),
          Some(19)
        ]
      )
    );
  }

  #[test]
  fn test_solve_part1() {
    let departure = 939;
    let buses = vec![
      Some(7),
      Some(13),
      None,
      None,
      Some(59),
      None,
      Some(31),
      Some(19),
    ];

    assert_eq!(solve_part1(&(departure, buses)), 295);
  }

  #[test]
  fn test_solve_part2() {
    let buses = vec![Some(7)];
    assert_eq!(solve_part2(&(0, buses)), Ok(7));

    let buses = vec![Some(7), Some(13)];
    assert_eq!(solve_part2(&(0, buses)), Ok(77));

    let buses = vec![Some(7), Some(13), None, None, Some(59)];
    assert_eq!(solve_part2(&(0, buses)), Ok(350));

    let buses = vec![Some(7), Some(13), None, None, Some(59), None, Some(31)];
    assert_eq!(solve_part2(&(0, buses)), Ok(70147));

    let buses = vec![Some(17), None, Some(13), Some(19)];
    assert_eq!(solve_part2(&(0, buses)), Ok(3417));

    let buses = vec![Some(67), Some(7), Some(59), Some(61)];
    assert_eq!(solve_part2(&(0, buses)), Ok(754018));

    let buses = vec![Some(1789), Some(37), Some(47), Some(1889)];
    assert_eq!(solve_part2(&(0, buses)), Ok(1202161486));
  }
}
