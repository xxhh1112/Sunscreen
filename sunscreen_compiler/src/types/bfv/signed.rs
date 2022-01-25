use seal::Plaintext as SealPlaintext;

use crate::types::{
    ops::{
        GraphCipherAdd, GraphCipherConstAdd, GraphCipherConstMul, GraphCipherConstSub,
        GraphCipherMul, GraphCipherNeg, GraphCipherPlainAdd, GraphCipherPlainMul,
        GraphCipherPlainSub, GraphCipherSub, GraphConstCipherSub, GraphPlainCipherSub,
    },
    Cipher,
};
use crate::{
    types::{intern::CircuitNode, BfvType, FheType, TypeNameInstance},
    with_ctx, CircuitInputTrait, Params, TypeName as DeriveTypeName, WithContext,
};

use sunscreen_runtime::{
    InnerPlaintext, NumCiphertexts, Plaintext, TryFromPlaintext, TryIntoPlaintext,
};
#[derive(Debug, Clone, Copy, DeriveTypeName, PartialEq, Eq)]
/**
 * A single signed integer.
 */
pub struct Signed {
    val: i64,
}

impl NumCiphertexts for Signed {
    const NUM_CIPHERTEXTS: usize = 1;
}

impl CircuitInputTrait for Signed {}
impl FheType for Signed {}
impl BfvType for Signed {}

fn significant_bits(val: u64) -> usize {
    let bits = std::mem::size_of::<u64>() * 8;

    for i in 0..bits {
        if (0x1 << (bits - i - 1)) & val != 0 {
            return bits - i + 1;
        }
    }

    0
}

impl TryIntoPlaintext for Signed {
    fn try_into_plaintext(
        &self,
        params: &Params,
    ) -> std::result::Result<Plaintext, sunscreen_runtime::Error> {
        let mut seal_plaintext = SealPlaintext::new()?;

        let signed_val = if self.val < 0 { -self.val } else { self.val } as u64;

        let sig_bits = significant_bits(signed_val);
        seal_plaintext.resize(sig_bits);

        for i in 0..sig_bits {
            let bit_value = (signed_val & 0x1 << i) >> i;

            let coeff_value = if self.val < 0 {
                bit_value * (params.plain_modulus as u64 - bit_value)
            } else {
                bit_value
            };

            seal_plaintext.set_coefficient(i, coeff_value);
        }

        Ok(Plaintext {
            data_type: self.type_name_instance(),
            inner: InnerPlaintext::Seal(vec![WithContext {
                params: params.clone(),
                data: seal_plaintext,
            }]),
        })
    }
}

