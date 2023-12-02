use nopolitics::benchmark::BenchmarkResult;
use nopolitics::{Error, Part, Solution, SolutionResult};
use std::path::Path;

pub fn create_solution() -> Result<Solution, Error> {
    Ok(Solution::new(module_path!(), solve, None))
}

fn solve(path: &Path, part: Part) -> Result<SolutionResult, Error> {
    let mut benchmark = BenchmarkResult::new_and_start(part);
    let strategy = nopolitics::parse::file_to_lines(path)?
        .iter()
        .map(|s| {
            let chars = s.chars().collect::<Vec<_>>();
            (chars[0], chars[2])
        })
        .collect::<Vec<(char, char)>>();
    benchmark.stop_parse_start_part();
    Ok(match part {
        Part::Part(1) => SolutionResult::part1(part1(&strategy), benchmark.stop_part_and_owned()),
        Part::Part(2) => SolutionResult::part2(part2(&strategy), benchmark.stop_part_and_owned()),
        Part::All => SolutionResult::part1and2(
            part1(&strategy),
            part2(&strategy),
            benchmark.stop_part_and_owned(),
        ),
        _ => {
            return Err(Error::Default);
        }
    })
}

fn part1(strategy: &[(char, char)]) -> String {
    strategy
        .iter()
        .map(|(o, p)| {
            let (opponent, player) = a_x_to_1(*o, *p);
            calc_score(opponent, player)
        })
        .sum::<i32>()
        .to_string()
}

fn part2(strategy: &[(char, char)]) -> String {
    strategy
        .iter()
        .map(|(o, r)| calc_score(*o as i32 - 64, choose_strategy(*o, *r)))
        .sum::<i32>()
        .to_string()
}

fn a_x_to_1(a: char, x: char) -> (i32, i32) {
    (a as i32 - 64, x as i32 - 87)
}

fn calc_score(opponent: i32, player: i32) -> i32 {
    player
        + match (opponent, player) {
            (1, 2) | (2, 3) | (3, 1) => 6,
            (1, 3) | (2, 1) | (3, 2) => 0,
            _ => 3,
        }
}

fn choose_strategy(opponent: char, result: char) -> i32 {
    let (opponent, result) = a_x_to_1(opponent, result);
    match result {
        1 => {
            let r = opponent - 1;
            if r == 0 {
                3
            } else {
                1
            }
        }
        2 => opponent,
        3 => (opponent % 3) + 1,
        _ => 0,
    }
}
