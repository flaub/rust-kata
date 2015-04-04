use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Filter;
use std::str::FromStr;
use std::str::Split;

struct Words<'a> {
	inner: Filter<Split<'a, fn(char) -> bool>, fn(&&str) -> bool>,
}

impl<'a> Iterator for Words<'a> {
	type Item = &'a str;

	fn next(&mut self) -> Option<&'a str> { self.inner.next() }
}

fn words(s: &str) -> Words {
	fn is_not_empty(s: &&str) -> bool { !s.is_empty() }
	let is_not_empty: fn(&&str) -> bool = is_not_empty;

	fn is_whitespace(c: char) -> bool { c.is_whitespace() }
	let is_whitespace: fn(char) -> bool = is_whitespace;

	Words { inner: s.split(is_whitespace).filter(is_not_empty) }
}

fn parse_temperature(s: &str) -> i32 {
	let digits = s.split('*').collect::<Vec<&str>>();
	i32::from_str(digits[0]).unwrap()
}

pub fn parse_weather(line_begin: usize, line_end: usize) -> (i32, i32) {
	let mut min_delta = i32::max_value();
	let mut min_day = 0;

	let fin = File::open("data/weather.dat").unwrap();
	let reader = BufReader::new(fin);
	let lines = reader.lines();

	for (i, item) in lines.enumerate() {
		let line = item.unwrap();

		if i >= line_begin && i <= line_end {
			let columns = words(&line).collect::<Vec<&str>>();
			let day = i32::from_str(columns[0]).unwrap();
			let max = parse_temperature(columns[1]);
			let min = parse_temperature(columns[2]);
			// println!("{}: ({}, {})", day, max, min);
			
			let delta = max - min;
			if delta < min_delta {
				min_day = day;
				min_delta = delta;
			}
		}
	}
	
	// println!("min delta({}): day {}", min_delta, min_day);
	return (min_day, min_delta);
}

#[cfg(test)]
mod test {
	use super::parse_weather;

	#[test]
	fn test_weather() {
		assert!((14, 2) == parse_weather(8, 37))
	}
}
