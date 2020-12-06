use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::collections::HashSet;

type Group = HashSet<char>;

#[aoc_generator(day6, part1)]
pub fn parse_input_part1(input: &str) -> Vec<Group> {
  input
    .split("\n\n")
    .map(|raw_group| {
      raw_group
        .lines()
        .collect::<Vec<&str>>()
        .concat()
        .chars()
        .collect()
    })
    .collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(groups: &[Group]) -> usize {
  groups.iter().map(|x| x.len()).sum()
}

#[aoc_generator(day6, part2)]
pub fn parse_input_part2(input: &str) -> Vec<Group> {
  input
    .split("\n\n")
    .map(|raw_group| {
      let answers: Vec<Group> = raw_group
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|person_answers| person_answers.chars().collect::<Group>())
        .collect();

      answers.iter().fold(answers[0].clone(), |result, partial| {
        result.intersection(&partial).copied().collect::<Group>()
      })
    })
    .collect()
}

#[aoc(day6, part2)]
pub fn solve_part2(groups: &[Group]) -> usize {
  groups.iter().map(|x| x.len()).sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_input_part1() {
    let input = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
    assert_eq!(
      parse_input_part1(input),
      vec![
        ['a', 'b', 'c'].iter().cloned().collect::<Group>(),
        ['a', 'b', 'c'].iter().cloned().collect::<Group>(),
        ['a', 'b', 'c'].iter().cloned().collect::<Group>(),
        ['a'].iter().cloned().collect::<Group>(),
        ['b'].iter().cloned().collect::<Group>(),
      ]
    );
  }

  #[test]
  fn test_solve_part1() {
    let groups = vec![
      ['a', 'b', 'c'].iter().cloned().collect::<Group>(),
      ['a', 'b', 'c'].iter().cloned().collect::<Group>(),
      ['a', 'b', 'c'].iter().cloned().collect::<Group>(),
      ['a'].iter().cloned().collect::<Group>(),
      ['b'].iter().cloned().collect::<Group>(),
    ];

    assert_eq!(solve_part1(&groups), 11);
  }

  #[test]
  fn test_parse_input_part2() {
    let input = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
    assert_eq!(
      parse_input_part2(input),
      vec![
        ['a', 'b', 'c'].iter().cloned().collect::<Group>(),
        [].iter().cloned().collect::<Group>(),
        ['a'].iter().cloned().collect::<Group>(),
        ['a'].iter().cloned().collect::<Group>(),
        ['b'].iter().cloned().collect::<Group>(),
      ]
    );
  }
}
