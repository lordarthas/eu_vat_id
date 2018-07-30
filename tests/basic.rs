#![cfg(test)]
extern crate eu_vat_id;

const TEST_IT_OK            : &str = "IT00400770939";
const TEST_IT_OK_STATE      : &str = "IT";
const TEST_IT_OK_LOCAL      : &str = "00400770939";
const TEST_INVALID          : &str = "3_4524DE";
const TEST_INVALID_STATE    : &str = "XX1234";
const TEST_INVALID_LOCAL    : &str = "IT00400";

#[test]
fn t_parse_ok() {
    assert_eq!(
        eu_vat_id::parse(TEST_IT_OK).unwrap(),
        eu_vat_id::VATID {
            state_iso       : String::from(TEST_IT_OK_STATE),
            local_vat_id    : String::from(TEST_IT_OK_LOCAL)
        }
    );
}

#[test]
fn t_parse_invalid() {
    assert_eq!(
        format!("{}",eu_vat_id::parse(TEST_INVALID).err().unwrap()),
        "invalid-base-structure"
    );
}

#[test]
fn t_parse_invalid_state() {
    assert_eq!(
        format!("{}",eu_vat_id::parse(TEST_INVALID_STATE).err().unwrap()),
        "invalid-state"
    );
}

#[test]
fn t_parse_invalid_local() {
    assert_eq!(
        format!("{}",eu_vat_id::parse(TEST_INVALID_LOCAL).err().unwrap()),
        "invalid-local_vat_id"
    );
}

#[test]
fn t_check_ok() {
    assert_eq!(eu_vat_id::check(TEST_IT_OK), true);
}

#[test]
fn t_check_by_state_ok() {
    assert_eq!(eu_vat_id::check_by_state(TEST_IT_OK_LOCAL, TEST_IT_OK_STATE), true);
}