// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use super::{DefaultProverChannel, FriProver};
use crate::{
    verifier::{DefaultVerifierChannel, FriVerifier},
    FriOptions, FriProof, VerifierError,
};
use crypto::{hashers::Blake2s_256, Hasher, RandomCoin};
use math::{fft, fields::f128::BaseElement, FieldElement};
use utils::{collections::Vec, Deserializable, Serializable, SliceReader};

type Blake2s = Blake2s_256<BaseElement>;

// PROVE/VERIFY TEST
// ================================================================================================

#[test]
fn fri_folding_2() {
    let trace_length_e = 12;
    let lde_blowup_e = 3;
    let folding_factor_e = 1;
    let max_remainder_size_e = 3;
    fri_prove_verify(
        trace_length_e,
        lde_blowup_e,
        folding_factor_e,
        max_remainder_size_e,
    )
}

#[test]
fn fri_folding_4() {
    let trace_length_e = 12;
    let lde_blowup_e = 3;
    let folding_factor_e = 2;
    let max_remainder_size_e = 8;
    fri_prove_verify(
        trace_length_e,
        lde_blowup_e,
        folding_factor_e,
        max_remainder_size_e,
    )
}

// TEST UTILS
// ================================================================================================

pub fn build_prover_channel(
    trace_length: usize,
    options: &FriOptions,
) -> DefaultProverChannel<BaseElement, BaseElement, Blake2s> {
    DefaultProverChannel::new(trace_length * options.blowup_factor(), 32)
}

pub fn build_evaluations(trace_length: usize, lde_blowup: usize) -> Vec<BaseElement> {
    let mut p = (0..trace_length as u128)
        .map(BaseElement::new)
        .collect::<Vec<_>>();
    let domain_size = trace_length * lde_blowup;
    p.resize(domain_size, BaseElement::ZERO);

    let twiddles = fft::get_twiddles::<BaseElement>(domain_size);

    fft::evaluate_poly(&mut p, &twiddles);
    p
}

pub fn verify_proof(
    proof: FriProof,
    commitments: Vec<<Blake2s as Hasher>::Digest>,
    evaluations: &[BaseElement],
    max_degree: usize,
    domain_size: usize,
    positions: &[usize],
    options: &FriOptions,
) -> Result<(), VerifierError> {
    // test proof serialization / deserialization
    let mut proof_bytes = Vec::new();
    proof.write_into(&mut proof_bytes);

    let mut reader = SliceReader::new(&proof_bytes);
    let proof = FriProof::read_from(&mut reader).unwrap();

    // verify the proof
    let mut channel = DefaultVerifierChannel::<BaseElement, Blake2s>::new(
        proof,
        commitments,
        domain_size,
        options.folding_factor(),
    )
    .unwrap();
    let mut coin = RandomCoin::<BaseElement, Blake2s>::new(&[]);
    let verifier = FriVerifier::new(&mut channel, &mut coin, options.clone(), max_degree).unwrap();
    let queried_evaluations = positions
        .iter()
        .map(|&p| evaluations[p])
        .collect::<Vec<_>>();
    verifier.verify(&mut channel, &queried_evaluations, &positions)
}
