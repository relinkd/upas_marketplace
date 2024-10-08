//! This module provides functions to manage the state of the indexer canister.

use candid::Principal;
use ic_stable_structures::{
    DefaultMemoryImpl, StableBTreeMap, StableVec, StableCell
};
use ic_cdk::{query, update};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager};
use std::cell::RefCell;
use crate::storable::{
    StorablePrincipal,
    Issuer,
    Memory
};
use crate::access::is_controller;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static PRINCIPAL_TO_ISSUER: RefCell<StableBTreeMap<StorablePrincipal, Issuer, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );
}

/// Retrieves the issuer associated with the given principal.
///
/// # Arguments
///
/// * `principal` - The principal for which to retrieve the issuer.
///
/// # Returns
///
/// * `Ok(Issuer)` if the issuer is found.
/// * `Err(String)` if the issuer is not found.
#[query(name = "getIssuer")]
pub fn get_issuer(principal: Principal) -> Result<Issuer, String> {
    if let Some(issuer) = PRINCIPAL_TO_ISSUER.with(|p| p.borrow().get(&StorablePrincipal(principal))) {
        Ok(issuer)
    } else {
        Err(String::from("Issuer not found"))
    }
}

/// Retrieves a batch of issuers starting from the given principal.
///
/// # Arguments
///
/// * `start_principal` - The principal from which to start the batch retrieval.
/// * `limit` - The maximum number of issuers to retrieve.
///
/// # Returns
///
/// * A vector of tuples containing the principal and issuer.
#[query(name = "getIssuersBatch")]
pub fn get_issuers_batch(start_principal: Option<Principal>, limit: usize) -> Vec<(Principal, Issuer)> {
    PRINCIPAL_TO_ISSUER.with(|p| {
        let map = p.borrow();
        let mut result = Vec::new();
        let mut count = 0;
        let mut started = start_principal.is_none();

        for (key, value) in map.iter() {
            if !started {
                if key.0 == start_principal.unwrap() {
                    started = true;
                } else {
                    continue;
                }
            }

            if count >= limit {
                break;
            }

            result.push((key.0, value.clone()));
            count += 1;
        }

        result
    })
}

/// Sets the issuer for the given principal.
///
/// # Arguments
///
/// * `principal` - The principal for which to set the issuer.
/// * `issuer` - The issuer to associate with the principal.
///
/// # Returns
///
/// * `Ok(())` if the issuer is set successfully.
/// * `Err(String)` if there is an error.
pub fn _set_issuer(principal: Principal, issuer: Issuer) -> Result<(), String> {
    PRINCIPAL_TO_ISSUER.with(|p| p.borrow_mut().insert(StorablePrincipal(principal), issuer));
    Ok(())
}

/// Removes the issuer associated with the given principal.
///
/// # Arguments
///
/// * `principal` - The principal for which to remove the issuer.
///
/// # Returns
///
/// * `Ok(())` if the issuer is removed successfully.
/// * `Err(String)` if there is an error or access is denied.
#[update(name = "removeIssuer")]
pub fn remove_issuer(principal: Principal) -> Result<(), String> {
    if(!is_controller()) {
        return Err(String::from("Access denied"));
    }
    PRINCIPAL_TO_ISSUER.with(|p| p.borrow_mut().remove(&StorablePrincipal(principal)));
    Ok(())
}