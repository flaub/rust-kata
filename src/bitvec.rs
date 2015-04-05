use std::iter::repeat;

static TRUE: bool = true;
static FALSE: bool = false;

static BITS: usize = 32;
type StorageType = u32;

pub struct BitVec {
	storage: Vec<StorageType>,
	nbits: usize,
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

	/// Clears all bits in this vector.
	#[inline]
	pub fn clear(&mut self) {
		for w in &mut self.storage { *w = 0; }
	}
}
