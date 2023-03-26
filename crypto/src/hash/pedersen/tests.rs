
use super::{Pedersen_256, ElementHasher, Hasher};
use math::{fields::f252::BaseElement, FieldElement};
use rand_utils::rand_array;

#[test]
fn hash_padding() {
    let b1 = [1u8, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let b2 = [1u8, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
    let expected = [0x01u8, 0x71, 0x98, 0x60, 0xCE, 0x69, 0x3E, 0x55, 0xFC, 0x2D, 0x36, 0xB3, 0xF0, 0x89, 0xBE, 0x57,
                    0x76u8, 0x89, 0xC6, 0xA0, 0xC2, 0x3A, 0x72, 0x8A, 0x87, 0x44, 0x88, 0x43, 0x63, 0x94, 0x29, 0xB1];
    assert_eq!(r.0, expected);
}

#[test]
fn test_hash_elements() {
    let e = [BaseElement::from(1u8), BaseElement::from(0u8)];
    let r = Pedersen_256::<BaseElement>::hash_elements(&e);
    let expected = [0x01u8, 0x6F, 0xB9, 0x38, 0x83, 0xFC, 0x27, 0xC1, 0x73, 0x98, 0x95, 0xE7, 0xC5, 0xBC, 0xB0, 0x94,
                    0x2Fu8, 0x87, 0xD1, 0x10, 0x14, 0x93, 0x42, 0x46, 0xB1, 0xC3, 0x57, 0xCB, 0xB0, 0x52, 0x74, 0x4A];
    assert_eq!(r.0, expected);
}