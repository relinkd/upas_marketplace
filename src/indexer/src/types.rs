//! This module defines types used in the indexer canister.

use icrc_ledger_types::icrc::generic_metadata_value::MetadataValue;
use std::collections::HashMap;

pub type Icrc7TokenMetadata = HashMap<String, MetadataValue>;