impl TryFromPlaintext for Signed {
    fn try_from_plaintext(
        plaintext: &Plaintext,
        params: &Params,
    ) -> std::result::Result<Self, sunscreen_runtime::Error> {
        let val = match &plaintext.inner {
            InnerPlaintext::Seal(p) => {
                if p.len() != 1 {
                    return Err(sunscreen_runtime::Error::IncorrectCiphertextCount);
                }

                let bits = usize::min(
                    usize::min(std::mem::size_of::<u64>() * 8, p[0].len()),
                    p[0].len(),
                );

                let negative_cutoff = (params.plain_modulus + 1) / 2;

                let mut val: i64 = 0;

                for i in 0..bits {
                    let coeff = p[0].get_coefficient(i);

                    if coeff < negative_cutoff {
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

impl From<i64> for Signed {
    fn from(val: i64) -> Self {
        Self { val }
    }
}

impl Into<i64> for Signed {
    fn into(self) -> i64 {
        self.val
    }
}

impl GraphCipherAdd for Signed {
    type Left = Signed;
    type Right = Signed;

    fn graph_cipher_add(
        a: CircuitNode<Cipher<Self::Left>>,
        b: CircuitNode<Cipher<Self::Right>>,
    ) -> CircuitNode<Cipher<Self::Left>> {
        with_ctx(|ctx| {
            let n = ctx.add_addition(a.ids[0], b.ids[0]);

            CircuitNode::new(&[n])
        })
    }
}

impl GraphCipherPlainAdd for Signed {
    type Left = Signed;
    type Right = Signed;

    fn graph_cipher_plain_add(
        a: CircuitNode<Cipher<Self::Left>>,
        b: CircuitNode<Self::Right>,
    ) -> CircuitNode<Cipher<Self::Left>> {
        with_ctx(|ctx| {
            let n = ctx.add_addition_plaintext(a.ids[0], b.ids[0]);

            CircuitNode::new(&[n])
        })
    }
}

impl GraphCipherConstAdd for Signed {
    type Left = Self;
    type Right = i64;

    fn graph_cipher_const_add(
        a: CircuitNode<Cipher<Self::Left>>,
        b: i64,
    ) -> CircuitNode<Cipher<Self::Left>> {
        with_ctx(|ctx| {
            let b = Self::from(b).try_into_plaintext(&ctx.params).unwrap();

            let lit = ctx.add_plaintext_literal(b.inner);
            let add = ctx.add_addition_plaintext(a.ids[0], lit);

            CircuitNode::new(&[add])
        })
    }
}

impl GraphCipherSub for Signed {
    type Left = Signed;
    type Right = Signed;

    fn graph_cipher_sub(
        a: CircuitNode<Cipher<Self::Left>>,
        b: CircuitNode<Cipher<Self::Right>>,
    ) -> CircuitNode<Cipher<Self::Left>> {
        with_ctx(|ctx| {
            let n = ctx.add_subtraction(a.ids[0], b.ids[0]);

            CircuitNode::new(&[n])
        })
    }
}

impl GraphCipherPlainSub for Signed {
    type Left = Signed;
    type Right = Signed;

    fn graph_cipher_plain_sub(
        a: CircuitNode<Cipher<Self::Left>>,
        b: CircuitNode<Self::Right>,
    ) -> CircuitNode<Cipher<Self::Left>> {
        with_ctx(|ctx| {
            let n = ctx.add_subtraction_plaintext(a.ids[0], b.ids[0]);

            CircuitNode::new(&[n])
        })
    }
}

impl GraphPlainCipherSub for Signed {
    type Left = Signed;
    type Right = Signed;

    fn graph_plain_cipher_sub(
        a: CircuitNode<Self::Left>,
        b: CircuitNode<Cipher<Self::Right>>,
    ) -> CircuitNode<Cipher<Self::Left>> {
        with_ctx(|ctx| {
            let n = ctx.add_subtraction_plaintext(b.ids[0], a.ids[0]);
            let n = ctx.add_negate(n);

            CircuitNode::new(&[n])
        })
    }
}

impl GraphCipherConstSub for Signed {
    type Left = Signed;
    type Right = i64;

    fn graph_cipher_const_sub(
        a: CircuitNode<Cipher<Self::Left>>,
        b: Self::Right,
    ) -> CircuitNode<Cipher<Self::Left>> {
        with_ctx(|ctx| {
            let b = Self::from(b).try_into_plaintext(&ctx.params).unwrap();

            let lit = ctx.add_plaintext_literal(b.inner);
            let n = ctx.add_subtraction_plaintext(a.ids[0], lit);

            CircuitNode::new(&[n])
        })
    }
}

impl GraphConstCipherSub for Signed {
    type Left = i64;
    type Right = Signed;

    fn graph_const_cipher_sub(
        a: i64,
        b: CircuitNode<Cipher<Self::Right>>,
    ) -> CircuitNode<Cipher<Self::Right>> {
        with_ctx(|ctx| {
            let a = Self::from(a).try_into_plaintext(&ctx.params).unwrap();

            let lit = ctx.add_plaintext_literal(a.inner);
            let n = ctx.add_subtraction_plaintext(b.ids[0], lit);
            let n = ctx.add_negate(n);

            CircuitNode::new(&[n])
        })
    }
}

impl GraphCipherNeg for Signed {
    type Val = Signed;

    fn graph_cipher_neg(a: CircuitNode<Cipher<Self>>) -> CircuitNode<Cipher<Self>> {
        with_ctx(|ctx| {
            let n = ctx.add_negate(a.ids[0]);

            CircuitNode::new(&[n])
        })
    }
}

impl GraphCipherMul for Signed {
    type Left = Signed;
    type Right = Signed;

    fn graph_cipher_mul(
        a: CircuitNode<Cipher<Self::Left>>,
        b: CircuitNode<Cipher<Self::Right>>,
    ) -> CircuitNode<Cipher<Self::Left>> {
        with_ctx(|ctx| {
            let n = ctx.add_multiplication(a.ids[0], b.ids[0]);

            CircuitNode::new(&[n])
        })
    }
}

impl GraphCipherConstMul for Signed {
    type Left = Self;
    type Right = i64;

    fn graph_cipher_const_mul(
        a: CircuitNode<Cipher<Self::Left>>,
        b: i64,
    ) -> CircuitNode<Cipher<Self::Left>> {
        with_ctx(|ctx| {
            let b = Self::from(b).try_into_plaintext(&ctx.params).unwrap();

            let lit = ctx.add_plaintext_literal(b.inner);
            let add = ctx.add_multiplication_plaintext(a.ids[0], lit);

            CircuitNode::new(&[add])
        })
    }
}

impl GraphCipherPlainMul for Signed {
    type Left = Signed;
    type Right = Signed;

    fn graph_cipher_plain_mul(
        a: CircuitNode<Cipher<Self::Left>>,
        b: CircuitNode<Self::Right>,
    ) -> CircuitNode<Cipher<Self::Left>> {
        with_ctx(|ctx| {
            let n = ctx.add_multiplication_plaintext(a.ids[0], b.ids[0]);

            CircuitNode::new(&[n])
        })
    }
}
