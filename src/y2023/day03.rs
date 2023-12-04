use nopolitics::benchmark::BenchmarkResult;
use nopolitics::{Error, Part, Solution, SolutionResult};
use regex::{Regex};
use std::path::Path;

pub fn create_solution() -> Result<Solution, Error> {
    Ok(Solution::new(module_path!(), solve, None))
}

#[allow(dead_code, unreachable_code, unused_variables, unused_mut)] // TODO
fn solve(path: &Path, part: Part) -> Result<SolutionResult, Error> {
    let mut benchmark = BenchmarkResult::new_and_start(part);
    let lines = nopolitics::parse::file_to_lines(path)?;
    benchmark.stop_parse_start_part();
    Ok(match part {
        Part::Part(1) => SolutionResult::part1(part1(&lines), benchmark.stop_part_and_owned()),
        Part::Part(2) => SolutionResult::part2(part2(&lines), benchmark.stop_part_and_owned()),
        Part::All => SolutionResult::part1and2(part1(&lines), part2(&lines), benchmark.stop_part_and_owned()),
        _ => {
            return Err(Error::Default);
        }
    })
}

fn part2(lines: &[String]) -> String {
    let positions = lines
        .iter()
        .enumerate()
        .flat_map(|(i, s)|
            s.char_indices().filter_map(|(idx,c)| if c == '*' { Some((i,idx))} else { None} ).collect::<Vec<_>>()
        )
        .collect::<Vec<(usize, usize)>>();
    positions
        .iter()
        .fold(0, |mut prev, (r, c)| {
            let mut search = vec![*r];
            if *r > 0 {
                search.push(r - 1)
            }
            if *r < lines.len() - 1 {
                search.push(r + 1)
            }
            search.sort();
            let start = if *c == 0 { 0 } else { *c - 1 };
            let end = if *c == lines[0].len() - 1 { *c } else { *c + 1 };
            let numbers = search.iter().filter_map(|v| {
                let numeric_indices = lines[*v][start..end+1]
                    .char_indices()
                    .filter_map(|(idx, c)|
                        {
                            if c.is_numeric() { Some(idx+start) } else { None }
                        })
                    .collect::<Vec<_>>();
                if numeric_indices.is_empty() {
                    None
                } else {
                    Some(extract_numbers(&lines[*v], &numeric_indices))
                }
            }).flatten().collect::<Vec<_>>();
            if numbers.len() == 2 {
                prev += numbers.iter().product::<usize>()
            }
            prev
        })
        .to_string()
}

fn extract_numbers(str: &str, numeric_indices: &[usize]) -> Vec<usize> {
    if numeric_indices.is_empty() {
        return vec![];
    }
    if numeric_indices.len() == 1 || numeric_indices.windows(2).all(|w| w[0] == w[1] - 1) {
        let start = find_start(str, numeric_indices[0]);
        let end = find_end(str, *numeric_indices.last().unwrap());
        vec![str[start..end + 1].parse::<usize>().unwrap()]
    } else {
        vec![
            str[find_start(str, numeric_indices[0])..(numeric_indices[0] + 1)]
                .parse::<usize>()
                .unwrap(),
            str[numeric_indices[1]..find_end(str, numeric_indices[1]) + 1]
                .parse::<usize>()
                .unwrap(),
        ]
    }
}

fn find_start(str: &str, init: usize) -> usize {
    for i in (0..init+1).rev() {
        if !str.chars().nth(i).unwrap().is_numeric() {
            return i+1;
        }
    }
    0
}

fn find_end(str: &str, init: usize) -> usize {
    for i in init..str.len() {
        if !str.chars().nth(i).unwrap().is_numeric() {
            return i-1;
        }
    }
    str.len() - 1
}

fn part1(lines: &[String]) -> String {
    let re = Regex::new(r"[0-9]+").unwrap();
    lines
        .iter()
        .enumerate()
        .fold(0, |mut prev, (i, next)| {
            let captures = re.captures_iter(next);
            for c in captures {
                let v = c.get(0).unwrap();
                let start = v.start();
                let end = start + v.len() - 1;
                let start = start.saturating_sub(1);
                let end = if end == next.len() - 1 { end } else { end + 1 };
                let mut searches = vec![i];
                if i > 0 {
                    searches.push(i - 1)
                }
                if i < lines.len() - 1 {
                    searches.push(i + 1)
                }
                if searches.iter().any(|j| {
                    lines[*j][start..end + 1]
                        .chars()
                        .any(|c| !c.is_alphanumeric() && c != '.')
                }) {
                    prev += v.as_str().parse::<u32>().unwrap();
                }
            }
            prev
        })
        .to_string()
}
