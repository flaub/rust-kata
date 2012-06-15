use std;

import io::reader_util;

impl reader_ext for io::reader {
	fn eachi_line(it: fn(uint, str) -> bool) {
		let mut i: uint = 0u;
		while !self.eof() {
			if !it(i, self.read_line()) { break; }
			i = i + 1u;
		}
	}
}

fn parse_temperature(s: str) -> int {
	if str::ends_with(s, "*") {
		ret parse_temperature(str::split_char(s, '*')[0]);
	}
	ret int::from_str(s).get();
}

fn parse_weather(line_begin: uint, line_end: uint) {
	let fin: io::reader = result::get(io::file_reader("weather.dat"));
	
	let mut min_delta = int::max_value;
	let mut min_day = 0u;

	for fin.eachi_line {|i, line|
		if i >= line_begin && i <= line_end {
			// io::println(#fmt("%u: %s", i, line));
			let columns = str::words(line);
			let day = uint::from_str(columns[0]).get();
			let max = parse_temperature(columns[1]);
			let min = parse_temperature(columns[2]);
			// io::println(#fmt("%u: (%d, %d)", day, max, min));
			
			let delta = max - min;
			if delta < min_delta {
				min_day = day;
				min_delta = delta;
			}
		}
	}
	
	io::println(#fmt("min delta(%d): day %u", min_delta, min_day))
}

#[test]
fn test_weather() {
	parse_weather(8u, 37u)
}
