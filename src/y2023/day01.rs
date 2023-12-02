use nopolitics::benchmark::BenchmarkResult;
use nopolitics::{Error, Part, Solution, SolutionResult};
use std::path::Path;

pub fn create_solution() -> Result<Solution, Error> {
    Ok(Solution::new(module_path!(), solve, None))
}

#[allow(dead_code, unreachable_code, unused_variables, unused_mut)] // TODO
fn solve(path: &Path, part: Part) -> Result<SolutionResult, Error> {
    let mut benchmark = BenchmarkResult::new_and_start(part);
    let input = nopolitics::parse::file_to_lines(path)?;
    benchmark.stop_parse_start_part();
    Ok(match part {
        Part::Part(1) => SolutionResult::part1(part1(&input), benchmark.stop_part_and_owned()),
        Part::Part(2) => SolutionResult::part2(part2(&input), benchmark.stop_part_and_owned()),
        Part::All => SolutionResult::part1and2(
            part1(&input),
            part2(&input),
            benchmark.stop_part_and_owned(),
        ),
        _ => {
            return Err(Error::Default);
        }
    })
}

fn part1(input: &[String]) -> String {
    input
        .iter()
        .fold(0, |prev, next| {
            let c = next.chars().filter(|x| x.is_numeric()).collect::<Vec<_>>();
            prev + c.first().and_then(|x| x.to_digit(10)).unwrap_or_default() * 10
                + c.last().and_then(|x| x.to_digit(10)).unwrap_or_default()
        })
        .to_string()
}

fn part2(input: &[String]) -> String {
    input
        .iter()
        .fold(0, |prev, next| {
            let first = get_first_number(next.chars(), false);
            let second = get_first_number(next.chars().rev(), true);
            prev + first * 10 + second
        })
        .to_string()
}

fn get_first_number<I: Iterator<Item = char>>(it: I, rev: bool) -> usize {
    let mut indices: [usize; 9] = [0; 9];
    for c in it {
        if c.is_numeric() {
            if let Some(v) = c.to_digit(10) {
                return v as usize;
            }
        }
        for (i, n) in NUMBERS.iter().enumerate() {
            let check_index = if rev {
                n.len() - indices[i] - 1
            } else {
                indices[i]
            };
            if c == n.chars().nth(check_index).unwrap_or_default() {
                indices[i] += 1;
                if indices[i] == n.len() {
                    return i + 1;
                }
            }
        }
    }
    0
}

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
