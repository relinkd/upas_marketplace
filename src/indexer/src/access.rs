//! This module provides access control functions for the indexer canister.

use ic_cdk::query;
use candid::Principal;

/// Returns the principal ID of the caller.
/// 
/// # Returns
/// 
/// * `Principal` - The principal ID of the caller.
#[query(name = "caller")]
pub fn caller() -> Principal {
    let id = ic_cdk::api::caller();

    return id;
}

/// Checks if the caller is a controller of the canister.
/// 
/// # Returns
/// 
/// * `bool` - `true` if the caller is a controller, `false` otherwise.
#[query(name = "isController")]
pub fn is_controller() -> bool {
    let id = ic_cdk::api::caller();
    let is_controller = ic_cdk::api::is_controller(&id);

    return is_controller;
}
