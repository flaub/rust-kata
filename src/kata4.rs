use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Filter;
use std::str::Split;

type FnIsWhitespace = fn(char) -> bool;
type FnIsNotEmpty = fn(&&str) -> bool;

struct Words<'a> {
	inner: Filter<Split<'a, FnIsWhitespace>, FnIsNotEmpty>,
}

impl<'a> Iterator for Words<'a> {
	type Item = &'a str;

	fn next(&mut self) -> Option<&'a str> { self.inner.next() }
}

fn words(s: &str) -> Words {
	fn is_not_empty(s: &&str) -> bool { !s.is_empty() }
	fn is_whitespace(c: char) -> bool { c.is_whitespace() }

	Words { 
		inner: s
			.split(is_whitespace as FnIsWhitespace)
			.filter(is_not_empty) 
	}
}

struct Data<K> (K, i32, i32);
struct Fold<K> (K, i32);

fn parse_file<K, F>(filename: &str, default: K, pluck: F) -> (K, i32) 
	where F : Fn(Vec<&str>) -> Option<Data<K>> {

	let fin = File::open(filename).unwrap();
	let reader = BufReader::new(fin);
	let lines = reader.lines();

	let init = Fold(default, i32::max_value());

	let Fold(key, value) = lines.filter_map(|x| {
		let line = x.unwrap();
		let columns = words(&line).collect::<Vec<&str>>();
		pluck(columns)
	}).fold(init, |state, item| {
		let delta = (item.1 - item.2).abs();
		if delta < state.1 { 
			Fold(item.0, delta)
		}
		else { 
			state
		}
	});

	return (key, value);
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
		
		fn trim(s: &str) -> i32 {
			s.trim_right_matches("*").parse::<i32>().unwrap()
		}

		return Some(Data(day, trim(columns[1]), trim(columns[2])));
	})
}

pub fn parse_football() -> (String, i32) {
	parse_file("data/football.dat", String::new(), |columns| {
		if columns.len() != 10 {
			return None;
		}

		return Some(Data(
			columns[1].to_string(), 
			columns[6].parse::<i32>().unwrap(), 
			columns[8].parse::<i32>().unwrap(),
		));
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
