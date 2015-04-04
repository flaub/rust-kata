use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Filter;
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
	s.trim_right_matches("*").parse::<i32>().unwrap()
}

struct Data<T> {
	key: T,
	value1: i32,
	value2: i32,
}

struct Accumulator<T> {
	key: T,
	value: i32,
}

fn min_delta<K>(acc: Accumulator<K>, item: Data<K>) -> Accumulator<K> {
	let delta = (item.value1 - item.value2).abs();
	if delta < acc.value { 
		Accumulator { key: item.key, value: delta }
	}
	else { 
		acc
	}
}

fn parse_file<K, F>(filename: &str, default: K, pluck: F) -> (K, i32) 
	where F : Fn(Vec<&str>) -> Option<Data<K>> {

	let fin = File::open(filename).unwrap();
	let reader = BufReader::new(fin);
	let lines = reader.lines();

	let init = Accumulator { key: default, value: i32::max_value() };

	let result = lines.filter_map(|x| {
		let line = x.unwrap();
		let columns = words(&line).collect::<Vec<&str>>();
		pluck(columns)
	}).fold(init, min_delta);

	return (result.key, result.value);
}

pub fn parse_weather() -> (i32, i32) {
	parse_file("data/weather.dat", 0, |columns| {
		if columns.len() < 3 {
			return None;
		}

		let day = match columns[0].parse::<i32>() {
			Ok(n) => n,
			Err(_) => return None
		};

		return Some(Data {
			key: day,
			value1: parse_temperature(columns[1]),
			value2: parse_temperature(columns[2]),
		});
	})
}

pub fn parse_football() -> (String, i32) {
	parse_file("data/football.dat", String::new(), |columns| {
		if columns.len() != 10 {
			return None;
		}

		return Some(Data {
			key: columns[1].to_string(),
			value1: columns[6].parse::<i32>().unwrap(),
			value2: columns[8].parse::<i32>().unwrap(),
		});
	})
}

#[cfg(test)]
mod test {
	use super::parse_weather;
	use super::parse_football;

	#[test]
	fn test_weather() {
		assert!((14, 2) == parse_weather())
	}

	#[test]
	fn test_football() {
		assert!(("Aston_Villa".to_string(), 1) == parse_football())
	}
}
