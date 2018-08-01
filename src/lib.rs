//! This crate provides validation of European Union *VAT IDs*, which are
//! (basically, even though there are exceptions) the company registration
//! numbers within the EU.
//! 
//! Please have a look at the function definitions for usage examples.
//! 
//! All countries which have Value Added Tax have VAT IDs, so not only
//! European Union states. This crate works with **EU VAT IDs only**.
//! 
//! We implement and pass all tests for the sample VIES IDs:
//! [http://ec.europa.eu/taxation_customs/vies/faq.html#item_11](http://ec.europa.eu/taxation_customs/vies/faq.html#item_11)
//! 
//! ## TODO
//! 
//! * Make regulax expressions more specific so they can check more state-specific things such are check digits.
//! * Add interface with VIES to provide, at which, an online verification of the existence of a VAT ID. **This may be a stand-alone crate, as it will need to pull in a few dependencies and I'd prefer this crate to stay light**.


#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;
extern crate regex;

use failure::Error;
use regex::Regex;
use std::collections::HashMap;

/// Struct containing VAT ID and some information about it
#[derive(Debug, PartialEq)]
pub struct VATID {
    pub state_iso    : String,
    pub local_vat_id : String
}

lazy_static! {
    static ref RX_GENERAL : Regex = Regex::new("^([A-Z]{2})(.+)$").unwrap(); // FIXME: limit char variants after country

    // Reference: http://ec.europa.eu/taxation_customs/vies/faq.html#item_11
    static ref RXS_COUNTRIES: HashMap<String, Regex> = {
        let mut m = HashMap::new();
        m.insert(String::from("AT"), Regex::new(r"^U[0-9]{8}$").unwrap());
        m.insert(String::from("BE"), Regex::new(r"^0[0-9]{9}$").unwrap());
        m.insert(String::from("BG"), Regex::new(r"^[0-9]{9,10}$").unwrap());
        m.insert(String::from("CY"), Regex::new(r"^[0-9]{8}[A-Za-z]$").unwrap());
        m.insert(String::from("CZ"), Regex::new(r"^[0-9]{8,10}$").unwrap());
        m.insert(String::from("DE"), Regex::new(r"^[0-9]{9}$").unwrap());
        m.insert(String::from("DK"), Regex::new(r"^[0-9]{2} ?[0-9]{2} ?[0-9]{2} ?[0-9]{2}$").unwrap());
        m.insert(String::from("EE"), Regex::new(r"^[0-9]{9}$").unwrap());
        m.insert(String::from("EL"), Regex::new(r"^[0-9]{9}$").unwrap());
        m.insert(String::from("ES"), Regex::new(r"^[A-Za-z0-9][0-9]{7}[A-Za-z0-9]$").unwrap());
        m.insert(String::from("FI"), Regex::new(r"^[0-9]{8}$").unwrap());
        m.insert(String::from("FR"), Regex::new(r"^[A-Za-z0-9]{2} ?[0-9]{9}$").unwrap());
        m.insert(String::from("GB"), Regex::new(r"^[0-9]{3} ?[0-9]{4} ?[0-9]{2}|[0-9]{3} ?[0-9]{4} ?[0-9]{2} ?[0-9]{3}|GD[0-9]{3}|HA[0-9]{3}$").unwrap());
        m.insert(String::from("HR"), Regex::new(r"^[0-9]{11}$").unwrap());
        m.insert(String::from("IT"), Regex::new(r"^[0-9]{11}$").unwrap());
        m.insert(String::from("HU"), Regex::new(r"^[0-9]{8}$").unwrap());
        m.insert(String::from("IE"), Regex::new(r"^[0-9][A-Za-z0-9\+\*][0-9]{5}[A-Za-z]{1,2}$").unwrap());
        m.insert(String::from("LT"), Regex::new(r"^[0-9]{9}|[0-9]{12}$").unwrap());
        m.insert(String::from("LU"), Regex::new(r"^[0-9]{8}$").unwrap());
        m.insert(String::from("LV"), Regex::new(r"^[0-9]{11}$").unwrap());
        m.insert(String::from("MT"), Regex::new(r"^[0-9]{8}$").unwrap());
        m.insert(String::from("NL"), Regex::new(r"^[0-9]{9}B[0-9]{2}$").unwrap());
        m.insert(String::from("PL"), Regex::new(r"^[0-9]{10}$").unwrap());
        m.insert(String::from("PT"), Regex::new(r"^[0-9]{9}$").unwrap());
        m.insert(String::from("RO"), Regex::new(r"^[0-9]{2,10}$").unwrap());
        m.insert(String::from("SE"), Regex::new(r"^[0-9]{12}$").unwrap());
        m.insert(String::from("SI"), Regex::new(r"^[0-9]{8}$").unwrap());
        m.insert(String::from("SK"), Regex::new(r"^[0-9]{10}$").unwrap());
        m
    };
}

/// Check if a VAT ID has a valid syntax. It's an offline check, so it doesn't guarantee the VAT ID exists.
/// 
/// The function is case insensitive and expects a VAT ID beginning with the 2-letter EU state ISO code.
///
/// # Examples
/// 
/// ```
/// if eu_vat_id::check("IT00400770939") == true {
///     println!("VAT ID is valid");
/// }
/// ```
pub fn check(vat_id: &str) -> bool {
    match parse(vat_id) {
        Ok(_vi)     => true,
        Err(_e)     => false
    }
}

/// Same as `check`, but expects a "local" VAT ID, with the state ISO code specified separately
///
/// # Examples
/// 
/// ```
/// if eu_vat_id::check_by_state("00400770939", "IT") == true {
///     println!("VAT ID is valid");
/// }
/// ```
pub fn check_by_state(local_vat_id: &str, state: &str) -> bool {
    check( &format!("{}{}", state, local_vat_id) )
}

/// Parses a VAT ID to see if it has a valid syntax. It's an offline check, so it doesn't guarantee the VAT ID exists.
/// In case of success the function returns a `VATID` struct, otherwise returns an `Error`.
/// 
/// The function is case insensitive
///
/// You should only use this function if you need to know why the VAT ID is not valid, otherwise please use `check()`.
///
/// # Examples
/// 
/// ```
/// match eu_vat_id::parse("IT00400770939") {
///     Ok(vi)      => println!("VAT ID is valid, state is {}", vi.state_iso),
///     Err(e)      => println!("VAT ID is invalid because: {:?}", e),    
/// }
/// ```
/// 
/// # Errors
/// 
/// * *invalid-base-structure* - VAT ID is not 2 letters + ??
/// * *invalid-state* - not an EU state
/// * *invalid-local_vat_id* - the VAT ID structure is not valid for the state specified
pub fn parse(vat_id : &str) -> Result<VATID, Error> {
    let (state_iso, local_vat_id) = match RX_GENERAL.captures(&vat_id.to_uppercase()) {
        Some(cp)    => (
            String::from( cp.get(1).unwrap().as_str() ),
            String::from( cp.get(2).unwrap().as_str() )
        ),
        None        => bail!("invalid-base-structure")
    };

    let rx_local = match RXS_COUNTRIES.get(&state_iso) {
        Some(rx)    => rx,
        None        => bail!("invalid-state")
    };

    if !rx_local.is_match(&local_vat_id) {
        bail!("invalid-local_vat_id");
    }

    Ok(VATID {
        state_iso   : state_iso,
        local_vat_id: local_vat_id
    })
}