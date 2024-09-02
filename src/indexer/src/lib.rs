#![doc = include_str!("../README.md")]

use ic_cdk_macros::export_candid;
use candid::Principal;

pub mod access;
pub mod state;
pub mod storable;
pub mod logic;
pub mod types;

use storable::*;
use types::*;

export_candid!();
