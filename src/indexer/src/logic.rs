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
pub async fn add_unverified_issuer(principal: Principal) -> Result<Issuer, String> {
    let is_issuer = get_issuer(principal);

    if(!is_controller()) {
        return Err(String::from("Access denied"));
    }

    match is_issuer {
        Ok(_) => return Err(String::from("Issuer already exist")),
        Err(_) => {

            let minter: (Principal, ) = ic_cdk::call(principal, "getMinter", (&[Account {
                owner: principal,
                subaccount: None
            }],)).await.unwrap();
            
            let issuer = Issuer {
                reputation_module: minter.0.to_string(),
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
        let balance: Result<(Vec<u128>, ) , _>= ic_cdk::call(key, "icrc7_balance_of", (&[Account {
            owner: principal,
            subaccount: None
        }],)).await;

        let mut balance_result: Vec<u128>;

        match balance {
            Ok(ok) => {
                balance_result = ok.0
            }
            Err(_) => {
                continue;
            }
        }

        if(balance_result.get(0).unwrap() == &0_u128) {continue};

        let owned: (Vec<u128>, ) = ic_cdk::call(key, "icrc7_tokens_of", (Account {
            owner: principal,
            subaccount: None
        }, None::<u128>, balance_result.get(0).unwrap() - 1,)).await.unwrap();

        collections_to_tokens.push((key, owned.0))
    }
    
    Ok(collections_to_tokens)
}

#[update(name = "getPrincipalAchievementsMetadata")] 
pub async fn get_principal_achievements_with_metadata(principal: Principal) -> Result<Vec<(Principal, Vec<Icrc7TokenMetadata>)>, String> {
    let tokens = get_principal_achievements(principal).await.unwrap();
    let mut collections_to_tokens_with_metadata: Vec<(Principal, Vec<Icrc7TokenMetadata>)> = vec![];

    for (key, value) in tokens {
        let metadata: (Vec<Option<Icrc7TokenMetadata>>, ) = ic_cdk::call(key, "icrc7_token_metadata", (value,)).await.unwrap();

        let metadata_some: Vec<Icrc7TokenMetadata> = metadata.0.into_iter().filter_map(|x| x).collect();

        if(metadata_some.len() > 0) {
            collections_to_tokens_with_metadata.push((key, metadata_some))
        }
    }
    
    Ok(collections_to_tokens_with_metadata)
}