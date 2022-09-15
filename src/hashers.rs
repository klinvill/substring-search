use itertools::enumerate;
use rand::random;

pub struct RollingPolynomial {
    hash: u64,
    salt: u64,
}

/// This rolling polynomial hash is one such that given a random salt (a) and bytes (of length l),
/// the hash is computed as: b1 * a^(l-1) + b2 * a^(l-2) + ... + bl * a^0. The generated hash is
/// 64 bits and we use wrapping operations so it's implicitly modulo 2^64.
///
/// This hash function offers an alternative to the rolling adler32 hash function.
impl RollingPolynomial {
    pub fn new() -> Self {
        let mut salt = random::<u8>() as u64;
        // Ensure the salt isn't 0 or 1, otherwise our rolling hash won't work well.
        while salt < 2 {
            // We only generate a byte-long random salt since it is used as an exponent which can
            // blow up quickly
            salt = random::<u8>() as u64;
        }
        RollingPolynomial {
            hash: 0,
            salt,
        }
    }

    pub fn with_salt(salt: u64) -> Self {
        RollingPolynomial {
            hash: 0,
            salt,
        }
    }

    pub fn from_buffer(bytes: &[u8]) -> Self {
        let mut instance = Self::new();
        instance.update_buffer(bytes);
        instance
    }

    pub fn from_buffer_with_salt(bytes: &[u8], salt: u64) -> Self {
        let mut instance = Self::with_salt(salt);
        instance.update_buffer(bytes);
        instance
    }

    pub fn update(&mut self, byte: u8) {
        self.hash = self.hash.wrapping_mul(self.salt).wrapping_add(byte as u64);
    }

    pub fn remove(&mut self, size: u32, byte: u8) {
        self.hash = self.hash.wrapping_sub(((byte as u64).wrapping_mul(self.salt.wrapping_pow(size - 1))))
    }

    pub fn update_buffer(&mut self, bytes: &[u8]) {
        for (i, b) in enumerate(bytes) {
            self.update(*b);
        }
    }

    pub fn hash(&self) -> u64 {
        self.hash
    }

    pub fn salt(&self) -> u64 {
        self.salt
    }
}

#[cfg(test)]
mod tests {
    use crate::hashers::RollingPolynomial;

    #[test]
    // Sanity check to make sure the rolling poly hash works as I expect. That is, removing the
    // first element and adding another element should produce the same hash as if you hashed those
    // same elements (in the same order) to begin with.
    fn test_rolling_polynomial() {
        let s = "This is a test string. - Normal Person";
        // Testing with window size of 5
        let mut hash = RollingPolynomial::from_buffer(s[0..5].as_bytes());
        hash.remove(5, s[0..1].as_bytes()[0]);
        hash.update(s[5..6].as_bytes()[0]);
        assert_eq!(hash.hash(), RollingPolynomial::from_buffer_with_salt(s[1..6].as_bytes(), hash.salt).hash());
    }
}
