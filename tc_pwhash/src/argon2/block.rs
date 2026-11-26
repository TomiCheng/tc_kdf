use tc_zeroize::Zeroize;

/// A 1024-byte block, as 128 little-endian `u64`s.
pub(super) type Block = [u64; 128];

/// Reads `bytes` into `block`. Constant time.
pub(super) fn block_from_bytes(block: &mut Block, bytes: &[u8; 1024]) {
    for (word, chunk) in block.iter_mut().zip(bytes.chunks_exact(8)) {
        let mut le = [0; 8];
        le.copy_from_slice(chunk);
        *word = u64::from_le_bytes(le);
    }
}

/// Writes `block` into `bytes`. Constant time.
pub(super) fn block_to_bytes(block: &Block, bytes: &mut [u8; 1024]) {
    for (chunk, word) in bytes.chunks_exact_mut(8).zip(block) {
        chunk.copy_from_slice(&word.to_le_bytes());
    }
}

/// The working blocks of the compression function G, Bouncy Castle's `FillBlock`: R, Z and
/// the address blocks of the data-independent passes, wiped on drop.
pub(super) struct Filler {
    r: Block,
    z: Block,
    pub(super) input: Block,
    pub(super) address: Block,
}

impl Filler {
    /// Zeroed working blocks. Constant time.
    pub(super) fn new() -> Self {
        Self {
            r: [0; 128],
            z: [0; 128],
            input: [0; 128],
            address: [0; 128],
        }
    }

    /// R = X ⊕ Y; apart from [`store`](Self::store), so that X and Y may be other blocks of
    /// the memory that holds the target. Constant time.
    pub(super) fn load(&mut self, x: &Block, y: &Block) {
        for ((r, x), y) in self.r.iter_mut().zip(x).zip(y) {
            *r = x ^ y;
        }
    }

    /// current = R ⊕ P(R), or XORed into current with `with_xor`, from the second pass of
    /// version 1.3 on. Constant time: `with_xor` is public.
    pub(super) fn store(&mut self, current: &mut Block, with_xor: bool) {
        self.mix();
        let mixed = self.r.iter().zip(&self.z);
        if with_xor {
            for (c, (r, z)) in current.iter_mut().zip(mixed) {
                *c ^= r ^ z;
            }
        } else {
            for (c, (r, z)) in current.iter_mut().zip(mixed) {
                *c = r ^ z;
            }
        }
    }

    /// The next address block: the counter in `input` goes up by one, and
    /// address = G(0, G(0, input)). Constant time.
    pub(super) fn next_addresses(&mut self) {
        self.input[6] += 1;
        self.r = self.input;
        self.mix_into_address();
        self.r = self.address;
        self.mix_into_address();
    }

    /// address = R ⊕ P(R). Constant time.
    fn mix_into_address(&mut self) {
        self.mix();
        for (a, (r, z)) in self.address.iter_mut().zip(self.r.iter().zip(&self.z)) {
            *a = r ^ z;
        }
    }

    /// Z = P(R). Constant time.
    fn mix(&mut self) {
        self.z = self.r;
        permute_block(&mut self.z);
    }
}

impl Drop for Filler {
    /// Wipes R, Z and the input and address blocks. Constant time.
    fn drop(&mut self) {
        self.r.zeroize();
        self.z.zeroize();
        self.input.zeroize();
        self.address.zeroize();
    }
}

/// The permutation of RFC 9106 section 3.5: the block as 8×8 registers of 16 bytes, with P
/// applied to each row and then to each column. Constant time.
fn permute_block(v: &mut Block) {
    for row in (0..128).step_by(16) {
        permute(v, core::array::from_fn(|k| row + k));
    }
    for column in (0..16).step_by(2) {
        permute(v, core::array::from_fn(|k| column + (k / 2) * 16 + k % 2));
    }
}

/// P over the 16 words at the indices `s`. Constant time.
fn permute(v: &mut Block, s: [usize; 16]) {
    gb(v, s[0], s[4], s[8], s[12]);
    gb(v, s[1], s[5], s[9], s[13]);
    gb(v, s[2], s[6], s[10], s[14]);
    gb(v, s[3], s[7], s[11], s[15]);
    gb(v, s[0], s[5], s[10], s[15]);
    gb(v, s[1], s[6], s[11], s[12]);
    gb(v, s[2], s[7], s[8], s[13]);
    gb(v, s[3], s[4], s[9], s[14]);
}

/// BLAKE2b's G with the addition a + b + 2·lo(a)·lo(b). Constant time.
fn gb(v: &mut Block, a: usize, b: usize, c: usize, d: usize) {
    v[a] = mul_add(v[a], v[b]);
    v[d] = (v[d] ^ v[a]).rotate_right(32);
    v[c] = mul_add(v[c], v[d]);
    v[b] = (v[b] ^ v[c]).rotate_right(24);
    v[a] = mul_add(v[a], v[b]);
    v[d] = (v[d] ^ v[a]).rotate_right(16);
    v[c] = mul_add(v[c], v[d]);
    v[b] = (v[b] ^ v[c]).rotate_right(63);
}

/// x + y + 2·lo(x)·lo(y), wrapping. Constant time.
fn mul_add(x: u64, y: u64) -> u64 {
    let product = (x & 0xffff_ffff) * (y & 0xffff_ffff);
    x.wrapping_add(y).wrapping_add(product << 1)
}
