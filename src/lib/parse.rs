use crate::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::path::Path;

#[derive(Debug)]
pub enum ParseError {
    IOError(std::io::Error),
    ParseIntError(ParseIntError),
}

impl From<std::io::Error> for ParseError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}
pub fn file_to_lines<P: AsRef<Path>>(path: P) -> Result<Vec<String>, ParseError> {
    let reader = BufReader::new(File::open(path)?);
    reader
        .lines()
        .collect::<Result<Vec<String>, std::io::Error>>()
        .map_err(ParseError::from)
}

pub fn split_clustered<T, F>(vec: &[T], predicate: F) -> Vec<&[T]>
where
    F: FnMut(&T) -> bool,
{
    vec.split(predicate).collect::<Vec<&[T]>>()
}

pub fn split_on_empty_lines(vec: &[String]) -> Vec<&[String]> {
    split_clustered(vec, |s| s.trim().is_empty())
}

pub fn to_coordinate(str: &str, sep: char) -> Result<(i32, i32), Error> {
    let r: Vec<i32> = str
        .split(sep)
        .map(|x| x.parse::<i32>().map_err(|e| e.into()))
        .collect::<Result<Vec<_>, Error>>()?;
    vec_to_tuple(&r)
}

pub fn vec_to_tuple<T: Clone>(vec: &[T]) -> Result<(T, T), Error> {
    Ok((
        vec.get(0).ok_or(Error::Default)?.clone(),
        vec.get(1).ok_or(Error::Default)?.clone(),
    ))
}

pub fn parse_to_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid() {
        let input = "30373
25512
65332
33549
35390";
        let grid = parse_to_grid(input);
        assert_eq!(grid[0], "30373".chars().collect::<Vec<char>>())
    }
}
