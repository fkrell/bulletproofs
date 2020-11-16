#![cfg_attr(feature = "docs", doc(include = "../../docs/r1cs-docs-example.md"))]

#[cfg_attr(feature = "docs", doc(include = "../../docs/cs-proof.md"))]
mod notes {}

mod constraint_system;
mod linear_combination;
mod metrics;
mod proof;
mod prover;
mod verifier;

pub use self::constraint_system::{
    ConstraintSystem, RandomizableConstraintSystem, RandomizedConstraintSystem,
};
pub use self::linear_combination::{LinearCombination, Variable};
pub use self::metrics::Metrics;
pub use self::proof::R1CSProof;
pub use self::prover::Prover;
pub use self::verifier::Verifier;
pub use self::verifier::verify_batch;

pub use crate::errors::R1CSError;
