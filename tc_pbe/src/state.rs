use alloc::vec::Vec;

use tc_zeroize::Zeroizing;

use crate::{PbeError, PbeParams};

/// The parameters a generator copies at `init`; the password and salt are wiped when replaced
/// and on drop, since scrypt passes a secret as the salt.
pub(crate) struct PbeState {
    pub(crate) password: Zeroizing<Vec<u8>>,
    pub(crate) salt: Zeroizing<Vec<u8>>,
    pub(crate) iteration_count: usize,
}

impl PbeState {
    /// Copies `params`, refusing an iteration count of 0 before copying anything. Constant time
    /// with respect to the password and salt.
    pub(crate) fn copy<P: PbeParams + ?Sized>(params: &P) -> Result<Self, PbeError> {
        let iteration_count = params.iteration_count();
        if iteration_count == 0 {
            return Err(PbeError::InvalidIterationCount(0));
        }
        Ok(Self {
            password: Zeroizing::new(params.password().to_vec()),
            salt: Zeroizing::new(params.salt().to_vec()),
            iteration_count,
        })
    }
}
