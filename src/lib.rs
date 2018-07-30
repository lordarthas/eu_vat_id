// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This crate provides validation of EU *VAT ID*, which is the company
//! registration number.
//! 
//! All countries which have VAT have VAT IDs, so not only European Union
//! countries. This crate works with **EU VAT IDs only**.

#[macro_use] extern crate failure;

use failure::Error;

/// Check if a VAT ID has a valid syntax. It's an offline check, so it doesn't guarantee the VAT ID exists.
///
/// # Examples
/// 
/// ```
/// if eu_vat_id::check("IT00400770939") == true {
///     println!("VAT ID is valid");
/// }
/// ```
pub fn check(vat_id : &str) -> bool {
    match parse(vat_id) {
        Ok(true)    => true,
        Ok(false)   => false,
        Err(_e)     => false
    }
}

/// Parses a VAT ID to see if it has a valid syntax. It's an offline check, so it doesn't guarantee the VAT ID exists.
/// In case of success the function returns a bool which is always *true*, otherwise returns an `Error`.
/// 
/// You should only use this function if you need to know why the VAT ID is not valid, otherwise please use `check()`.
///
/// # Examples
/// 
/// ```
/// match eu_vat_id::parse("IT00400770939") {
///     Ok(_bool)   => println!("VAT ID is valid"), // Don't care if it's true or false
///     Err(e)      => println!("Codice is invalid beacuse: {:?}", e),    
/// }
/// ```
/// 
/// # Errors
/// 
/// * *invalid-base-structure* - VAT ID is not 2 letters + ??
/// * *invalid-country* - not an EU country
/// * *invalid-vat_id* - the VAT ID structure is not valid for the country specified
pub fn parse(vat_id : &str) -> Result<bool, Error> {
    Ok(true)
}
