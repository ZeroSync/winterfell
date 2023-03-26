
use crate::hash::pedersen::pedersen_hash;
use super::{Pedersen_256, ElementHasher, Hasher};
use math::{fields::f252::BaseElement, FieldElement};
use rand_utils::rand_array;

#[test]
fn hash_padding() {
    let b1 = [1u8, 2, 3, 4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let b2 = [1u8, 2, 3, 4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
              0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    // adding a zero bytes at the end of a byte string should result in a different hash
    let r1 = Pedersen_256::<BaseElement>::hash(&b1);
    let r2 = Pedersen_256::<BaseElement>::hash(&b2);
    assert_ne!(r1, r2);
}

#[test]
fn hash_elements_padding() {
    let e1: [BaseElement; 2] = rand_array();
    let e2 = [e1[0], e1[1], BaseElement::ZERO];

    // adding a zero element at the end of a list of elements should result in a different hash
    let r1 = Pedersen_256::hash_elements(&e1);
    let r2 = Pedersen_256::hash_elements(&e2);
    assert_ne!(r1, r2);
}

#[test]
fn test_hash_pedersen() {
    // Set input to "abc"
    let e = [BaseElement::from(0x61626300u32)];

    let r = Pedersen_256::hash_elements(&e);

    // Cairo is testing against the same hash to ensure a ground truth
    let expected = [0xb1u8, 0x29, 0x94, 0x63, 0x43, 0x88, 0x44, 0x87, 0x8a, 0x72, 0x3a, 0xc2, 0xa0, 0xc6, 0x89, 0x76,
                    0x57u8, 0xbe, 0x89, 0xf0, 0xb3, 0x36, 0x2d, 0xfc, 0x55, 0x3e, 0x69, 0xce, 0x60, 0x98, 0x71, 0x01];
    assert_eq!(r.0, expected);
}

#[test]
fn test_hash_elements() {
    let e = [BaseElement::from(1u8), BaseElement::from(0u8)];
    let r = Pedersen_256::<BaseElement>::hash_elements(&e);
    let expected = [0x4a, 0x74, 0x52, 0xb0, 0xcb, 0x57, 0xc3, 0xb1, 0x46, 0x42, 0x93, 0x14, 0x10, 0xd1, 0x87, 0x2f,
                    0x94, 0xb0, 0xbc, 0xc5, 0xe7, 0x95, 0x98, 0x73, 0xc1, 0x27, 0xfc, 0x83, 0x38, 0xb9, 0x6f, 0x01];
    assert_eq!(r.0, expected);
}

#[test]
fn test_pedersen_hash() {
    let e = [0xFFu8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
             0xFFu8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x03];
    let digest = pedersen_hash(&e);
    println!("test_pedersen_hash {:?}", digest);
}