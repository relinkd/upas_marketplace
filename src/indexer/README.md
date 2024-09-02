# Indexer Canister

The indexer canister is responsible for indexing and managing data within the system. It includes functionality for data retrieval, storage, and indexing operations.

## Modules

- `access`: Provides access control functions for checking if the caller is authorized and for retrieving the caller's principal ID.
- `storage`: Manages the storage of indexed data, including functions for adding, updating, and retrieving data.
- `indexing`: Contains the logic for indexing data, including functions for creating and managing indexes.
- `utils`: Contains utility functions used across the indexer canister.
- `state`: Manages the state of the indexer canister, including functions for saving and restoring state.

## Access Module

The `access` module provides functions to check if the caller is authorized to perform certain operations and to retrieve the caller's principal ID.

## Storage Module

The `storage` module manages the storage of indexed data. It includes functions to add new data, update existing data, and retrieve data from storage.

## Indexing Module

The `indexing` module contains the core logic for indexing data. It includes functions to create new indexes, update existing indexes, and manage the indexing process.

## Utils Module

The `utils` module contains utility functions that are used across the indexer canister. These functions support various operations such as data formatting, validation, and other common tasks.

## State Module

The `state` module manages the state of the indexer canister. It includes functions to save the current state and restore it when needed.