# Indexer Canister

The indexer canister is responsible for indexing and managing data within the system. It includes functionality for data retrieval, storage, and indexing operations.

## Modules

- `access`: Provides access control functions for checking if the caller is authorized and for retrieving the caller's principal ID.
- `storage`: Manages the storage of indexed data, including functions for adding, updating, and retrieving data.
- `indexing`: Contains the logic for indexing data, including functions for creating and managing indexes.
- `query`: Provides functions for querying indexed data, including search and retrieval operations.
- `utils`: Contains utility functions used across the indexer canister.

## Access Module

The `access` module provides functions to check if the caller is authorized to perform certain operations and to retrieve the caller's principal ID.

## Storage Module

The `storage` module manages the storage of indexed data. It includes functions to add new data, update existing data, and retrieve data from storage.

## Indexing Module

The `indexing` module contains the core logic for indexing data. It includes functions to create new indexes, update existing indexes, and manage the indexing process.

## Query Module

The `query` module provides functions for querying indexed data. It includes search functions to retrieve data based on various criteria and retrieval functions to get specific data entries.

## Utils Module

The `utils` module contains utility functions that are used across the indexer canister. These functions support various operations such as data formatting, validation, and other common tasks.