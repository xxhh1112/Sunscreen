use std::ops::{Add, Mul};

use seal::Plaintext as SealPlaintext;

use crate::{
    types::{BfvType, CircuitNode, FheType},
    Context, TypeName as DeriveTypeName, CURRENT_CTX, Params,
};
use sunscreen_runtime::{
    InnerPlaintext, NumCiphertexts, Plaintext, TryFromPlaintext, TryIntoPlaintext,
};

impl CircuitNode<Unsigned> {
    /**
     * Returns the plain modulus parameter for the given BFV scheme
     */
    pub fn get_plain_modulus() -> u64 {
        with_ctx(|ctx| ctx.params.plain_modulus)
    }
}

#[derive(Debug, Clone, Copy, DeriveTypeName, PartialEq, Eq)]
/**
 * A single unsigned integer.
 */
pub struct Unsigned {
    val: u64,
}

impl std::ops::Deref for Unsigned {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl FheType for Unsigned {}
impl BfvType for Unsigned {}

impl Unsigned {}

impl Add for CircuitNode<Unsigned> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        with_ctx(|ctx| Self::new(ctx.add_addition(self.id, other.id)))
    }
}

impl Mul for CircuitNode<Unsigned> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        with_ctx(|ctx| Self::new(ctx.add_multiplication(self.id, other.id)))
    }
}

fn with_ctx<F, R>(f: F) -> R
where
    F: FnOnce(&mut Context) -> R,
{
    CURRENT_CTX.with(|ctx| {
        let mut option = ctx.borrow_mut();
        let ctx = option
            .as_mut()
            .expect("Called Ciphertext::new() outside of a context.");

        f(ctx)
    })
}

impl TryIntoPlaintext for Unsigned {
    fn try_into_plaintext(&self, _params: &Params) -> std::result::Result<Plaintext, sunscreen_runtime::Error> {
        let mut seal_plaintext = SealPlaintext::new()?;
        let bits = std::mem::size_of::<u64>() * 8;

        seal_plaintext.resize(bits);

        for i in 0..bits {
            let bit_value = (self.val & 0x1 << i) >> i;
            seal_plaintext.set_coefficient(i, bit_value);
        }

        Ok(Plaintext {
            inner: InnerPlaintext::Seal(vec![seal_plaintext]),
        })
    }
}

impl TryFromPlaintext for Unsigned {
    fn try_from_plaintext(
        plaintext: &Plaintext,
        _params: &Params
    ) -> std::result::Result<Self, sunscreen_runtime::Error> {
        let val = match &plaintext.inner {
            InnerPlaintext::Seal(p) => {
                if p.len() != 1 {
                    return Err(sunscreen_runtime::Error::IncorrectCiphertextCount);
                }

                let mut val = 0u64;
                let bits = usize::min(std::mem::size_of::<u64>() * 8, p[0].len());

                for i in 0..bits {
                    val += p[0].get_coefficient(i) * (1 << i);
                }

                Self { val }
            }
        };

        Ok(val)
    }
}

impl NumCiphertexts for Unsigned {
    fn num_ciphertexts() -> usize {
        1
    }
}

impl From<u64> for Unsigned {
    fn from(val: u64) -> Self {
        Self { val }
    }
}

impl Into<u64> for Unsigned {
    fn into(self) -> u64 {
        self.val
    }
}

#[derive(Debug, Clone, Copy, DeriveTypeName, PartialEq, Eq)]
/**
 * A single signed integer.
 */
pub struct Signed {
    val: i64,
}

impl NumCiphertexts for Signed {
    fn num_ciphertexts() -> usize {
        1
    }
}

impl FheType for Signed {}

impl TryIntoPlaintext for Signed {
    fn try_into_plaintext(&self, params: &Params) -> std::result::Result<Plaintext, sunscreen_runtime::Error> {
        let mut seal_plaintext = SealPlaintext::new()?;
        let bits = std::mem::size_of::<u64>() * 8;

        seal_plaintext.resize(bits);

        let unsigned_val = if self.val < 0 {
            -self.val
        } else {
            self.val
        } as u64;

        for i in 0..bits {
            let bit_value = (unsigned_val & 0x1 << i) >> i;

            let coeff_value = if self.val < 0 {
                params.plain_modulus as u64 - bit_value
            } else {
                bit_value
            };

            seal_plaintext.set_coefficient(i, coeff_value);
        }

        Ok(Plaintext {
            inner: InnerPlaintext::Seal(vec![seal_plaintext]),
        })
    }
}

impl TryFromPlaintext for Signed {
    fn try_from_plaintext(
        plaintext: &Plaintext,
        params: &Params
    ) -> std::result::Result<Self, sunscreen_runtime::Error> {
        let val = match &plaintext.inner {
            InnerPlaintext::Seal(p) => {
                if p.len() != 1 {
                    return Err(sunscreen_runtime::Error::IncorrectCiphertextCount);
                }

                let bits = usize::min(std::mem::size_of::<u64>() * 8, p[0].len());

                let negative_cutoff = (params.plain_modulus + 1) / 2;

                let mut val: i64 = 0;

                for i in 0..bits {
                    let coeff =  p[0].get_coefficient(i);
                    
                    if coeff > negative_cutoff {
                        val += ((0x1 << i) * coeff) as i64;
                    } else {
                        val -= ((0x1 << i) * (params.plain_modulus - coeff)) as i64;
                    }
                }

                Self { val }
            }
        };

        Ok(val)
    }
}

impl CircuitNode<Signed> {
    /**
     * Returns the plain modulus parameter for the given BFV scheme
     */
    pub fn get_plain_modulus() -> u64 {
        with_ctx(|ctx| ctx.params.plain_modulus)
    }
}


impl From<i64> for Signed {
    fn from(val: i64) -> Self {
        Self { val }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_convert_u64_to_unsigned() {
        let foo: Unsigned = 64u64.into();

        assert_eq!(foo.val, 64);
    }

    #[test]
    fn can_convert_unsigned_to_u64() {
        let foo = Unsigned { val: 64 };
        let converted: u64 = foo.into();

        assert_eq!(converted, 64u64);
    }
}
