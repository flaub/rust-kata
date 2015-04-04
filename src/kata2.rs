use std::cmp::Ordering;

fn binary_search(value: i32, slice: &[i32], begin: usize, end: usize) -> i32 {
	let len = end - begin;
	match len {
		0 => -1,
		1 if slice[begin] == value => begin as i32,
		1 => -1,
		_ => {
			let half = begin + len / 2;
			match value.cmp(&slice[half]) {
				Ordering::Less    => binary_search(value, slice, begin, half),
				Ordering::Greater => binary_search(value, slice, half, end),
				Ordering::Equal   => half as i32
			}
		}
	}
}

pub fn chop(value: i32, slice: &[i32]) -> i32 {
	binary_search(value, slice, 0, slice.len())
}

#[cfg(test)]
mod test {
	use super::chop;

	#[test]
	fn test_chop() {
		assert!(-1 == chop(3, &[]));
		assert!(-1 == chop(3, &[1]));
		assert!(0 == chop(1, &[1]));

		assert!(0 == chop(1, &[1, 3, 5]));
		assert!(1 == chop(3, &[1, 3, 5]));
		assert!(2 == chop(5, &[1, 3, 5]));
		assert!(-1 == chop(0, &[1, 3, 5]));
		assert!(-1 == chop(2, &[1, 3, 5]));
		assert!(-1 == chop(4, &[1, 3, 5]));
		assert!(-1 == chop(6, &[1, 3, 5]));

		assert!(0 == chop(1, &[1, 3, 5, 7]));
		assert!(1 == chop(3, &[1, 3, 5, 7]));
		assert!(2 == chop(5, &[1, 3, 5, 7]));
		assert!(3 == chop(7, &[1, 3, 5, 7]));
		assert!(-1 == chop(0, &[1, 3, 5, 7]));
		assert!(-1 == chop(2, &[1, 3, 5, 7]));
		assert!(-1 == chop(4, &[1, 3, 5, 7]));
		assert!(-1 == chop(6, &[1, 3, 5, 7]));
		assert!(-1 == chop(8, &[1, 3, 5, 7]));
	}
}
