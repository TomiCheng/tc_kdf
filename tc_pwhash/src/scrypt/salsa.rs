use tc_zeroize::Zeroize;

/// The Salsa20/8 core of RFC 7914, in place. tc_salsa20 keeps its core private, so this
/// follows the RFC. Constant time.
pub(super) fn salsa20_8(block: &mut [u32; 16]) {
    let mut x = *block;
    for _ in 0..4 {
        quarter_round(&mut x, 0, 4, 8, 12);
        quarter_round(&mut x, 5, 9, 13, 1);
        quarter_round(&mut x, 10, 14, 2, 6);
        quarter_round(&mut x, 15, 3, 7, 11);
        quarter_round(&mut x, 0, 1, 2, 3);
        quarter_round(&mut x, 5, 6, 7, 4);
        quarter_round(&mut x, 10, 11, 8, 9);
        quarter_round(&mut x, 15, 12, 13, 14);
    }
    for (word, mixed) in block.iter_mut().zip(&x) {
        *word = word.wrapping_add(*mixed);
    }
    x.zeroize();
}

/// Salsa20's quarter round on the words at `a`, `b`, `c` and `d`. Constant time.
fn quarter_round(x: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
    x[b] ^= x[a].wrapping_add(x[d]).rotate_left(7);
    x[c] ^= x[b].wrapping_add(x[a]).rotate_left(9);
    x[d] ^= x[c].wrapping_add(x[b]).rotate_left(13);
    x[a] ^= x[d].wrapping_add(x[c]).rotate_left(18);
}
