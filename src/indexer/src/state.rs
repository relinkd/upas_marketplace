use ic_stable_structures::{
    DefaultMemoryImpl, StableBTreeMap, StableVec, StableCell
};
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