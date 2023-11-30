use nopolitics::benchmark::BenchmarkResult;
use nopolitics::{Error, Part, Solution, SolutionResult};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::PathBuf;

pub fn create_solution() -> Result<Solution, Error> {
    Ok(Solution::new(module_path!(), solve, None))
}

fn parse_line(line: &str) -> (Vec<char>, [char; 5], i32) {
    let split = line.rsplitn(2, '-').collect::<Vec<_>>();
    let checksum = split[0]
        .rsplit('[')
        .next()
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    let mut array = ['0'; 5];
    array.copy_from_slice(&checksum[0..5]);
    (
        split[1].chars().filter(|c| *c != '-').collect(),
        array,
        split[0].split('[').next().unwrap().parse::<i32>().unwrap(),
    )
}

#[allow(dead_code, unreachable_code, unused_variables, unused_mut)] // TODO
fn solve(path: &PathBuf, part: Part) -> Result<SolutionResult, Error> {
    let mut benchmark = BenchmarkResult::new_and_start(part);
    let mut lines = nopolitics::parse::file_to_lines(path)?
        .iter()
        .map(|l| parse_line(l))
        .collect::<Vec<_>>();
    benchmark.stop_parse_start_part();
    Ok(match part {
        Part::Part(1) => SolutionResult::part1(part1(&lines), benchmark.stop_part_and_owned()),
        Part::Part(2) => SolutionResult::part2(part2(&mut lines), benchmark.stop_part_and_owned()),
        Part::All => SolutionResult::part1and2(part1(&lines), part2(&mut lines), benchmark.stop_part_and_owned()),
        _ => {
            return Err(Error::Default);
        }
    })
}

fn shift(lines: &mut [(Vec<char>, [char; 5], i32)]) {
    lines.iter_mut().for_each(|(v, _, num)| {
        let shift = (*num % 26) as u32;
        v.iter_mut()
            .for_each(|c| *c = char::from_u32((((*c as u32) + shift - 97) % 26) + 97).unwrap());
    });
}

fn part1(lines: &[(Vec<char>, [char; 5], i32)]) -> String {
    lines
        .iter()
        .filter(|(v, check, _)| {
            let mut frequencies: Vec<(char, i32)> = v
                .iter()
                .copied()
                .fold(HashMap::new(), |mut map, val| {
                    map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
                    map
                })
                .into_iter()
                .collect::<Vec<_>>();
            frequencies.sort_by(|(c1, a1), (c2, a2)| match a2.cmp(a1) {
                Ordering::Equal => c1.cmp(c2),
                ord => ord,
            });
            frequencies
                .iter()
                .zip(check.iter())
                .filter(|((c, _), check_c)| *c == **check_c)
                .count()
                == 5
        })
        .fold(0, |prev, (_, _, num)| prev + num)
        .to_string()
}

fn part2(lines: &mut[(Vec<char>, [char; 5], i32)]) -> String {
    shift(lines);
    // lines.iter().for_each(|x| println!("{}", x.0.iter().collect::<String>()));
    lines.iter().find(|x| x.0.iter().collect::<String>().contains("north")).map(|x| x.2.to_string()).unwrap_or_default()
}
