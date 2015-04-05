use std::default::Default;
use std::fmt;
use std::iter::repeat;
// use std::iter::Cloned;
use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::ops::Index;
// use std::slice;

// type Blocks<'a> = Cloned<slice::Iter<'a, u32>>;
// type MutBlocks<'a> = slice::IterMut<'a, u32>;

static TRUE: bool = true;
static FALSE: bool = false;

static BITS: usize = 64;

pub struct BitVec {
	storage: Vec<u64>,
	nbits: usize,
}

impl Index<usize> for BitVec {
	type Output = bool;

	#[inline]
	fn index(&self, i: usize) -> &bool {
		if self.get(i).expect("index out of bounds") {
			&TRUE
		} else {
			&FALSE
		}
	}
}

/// Computes how many blocks are needed to store that many bits
fn blocks_for_bits(bits: usize) -> usize {
	// If we want 17 bits, dividing by 32 will produce 0. So we add 1 to make sure we
	// reserve enough. But if we want exactly a multiple of 32, this will actually allocate
	// one too many. So we need to check if that's the case. We can do that by computing if
	// bitwise AND by `32 - 1` is 0. But LLVM should be able to optimize the semantically
	// superior modulo operator on a power of two to this.
	//
	// Note that we can technically avoid this branch with the expression
	// `(nbits + u32::BITS - 1) / 32::BITS`, but if nbits is almost usize::MAX this will overflow.
	if bits % BITS == 0 {
		bits / BITS
	} else {
		bits / BITS + 1
	}
}

/// The bitvector type.
impl BitVec {
	pub fn new() -> BitVec {
		BitVec { storage: Vec::new(), nbits: 0 }
	}

    /// Creates a `BitVec` that holds `nbits` elements, setting each element
    /// to `bit`.
    pub fn from_elem(nbits: usize, bit: bool) -> BitVec {
        let nblocks = blocks_for_bits(nbits);
        let mut bit_vec = BitVec {
            storage: repeat(if bit { !0 } else { 0 }).take(nblocks).collect(),
            nbits: nbits
        };
        bit_vec.fix_last_block();
        bit_vec
    }

	/// Constructs a new, empty `BitVec` with the specified capacity.
	///
	/// The bitvector will be able to hold at least `capacity` bits without
	/// reallocating. If `capacity` is 0, it will not allocate.
	///
	/// It is important to note that this function does not specify the
	/// *length* of the returned bitvector, but only the *capacity*.
	pub fn with_capacity(nbits: usize) -> BitVec {
		BitVec {
			storage: Vec::with_capacity(blocks_for_bits(nbits)),
			nbits: 0,
		}
	}

	/// Return the total number of bits in this vector
	#[inline]
	pub fn len(&self) -> usize { self.nbits }

	/// Returns true if there are no bits in this vector
	#[inline]
	pub fn is_empty(&self) -> bool { self.len() == 0 }

	/// An operation might screw up the unused bits in the last block of the
	/// `BitVec`. As per (3), it's assumed to be all 0s. This method fixes it up.
	fn fix_last_block(&mut self) {
		let extra_bits = self.len() % BITS;
		if extra_bits > 0 {
			let mask = (1 << extra_bits) - 1;
			let storage_len = self.storage.len();
			self.storage[storage_len - 1] &= mask;
		}
	}

	/// Retreives the value at index `i`, or `None` if the index is out of bounds.
	#[inline]
	pub fn get(&self, i: usize) -> Option<bool> {
		if i >= self.nbits {
			return None;
		}
		let w = i / BITS;
		let b = i % BITS;
		self.storage.get(w).map(|&block|
			(block & (1 << b)) != 0
		)
	}

	/// Sets the value of a bit at an index `i`.
	///
	/// # Panics
	///
	/// Panics if `i` is out of bounds.
	#[inline]
	pub fn set(&mut self, i: usize, x: bool) {
		assert!(i < self.nbits);
		let w = i / BITS;
		let b = i % BITS;
		let flag = 1 << b;
		let val = if x { self.storage[w] | flag }
				  else { self.storage[w] & !flag };
		self.storage[w] = val;
	}

	/// Sets all bits to 1.
	#[inline]
	pub fn set_all(&mut self) {
		for w in &mut self.storage { *w = !0; }
		self.fix_last_block();
	}

	/// Flips all bits.
	#[inline]
	pub fn negate(&mut self) {
		for w in &mut self.storage { *w = !*w; }
		self.fix_last_block();
	}

