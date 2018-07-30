#![cfg(test)]
extern crate eu_vat_id;

const TEST_IT_OK            : &str = "IT00400770939";

#[test]
fn t_check_ok() {
    assert_eq!(eu_vat_id::check(TEST_IT_OK), true);
}