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

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static PRINCIPAL_TO_ISSUER: RefCell<StableBTreeMap<StorablePrincipal, Issuer, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );
}

#[query(name = "getIssuer")]
pub fn get_issuer(principal: Principal) -> Result<Issuer, String> {
    if let Some(issuer) = PRINCIPAL_TO_ISSUER.with(|p| p.borrow().get(&StorablePrincipal(principal))) {
        Ok(issuer)
    } else {
        Err(String::from("Issuer not found"))
    }
}

pub fn _set_issuer(principal: Principal, issuer: Issuer) -> Result<(), String> {
    PRINCIPAL_TO_ISSUER.with(|p| p.borrow_mut().insert(StorablePrincipal(principal), issuer));
    Ok(())
}