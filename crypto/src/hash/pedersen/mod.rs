
use super::{ByteDigest, ElementHasher, Hasher};
use starknet_crypto::pedersen_hash as pedersen;
use starknet_ff::FieldElement as Fe;
use core::{fmt::Debug, marker::PhantomData};
use math::field::f252::{FieldElement, StarkField};

#[cfg(test)]
mod tests;

pub fn pedersen_hash(bytes: &[u8]) -> [u8; 32] {
    assert_eq!(bytes.len() % 32, 0, "bytes len must be divisible by 32");
    let len = Fe::from(bytes.len() / 32);

    let hash = bytes.chunks(32).fold(Fe::from(0u8), |hash, slice| {
        let item = {
            let mut chunk = [0u8; 32];
            write_be_bytes(slice, &mut chunk);
            Fe::from_bytes_be(&chunk).unwrap()
        };
        pedersen(&hash, &item)
    });
    let result = pedersen(&hash, &len).to_bytes_be();

    let mut digest = [0u8; 32];
    write_be_bytes(result.as_slice(), &mut digest);

    return digest;
}

// PEDERSEN 256-BIT OUTPUT
// ================================================================================================

/// Implementation of the [Hasher](super::Hasher) trait for PEDERSEN hash function with 256-bit
/// output.
#[derive(Debug, PartialEq, Eq)]
pub struct Pedersen_256<B: StarkField>(PhantomData<B>);

impl<B: StarkField> Hasher for Pedersen_256<B> {
    type Digest = ByteDigest<32>;

    fn hash(bytes: &[u8]) -> Self::Digest {
        ByteDigest(pedersen_hash(bytes))
    }

    fn merge(values: &[Self::Digest; 2]) -> Self::Digest {
        ByteDigest(pedersen_hash(ByteDigest::digests_as_bytes(values)).into())
    }

    fn merge_with_int(seed: Self::Digest, value: u64) -> Self::Digest {
        let mut int = [0u8; 32];
        int[0..8].copy_from_slice(&value.to_le_bytes());
        Self::merge(&[seed, ByteDigest(int)])
    }
}

use utils::collections::Vec;

impl<B: StarkField> ElementHasher for Pedersen_256<B> {
    type BaseField = B;

    fn hash_elements<E: FieldElement<BaseField = Self::BaseField>>(elements: &[E]) -> Self::Digest {

        let len = Fe::from(elements.len());
        let montgomery = Fe::from_bytes_be(&[0x00u8,0x40,0x00,0x00,0x00,0x00,0x00,0x01,
                                             0x10u8,0x00,0x00,0x00,0x00,0x00,0x01,0x21,
                                             0x00u8,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
                                             0x00u8,0x00,0x00,0x00,0x00,0x00,0x00,0x00]).unwrap();

        let elements_as_chunks = E::elements_as_bytes(elements).chunks(E::ELEMENT_BYTES);
        
        let mut data: Vec<u8> = vec![0; elements.len()*E::ELEMENT_BYTES];

        for (dst, src) in data.chunks_mut(E::ELEMENT_BYTES).zip(elements_as_chunks) {

            let element = {
                let mut chunk = [0u8; 32];
                write_be_bytes(&src, &mut chunk);
                Fe::from_bytes_be(&chunk).unwrap()
            } * montgomery;

            dst.copy_from_slice(&element.to_bytes_be());

        }

        let mut data_endian: Vec<u8> = vec![0; elements.len()*E::ELEMENT_BYTES];
        for (dst, src) in data_endian.chunks_mut(32).zip(data.chunks(32)) {

            write_be_bytes(&src, dst.try_into().unwrap()); 
        }

        ByteDigest(pedersen_hash(data_endian.as_slice()))
    }
}

fn write_be_bytes(value: &[u8], out: &mut [u8; 32]) {
    for (src, dst) in value.iter().rev().zip(out) {
        *dst = *src;
    }
}
