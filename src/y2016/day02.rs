use nopolitics::benchmark::BenchmarkResult;
use nopolitics::{Error, Part, Point2, Solution, SolutionResult};
use std::collections::HashMap;
use std::path::PathBuf;

pub fn create_solution() -> Result<Solution, Error> {
    Ok(Solution::new(module_path!(), solve, None))
}

#[allow(dead_code, unreachable_code, unused_variables, unused_mut)] // TODO
fn solve(path: &PathBuf, part: Part) -> Result<SolutionResult, Error> {
    let mut benchmark = BenchmarkResult::new_and_start(part);
    let instructions: Vec<Vec<Point2<i32>>> = nopolitics::parse::file_to_lines(path)?
        .iter()
        .map(|x| {
            x.trim()
                .chars()
                .map(|c| match c {
                    'U' => Point2::new(1, 0),
                    'D' => Point2::new(-1, 0),
                    'R' => Point2::new(0, 1),
                    'L' => Point2::new(0, -1),
                    _ => Point2::new(0, 0),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    benchmark.stop_parse_start_part();
    Ok(match part {
        Part::Part(1) => {
            SolutionResult::part1(part1(&instructions), benchmark.stop_part_and_owned())
        }
        Part::Part(2) => {
            SolutionResult::part2(part2(&instructions), benchmark.stop_part_and_owned())
        }
        Part::All => SolutionResult::part1and2(
            part1(&instructions),
            part2(&instructions),
            benchmark.stop_part_and_owned(),
        ),
        _ => {
            return Err(Error::Default);
        }
    })
}

fn part1(instructions: &[Vec<Point2<i32>>]) -> String {
    let mut prev_result = Point2::new(0, 0);
    instructions
        .iter()
        .map(|v| {
            prev_result = v.iter().fold(prev_result.clone(), |prev, add| {
                let new = prev + add.clone();
                Point2::new(new.x.signum(), new.y.signum())
            });
            prev_result
        })
        .map(|p| {
            match (p.x, p.y) {
                (1, -1) => 1,
                (1, 0) => 2,
                (1, 1) => 3,
                (0, -1) => 4,
                (0, 0) => 5,
                (0, 1) => 6,
                (-1, -1) => 7,
                (-1, 0) => 8,
                (-1, 1) => 9,
                _ => 3000,
            }
            .to_string()
        })
        .collect::<Vec<_>>()
        .join("")
}
fn part2(instructions: &[Vec<Point2<i32>>]) -> String {
    let mut prev_result = Point2::new(0, -2);
    let map = HashMap::from([
        ([2, 0], '1'),
        ([1, -1], '2'),
        ([1, 0], '3'),
        ([1, 1], '4'),
        ([0, -2], '5'),
        ([0, -1], '6'),
        ([0, 0], '7'),
        ([0, 1], '8'),
        ([0, 2], '9'),
        ([-1, -1], 'A'),
        ([-1, 0], 'B'),
        ([-1, 1], 'C'),
        ([-2, 0], 'D'),
    ]);
    instructions
        .iter()
        .map(|v| {
            prev_result = v.iter().fold(prev_result.clone(), |prev, add| {
                let new = prev + add.clone();
                let next = if map.contains_key(&new.to_array()) {
                    new
                } else {
                    prev
                };
                next
            });
            prev_result
        })
        .map(|p| map[&p.to_array()])
        .collect::<String>()
}
