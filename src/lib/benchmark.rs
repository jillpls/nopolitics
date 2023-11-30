use crate::Part;
use std::fmt::{Display, Formatter};
use std::iter::Sum;
use std::ops::Add;
use std::time::{Duration, Instant};

#[derive(Default, Debug, Copy, Clone)]
pub struct BenchmarkResult {
    parse: Option<Duration>,
    part: Option<Duration>,
    parse_start: Option<Instant>,
    part_start: Option<Instant>,
    part_id: Part,
}

impl Add<BenchmarkResult> for BenchmarkResult {
    type Output = BenchmarkResult;

    fn add(self, rhs: BenchmarkResult) -> Self::Output {
        self + &rhs
    }
}

impl Add<&BenchmarkResult> for BenchmarkResult {
    type Output = BenchmarkResult;

    fn add(self, rhs: &BenchmarkResult) -> Self::Output {
        let part = Some(self.part.unwrap_or_default() + rhs.part.unwrap_or_default());
        let parse = Some(self.parse.unwrap_or_default() + rhs.parse.unwrap_or_default());
        Self {
            parse,
            part,
            ..Self::default()
        }
    }
}

impl<'a> Sum<&'a BenchmarkResult> for BenchmarkResult {
    fn sum<I: Iterator<Item = &'a BenchmarkResult>>(iter: I) -> Self {
        iter.fold(BenchmarkResult::default(), |prev, next| prev + next)
    }
}

impl Sum for BenchmarkResult {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(BenchmarkResult::default(), |prev, next| prev + next)
    }
}

impl Display for BenchmarkResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Parse: {} | Part: {} | Combined: {}",
            format_duration(self.parse.unwrap_or_default()),
            format_duration(self.part.unwrap_or_default()),
            format_duration(self.combined())
        )
    }
}

impl BenchmarkResult {
    pub fn part(&self) -> Part {
        self.part_id
    }
    pub fn new(part: Part) -> Self {
        Self {
            parse: None,
            part: None,
            parse_start: None,
            part_start: None,
            part_id: part,
        }
    }

    pub fn start_parse(&mut self) {
        self.parse_start = Some(Instant::now());
    }

    pub fn new_and_start(part: Part) -> Self {
        let mut r = Self::new(part);
        r.start_parse();
        r
    }

    pub fn stop_parse(&mut self) {
        if let Some(before) = self.parse_start {
            self.parse = Some(Instant::now() - before);
        }
    }

    pub fn stop_parse_start_part(&mut self) {
        self.stop_parse();
        self.start_part();
    }

    pub fn start_part(&mut self) {
        self.part_start = Some(Instant::now());
    }
    pub fn stop_part(&mut self) {
        if let Some(before) = self.part_start {
            self.part = Some(Instant::now() - before);
        }
    }

    pub fn stop_part_and_owned(mut self) -> Self {
        self.stop_part();
        self
    }

    pub fn combined(&self) -> Duration {
        self.parse.unwrap_or(Duration::default()) + self.part.unwrap_or(Duration::default())
    }
}

pub fn format_duration(duration: Duration) -> String {
    let minutes = format!("{:02}", duration.as_secs() / 60);
    let seconds = format!("{:02}", duration.as_secs() % 60);
    let millis = format!("{:03}", duration.as_millis() % 1000);
    let micros = format!("{:03}", duration.as_micros() % 1000);
    let nanos = format!("{:03}", duration.as_nanos() % 1000);
    format!("[{}:{}:{}:{}:{}]", minutes, seconds, millis, micros, nanos)
}
