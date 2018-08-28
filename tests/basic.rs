#![cfg(test)]
extern crate eu_vat_id;

const TEST_IT_OK: &str = "IT00400770939";
const TEST_IT_OK_STATE: &str = "IT";
const TEST_IT_OK_LOCAL: &str = "00400770939";
const TEST_INVALID: &str = "3_4524DE";
const TEST_INVALID_STATE: &str = "XX1234";
const TEST_INVALID_LOCAL: &str = "IT00400";
const TEST_VIESFAQ_OK: &[&str] = &[
    "ATU99999999",
    "BE0999999999",
    "BG999999999",
    "BG9999999999",
    "CY99999999L",
    "CZ99999999",
    "CZ999999999",
    "CZ9999999999",
    "DE999999999",
    "DK99 99 99 99",
    "EE999999999",
    "EL999999999",
    "ESX9999999X",
    "FI99999999",
    "FRXX 999999999",
    "GB999 9999 99",
    "GB999 9999 99 999",
    "GBGD999",
    "GBHA999",
    "HR99999999999",
    "HU99999999",
    "IE9S99999L",
    "IE9999999WI",
    "IT99999999999",
    "LT999999999",
    "LT999999999999",
    "LU99999999",
    "LV99999999999",
    "MT99999999",
    "NL999999999B99",
    "PL9999999999",
    "PT999999999",
    "RO999999999",
    "SE999999999999",
    "SI99999999",
    "SK9999999999",
];

#[test]
fn t_parse_ok() {
    assert_eq!(
        eu_vat_id::parse(TEST_IT_OK).unwrap(),
        eu_vat_id::VATID {
            state_iso: String::from(TEST_IT_OK_STATE),
            local_vat_id: String::from(TEST_IT_OK_LOCAL)
        }
    );
}

#[test]
fn t_parse_invalid() {
    assert_eq!(
        format!("{}", eu_vat_id::parse(TEST_INVALID).err().unwrap()),
        "invalid-base-structure"
    );
}

#[test]
fn t_parse_invalid_state() {
    assert_eq!(
        format!("{}", eu_vat_id::parse(TEST_INVALID_STATE).err().unwrap()),
        "invalid-state"
    );
}

#[test]
fn t_parse_invalid_local() {
    assert_eq!(
        format!("{}", eu_vat_id::parse(TEST_INVALID_LOCAL).err().unwrap()),
        "invalid-local_vat_id"
    );
}

#[test]
fn t_check_ok() {
    assert_eq!(eu_vat_id::check(TEST_IT_OK), true);
}

#[test]
fn t_check_by_state_ok() {
    assert_eq!(
        eu_vat_id::check_by_state(TEST_IT_OK_LOCAL, TEST_IT_OK_STATE),
        true
    );
}

#[test]
fn t_check_ok_viesfaq() {
    for vq in TEST_VIESFAQ_OK.iter() {
        // println!("{}", vq);
        assert_eq!(eu_vat_id::check(vq), true);
    }
}
