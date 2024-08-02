use candid::Principal;
use ic_cdk::{query, update};
use crate::storable::Issuer;
use crate::access::is_controller;
use crate::state::{
    get_issuer,
    _set_issuer
};

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

