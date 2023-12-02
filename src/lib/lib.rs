#![warn(
clippy::all,
clippy::pedantic,
clippy::nursery,
clippy::cargo,
)]

#![allow(clippy::implicit_return)]

use crate::benchmark::BenchmarkResult;
use crate::parse::ParseError;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use std::num::ParseIntError;
use std::path::{Path, PathBuf};

pub mod benchmark;
pub mod parse;
pub mod types;
pub use types::*;

#[derive(Default, Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Part {
    #[default]
    All,
    Part(usize),
}

impl Display for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "All"),
            Self::Part(i) => write!(f, "{}", i),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Default,
    Parse(ParseError),
}

impl From<ParseError> for Error {
    fn from(value: ParseError) -> Self {
        Self::Parse(value)
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        <ParseIntError as Into<ParseError>>::into(value).into()
    }
}

#[derive(Debug)]
pub struct SolutionResult {
    pub results: HashMap<usize, String>,
    pub benchmark: BenchmarkResult,
}

impl SolutionResult {
    pub fn part1(result: String, benchmark: BenchmarkResult) -> Self {
        Self {
            results: HashMap::from([(1, result)]),
            benchmark,
        }
    }

    pub fn part2(result: String, benchmark: BenchmarkResult) -> Self {
        Self {
            results: HashMap::from([(2, result)]),
            benchmark,
        }
    }

    pub fn part1and2(part1: String, part2: String, benchmark: BenchmarkResult) -> Self {
        Self {
            results: HashMap::from([(1, part1), (2, part2)]),
            benchmark,
        }
    }
}

type Solve = dyn Fn(&Path, Part) -> Result<SolutionResult, Error>;

pub struct Solution {
    path: PathBuf,
    solve: Box<Solve>,
    solve_parallel: Option<Box<Solve>>,
}

impl Solution {
    pub fn new<S: Fn(&Path, Part) -> Result<SolutionResult, Error> + 'static>(
        path: &str,
        solve: S,
        solve_parallel: Option<S>,
    ) -> Self {
        let path_elements = path
            .split("::")
            .collect::<Vec<&str>>()
            .iter()
            .rev()
            .take(2)
            .rev()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let path = Path::new(&path_elements[0]).join(&path_elements[1]);
        Self {
            path,
            solve: Box::new(solve),
            solve_parallel: solve_parallel.map(|f| {
                Box::new(f) as Box<dyn Fn(&Path, Part) -> Result<SolutionResult, Error>>
            }),
        }
    }

    pub fn run_main(&self, parallel: bool, part: Part) -> Result<SolutionResult, Error> {
        self.run(parallel, "main.txt", part)
    }

    pub fn run(&self, parallel: bool, file: &str, part: Part) -> Result<SolutionResult, Error> {
        if parallel {
            self.solve_parallel.as_ref().ok_or(Error::Default)?(&self.path(file), part)
        } else {
            (self.solve)(&self.path(file), part)
        }
    }

    fn path(&self, file: &str) -> PathBuf {
        Path::new("input").join(&self.path).join(file)
    }
}

pub struct SolutionTester {
    solution: Solution,
}

impl SolutionTester {
    pub fn new(solution: Solution) -> Self {
        Self { solution }
    }

    pub fn test_part1_single_thread(&self) {
        let r = self
            .solution
            .run(false, "example.txt", Part::Part(1))
            .unwrap();
        assert_eq!(
            r.results[&1],
            read_to_string(self.solution.path("example_result_part1.txt")).unwrap()
        );
    }

    pub fn test_part1_parallel(&self) {
        if self.solution.solve_parallel.is_none() {
            return;
        }
        let r = self
            .solution
            .run(true, "example.txt", Part::Part(1))
            .unwrap();
        assert_eq!(
            r.results[&1],
            read_to_string(self.solution.path("example_result_part1.txt")).unwrap()
        );
    }

    pub fn test_part2_single_thread(&self) {
        let r = self
            .solution
            .run(false, self.part2path().as_str(), Part::Part(2))
            .unwrap();
        assert_eq!(
            r.results[&2],
            read_to_string(self.solution.path("example_result_part2.txt")).unwrap()
        );
    }

    fn part2path(&self) -> String {
        if self.solution.path("example2.txt").exists() {
            "example2.txt".to_string()
        } else {
            "example.txt".to_string()
        }
    }

    pub fn test_part2_parallel(&self) {
        if self.solution.solve_parallel.is_none() {
            return;
        }
        let r = self
            .solution
            .run(true, self.part2path().as_str(), Part::Part(2))
            .unwrap();
        assert_eq!(
            r.results[&2],
            read_to_string(self.solution.path("example_result_part2.txt")).unwrap()
        );
    }

    pub fn test_all(&self) {
        self.test_part1_single_thread();
        self.test_part1_parallel();
        self.test_part2_single_thread();
        self.test_part2_parallel();
    }
}
