use soroban_sdk::contracterror;

/// Errors returned by the escrow contract.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    /// The caller is not authorized to perform this action.
    Unauthorized = 1,
    /// The escrow has already been funded.
    AlreadyFunded = 2,
    /// The escrow has not been funded yet.
    NotFunded = 3,
    /// The escrow has already been resolved.
    AlreadyResolved = 4,
    /// The provided amount is invalid.
    InvalidAmount = 5,
    /// The provided resolution is invalid.
    InvalidResolution = 6,
    /// The dispute has not been raised.
    NoDispute = 7,
    /// The escrow is not in a state that allows this action.
    InvalidState = 8,
}
