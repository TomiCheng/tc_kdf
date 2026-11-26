use tc_zeroize::Zeroize;

use super::tables::{P_INIT, S_INIT};

const ROUNDS: usize = 16;

// "OrpheanBeholderScryDoubt"
const MAGIC: [u32; 6] = [
    0x4f727068, 0x65616e42, 0x65686f6c, 0x64657253, 0x63727944, 0x6f756274,
];

#[derive(Clone, Copy)]
enum Table {
    P,
    S,
}

struct EksBlowfish {
    p: [u32; 18],
    s: [u32; 1024],
}

impl EksBlowfish {
    /// The initial P-array and S-boxes. Constant time.
    fn new() -> Self {
        Self {
            p: P_INIT,
            s: S_INIT,
        }
    }

    /// Blowfish's F. Variable time (cache timing): it indexes the S-boxes with the bytes of
    /// `x`, which depend on the password.
    fn f(&self, x: u32) -> u32 {
        let s = &self.s;
        (s[(x >> 24) as usize].wrapping_add(s[256 + ((x >> 16) & 0xff) as usize])
            ^ s[512 + ((x >> 8) & 0xff) as usize])
            .wrapping_add(s[768 + (x & 0xff) as usize])
    }

    /// One Blowfish encryption of `(l, r)`. Variable time (cache timing), through
    /// [`f`](Self::f).
    fn encrypt(&self, mut l: u32, mut r: u32) -> (u32, u32) {
        l ^= self.p[0];
        for i in (1..ROUNDS).step_by(2) {
            r ^= self.f(l) ^ self.p[i];
            l ^= self.f(r) ^ self.p[i + 1];
        }
        r ^= self.p[ROUNDS + 1];
        (r, l)
    }

    /// Bouncy Castle's `ProcessTable` and `ProcessTableWithSalt` in one: each pair is XORed
    /// with its half of the salt before it is encrypted into the table. Variable time (cache
    /// timing), through [`encrypt`](Self::encrypt).
    fn process(&mut self, table: Table, salt: &[u32; 4], mut l: u32, mut r: u32) {
        let len = match table {
            Table::P => self.p.len(),
            Table::S => self.s.len(),
        };
        for i in (0..len).step_by(2) {
            let half = i & 2;
            (l, r) = self.encrypt(l ^ salt[half], r ^ salt[half + 1]);
            let words = match table {
                Table::P => &mut self.p[i..i + 2],
                Table::S => &mut self.s[i..i + 2],
            };
            words[0] = l;
            words[1] = r;
        }
    }

    /// The expensive key schedule's ExpandKey. Variable time (cache timing), through
    /// [`process`](Self::process).
    fn expand_key(&mut self, key: &[u8], salt: &[u32; 4]) {
        let mut at = 0;
        for p in &mut self.p {
            let mut word = 0;
            for _ in 0..4 {
                word = (word << 8) | u32::from(key[at]);
                at += 1;
                if at == key.len() {
                    at = 0;
                }
            }
            *p ^= word;
        }
        self.process(Table::P, salt, 0, 0);
        let swapped = [salt[2], salt[3], salt[0], salt[1]];
        self.process(Table::S, &swapped, self.p[16], self.p[17]);
    }

    /// "OrpheanBeholderScryDoubt" encrypted 64 times. Variable time (cache timing), through
    /// [`encrypt`](Self::encrypt).
    fn encrypt_magic(&self) -> [u8; 24] {
        let mut text = MAGIC;
        for _ in 0..64 {
            for pair in text.chunks_exact_mut(2) {
                (pair[0], pair[1]) = self.encrypt(pair[0], pair[1]);
            }
        }
        let mut out = [0; 24];
        for (bytes, word) in out.chunks_exact_mut(4).zip(&text) {
            bytes.copy_from_slice(&word.to_be_bytes());
        }
        text.zeroize();
        out
    }
}

impl Drop for EksBlowfish {
    /// Wipes the P-array and S-boxes. Constant time.
    fn drop(&mut self) {
        self.p.zeroize();
        self.s.zeroize();
    }
}

/// The raw 24-byte bcrypt output. The caller ensures that `password` is not empty and at most
/// 72 bytes and that `cost` is in 4..=31. Variable time (cache timing): Blowfish indexes its
/// S-boxes with bytes that depend on the password.
pub(super) fn derive(password: &[u8], salt: &[u8; 16], cost: u8) -> [u8; 24] {
    let mut salt_words = [0; 4];
    for (word, bytes) in salt_words.iter_mut().zip(salt.chunks_exact(4)) {
        *word = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    }

    let mut state = EksBlowfish::new();
    state.expand_key(password, &salt_words);
    for _ in 0..1u32 << cost {
        state.expand_key(password, &[0; 4]);
        state.expand_key(salt, &[0; 4]);
    }
    state.encrypt_magic()
}