	// /// Returns `true` if all bits are 0.
	// pub fn none(&self) -> bool {
	//     self.blocks().all(|w| w == 0)
	// }

	// /// Returns `true` if any bit is 1.
	// #[inline]
	// pub fn any(&self) -> bool {
	//     !self.none()
	// }

	/// Returns an iterator over the elements of the vector in order.
	#[inline]
	pub fn iter(&self) -> Iter {
		Iter { bit_vec: self, next_idx: 0, end_idx: self.nbits }
	}

	/// Reserves capacity for at least `additional` more bits to be inserted in the given
	/// `BitVec`. The collection may reserve more space to avoid frequent reallocations.
	///
	/// # Panics
	///
	/// Panics if the new capacity overflows `usize`.
	pub fn reserve(&mut self, additional: usize) {
		let desired_cap = self.len().checked_add(additional).expect("capacity overflow");
		let storage_len = self.storage.len();
		if desired_cap > self.capacity() {
			self.storage.reserve(blocks_for_bits(desired_cap) - storage_len);
		}
	}

	/// Returns the capacity in bits for this bit vector. Inserting any
	/// element less than this amount will not trigger a resizing.
	///
	/// # Examples
	///
	/// ```
	/// use kata::bitvec::BitVec;
	///
	/// let mut bv = BitVec::new();
	/// bv.reserve(10);
	/// assert!(bv.capacity() >= 10);
	/// ```
	#[inline]
	pub fn capacity(&self) -> usize {
		self.storage.capacity().checked_mul(BITS).unwrap_or(usize::max_value())
	}

	/// Pushes a `bool` onto the end.
	pub fn push(&mut self, elem: bool) {
		if self.nbits % BITS == 0 {
			self.storage.push(0);
		}
		let insert_pos = self.nbits;
		self.nbits = self.nbits.checked_add(1).expect("Capacity overflow");
		self.set(insert_pos, elem);
	}

	/// Clears all bits in this vector.
	#[inline]
	pub fn clear(&mut self) {
		for w in &mut self.storage { *w = 0; }
	}
}

impl Default for BitVec {
	#[inline]
	fn default() -> BitVec { BitVec::new() }
}

impl FromIterator<bool> for BitVec {
	fn from_iter<I: IntoIterator<Item=bool>>(iter: I) -> BitVec {
		let mut ret = BitVec::new();
		ret.extend(iter);
		ret
	}
}

impl Extend<bool> for BitVec {
	#[inline]
	fn extend<I: IntoIterator<Item=bool>>(&mut self, iterable: I) {
		let iterator = iterable.into_iter();
		let (min, _) = iterator.size_hint();
		self.reserve(min);
		for element in iterator {
			self.push(element)
		}
	}
}

impl Clone for BitVec {
	#[inline]
	fn clone(&self) -> BitVec {
		BitVec { storage: self.storage.clone(), nbits: self.nbits }
	}

	// #[inline]
	// fn clone_from(&mut self, source: &BitVec) {
	//     self.nbits = source.nbits;
	//     self.storage.clone_from(&source.storage);
	// }
}

impl fmt::Debug for BitVec {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		for bit in self {
			try!(write!(fmt, "{}", if bit { 1 } else { 0 }));
		}
		Ok(())
	}
}

/// An iterator for `BitVec`.
#[derive(Clone)]
pub struct Iter<'a> {
	bit_vec: &'a BitVec,
	next_idx: usize,
	end_idx: usize,
}

impl<'a> Iterator for Iter<'a> {
	type Item = bool;

	#[inline]
	fn next(&mut self) -> Option<bool> {
		if self.next_idx != self.end_idx {
			let idx = self.next_idx;
			self.next_idx += 1;
			Some(self.bit_vec[idx])
		} else {
			None
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		let rem = self.end_idx - self.next_idx;
		(rem, Some(rem))
	}
}

impl<'a> DoubleEndedIterator for Iter<'a> {
	#[inline]
	fn next_back(&mut self) -> Option<bool> {
		if self.next_idx != self.end_idx {
			self.end_idx -= 1;
			Some(self.bit_vec[self.end_idx])
		} else {
			None
		}
	}
}

impl<'a> ExactSizeIterator for Iter<'a> {}

impl<'a> IntoIterator for &'a BitVec {
	type Item = bool;
	type IntoIter = Iter<'a>;

	fn into_iter(self) -> Iter<'a> {
		self.iter()
	}
}
