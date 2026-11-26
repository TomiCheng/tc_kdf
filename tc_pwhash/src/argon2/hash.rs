use tc_blake::Blake2bDigest;
use tc_digest::Digest;
use tc_zeroize::Zeroize;

/// H' of RFC 9106 section 3.3, which stretches BLAKE2b to any length; BLAKE2b's `do_final`
/// resets its state, so no intermediate value is left. Constant time.
pub(super) fn variable_hash(input: &[u8], out: &mut [u8]) {
    let length = (out.len() as u32).to_le_bytes();
    if out.len() <= 64 {
        let mut digest = Blake2bDigest::with_digest_size(out.len() * 8);
        digest.update(&length);
        digest.update(input);
        digest.do_final(out);
        return;
    }

    let mut v = [0; 64];
    let mut digest = Blake2bDigest::new();
    digest.update(&length);
    digest.update(input);
    digest.do_final(&mut v);
    out[..32].copy_from_slice(&v[..32]);

    let r = out.len().div_ceil(32) - 2;
    let mut at = 32;
    for _ in 2..=r {
        digest.update(&v);
        digest.do_final(&mut v);
        out[at..at + 32].copy_from_slice(&v[..32]);
        at += 32;
    }

    let mut last = Blake2bDigest::with_digest_size((out.len() - 32 * r) * 8);
    last.update(&v);
    last.do_final(&mut out[at..]);
    v.zeroize();
}
