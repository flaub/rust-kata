use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

struct Data<K>(K, i32, i32);
struct Fold<K>(K, i32);

fn trim(s: &str) -> &str {
    s.trim_end_matches("*")
}

fn words(s: &str) -> Vec<&str> {
    s.split(char::is_whitespace)
        .filter(|s| !s.is_empty())
        .collect()
}

macro_rules! parse {
    ($src:ident, $ix:expr) => {
        if $src.len() < $ix + 1 {
            return None;
        } else {
            match FromStr::from_str(trim($src[$ix])) {
                Ok(val) => val,
                Err(_) => return None,
            }
        }
    };
}

fn parse_file<K>(filename: &str, default: K, ixs: [usize; 3]) -> (K, i32)
where
    K: FromStr,
{
    let fin = File::open(filename).unwrap();
    let reader = BufReader::new(fin);
    let lines = reader.lines();

    let init = Fold(default, i32::max_value());

    let Fold(key, value) = lines
        .filter_map(|x| {
            let line = x.unwrap();
            let columns = words(&line);

            let key = parse!(columns, ixs[0]);
            let v1 = parse!(columns, ixs[1]);
            let v2 = parse!(columns, ixs[2]);

            return Some(Data(key, v1, v2));
        })
        .fold(init, |state, item| {
            let delta = (item.1 - item.2).abs();
            if delta < state.1 {
                Fold(item.0, delta)
            } else {
                state
            }
        });

    return (key, value);
}

pub fn parse_weather() -> (i32, i32) {
    parse_file("data/weather.dat", 0, [0, 1, 2])
}

pub fn parse_football() -> (String, i32) {
    parse_file("data/football.dat", String::new(), [1, 6, 8])
}

#[cfg(test)]
mod test {
    use super::parse_football;
    use super::parse_weather;

    #[test]
    fn test_weather() {
        assert!((14, 2) == parse_weather())
    }

    #[test]
    fn test_football() {
        assert!(("Aston_Villa".to_string(), 1) == parse_football())
    }
}
