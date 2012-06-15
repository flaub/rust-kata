use std;

fn find(v: int, array: [int]) -> int {
	for array.eachi { |i, elt|
		if elt == v { ret i as int }
	}
	ret -1;
}

enum compare {
	lt,
	gt,
	eq
}

fn compare<T>(a: T, b: T) -> compare {
	if a < b { lt }
	else if a > b { gt }
	else { eq }
}

fn _chop(value: int, array: [int], begin: uint, end: uint) -> int {
	let len = end - begin;
	alt len {
		0u { ret -1; }
		1u if array[begin] == value { ret begin as int; }
		1u { ret -1; }
		_ {
			let half = begin + len / 2u;
			alt compare(value, array[half]) {
				lt { ret _chop(value, array, begin, half); }
				gt { ret _chop(value, array, half, end); }
				eq { ret half as int; }
			}
		}
	}
}

fn chop(value: int, array: [int]) -> int {
	ret _chop(value, array, 0u, array.len())
}

fn main() {
	io::println("Hello World");
}

#[test]
fn test_chop() {
	assert(-1 == chop(3, []));
	assert(-1 == chop(3, [1]));
	assert(0 == chop(1, [1]));

	assert(0 == chop(1, [1, 3, 5]));
	assert(1 == chop(3, [1, 3, 5]));
	assert(2 == chop(5, [1, 3, 5]));
	assert(-1 == chop(0, [1, 3, 5]));
	assert(-1 == chop(2, [1, 3, 5]));
	assert(-1 == chop(4, [1, 3, 5]));
	assert(-1 == chop(6, [1, 3, 5]));

	assert(0 == chop(1, [1, 3, 5, 7]));
	assert(1 == chop(3, [1, 3, 5, 7]));
	assert(2 == chop(5, [1, 3, 5, 7]));
	assert(3 == chop(7, [1, 3, 5, 7]));
	assert(-1 == chop(0, [1, 3, 5, 7]));
	assert(-1 == chop(2, [1, 3, 5, 7]));
	assert(-1 == chop(4, [1, 3, 5, 7]));
	assert(-1 == chop(6, [1, 3, 5, 7]));
	assert(-1 == chop(8, [1, 3, 5, 7]));
}
