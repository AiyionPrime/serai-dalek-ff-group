use std::io::Cursor;

use rand_core::{RngCore, CryptoRng};

use group::{ff::Field, Group};

use crate::{Curve, FrostCore, tests::core_gen};

// Test generation of FROST keys
fn key_generation<R: RngCore + CryptoRng, C: Curve>(rng: &mut R) {
  // This alone verifies the verification shares and group key are agreed upon as expected
  core_gen::<_, C>(rng);
}

// Test serialization of generated keys
fn keys_serialization<R: RngCore + CryptoRng, C: Curve>(rng: &mut R) {
  for (_, keys) in core_gen::<_, C>(rng) {
    assert_eq!(&FrostCore::<C>::deserialize(&mut Cursor::new(keys.serialize())).unwrap(), &keys);
  }
}

pub fn test_curve<R: RngCore + CryptoRng, C: Curve>(rng: &mut R) {
  // TODO: Test the Curve functions themselves

  // Test successful multiexp, with enough pairs to trigger its variety of algorithms
  // Multiexp has its own tests, yet only against k256 and Ed25519 (which should be sufficient
  // as-is to prove multiexp), and this doesn't hurt
  {
    let mut pairs = Vec::with_capacity(1000);
    let mut sum = C::G::identity();
    for _ in 0 .. 10 {
      for _ in 0 .. 100 {
        pairs.push((C::F::random(&mut *rng), C::GENERATOR * C::F::random(&mut *rng)));
        sum += pairs[pairs.len() - 1].1 * pairs[pairs.len() - 1].0;
      }
      assert_eq!(multiexp::multiexp(&pairs), sum);
      assert_eq!(multiexp::multiexp_vartime(&pairs), sum);
    }
  }

  // Test FROST key generation and serialization of FrostCore works as expected
  key_generation::<_, C>(rng);
  keys_serialization::<_, C>(rng);
}
