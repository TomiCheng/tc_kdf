use tc_zeroize::Zeroize;

use super::salsa::salsa20_8;

/// scryptBlockMix of RFC 7914 over `input` and `output`, each 2r blocks of 16 words.
/// Constant time.
fn block_mix(input: &[u32], output: &mut [u32]) {
    let r = input.len() / 32;
    let mut x = [0; 16];
    x.copy_from_slice(&input[input.len() - 16..]);
    for (i, block) in input.chunks_exact(16).enumerate() {
        for (x, b) in x.iter_mut().zip(block) {
            *x ^= b;
        }
        salsa20_8(&mut x);
        let at = (i / 2 + (i % 2) * r) * 16;
        output[at..at + 16].copy_from_slice(&x);
    }
    x.zeroize();
}

/// scryptROMix of RFC 7914, in place on `block`, with `v` of `n` blocks and `scratch` of one.
/// Bouncy Castle splits `v` into chunks of at most 32 KiB for .NET's allocator alone; here it
/// is one allocation. Variable time (cache timing): the second loop reads `v` at an index
/// taken from the state, which depends on the password.
pub(super) fn ro_mix(block: &mut [u32], v: &mut [u32], scratch: &mut [u32], n: usize) {
    let len = block.len();
    for entry in v.chunks_exact_mut(len) {
        entry.copy_from_slice(block);
        block_mix(entry, block);
    }
    for _ in 0..n {
        // Integerify: the first word of the last 64-byte block; n is at most 2^31, so one
        // word is enough
        let j = block[len - 16] as usize & (n - 1);
        let entry = &v[j * len..(j + 1) * len];
        for ((s, x), v) in scratch.iter_mut().zip(block.iter()).zip(entry) {
            *s = x ^ v;
        }
        block_mix(scratch, block);
    }
}
