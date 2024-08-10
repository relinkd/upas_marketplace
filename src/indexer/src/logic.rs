use candid::Principal;
use ic_cdk::{query, update};
use crate::storable::Issuer;
use crate::access::is_controller;
use icrc_ledger_types::icrc1::account::Account;
use crate::state::{
    get_issuer,
    _set_issuer,
    get_issuers_batch
};
use crate::types::Icrc7TokenMetadata;


#[update(name = "addUnverifiedIssuer")]
pub fn add_unverified_issuer(principal: Principal) -> Result<Issuer, String> {
    let is_issuer = get_issuer(principal);

    match is_issuer {
        Ok(_) => return Err(String::from("Issuer already exist")),
        Err(_) => {
            let issuer = Issuer {
                reputation_module: String::from("null"),
                issuer_type: String::from("null"),
                verified: false,
                name: String::from("null"),
                description: String::from("null")
            };

            _set_issuer(principal, issuer.clone())?;

            return Ok(issuer)
        },
    };
   
}

#[update(name = "verifyIssuer")]
pub fn verify_issuer(principal: Principal, issuer_type: String, metadata: (String, String, String)) -> Result<(), String> {
    let issuer = get_issuer(principal);

    match issuer {
        Ok(_) => {
            if(!is_controller()) {
                return Err(String::from("Access denied"));
            }

            _set_issuer(principal, Issuer {
                issuer_type: issuer_type,
                verified: true,
                reputation_module: metadata.0,
                name: metadata.1,
                description: metadata.2
            })?;

            return Ok(())
        },
        Err(_) => {
            return Err(String::from("Issuer not found"))
        },
    };
   
}

#[update(name = "getPrincipalAchievements")] 
pub async fn get_principal_achievements(principal: Principal) -> Result<Vec<(Principal, Vec<u128>)>, String> {
    let issuers = get_issuers_batch(None, 10);
    let mut collections_to_tokens: Vec<(Principal, Vec<u128>)> = vec![];

    for (key, value) in issuers {
        let owned: (Vec<u128>, ) = ic_cdk::call(key, "icrc7_balance_of", (&[Account {
            owner: principal,
            subaccount: None
        }],)).await.unwrap();

        collections_to_tokens.push((key, owned.0))
    }
    
    Ok(collections_to_tokens)
}

#[update(name = "getPrincipalAchievementsMetadata")] 
pub async fn get_principal_achievements_with_metadata(principal: Principal) -> Result<Vec<(Principal, Vec<Option<Icrc7TokenMetadata>>)>, String> {
    let tokens = get_principal_achievements(principal).await.unwrap();
    let mut collections_to_tokens_with_metadata: Vec<(Principal, Vec<Option<Icrc7TokenMetadata>>)> = vec![];

    for (key, value) in tokens {
        let metadata: (Vec<Option<Icrc7TokenMetadata>>, ) = ic_cdk::call(key, "icrc7_token_metadata", (value,)).await.unwrap();

        collections_to_tokens_with_metadata.push((key, metadata.0))
    }
    
    Ok(collections_to_tokens_with_metadata)
}