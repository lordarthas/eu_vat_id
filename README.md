# eu_vat_id

Rust crate to validate European Union business VAT IDs.

## TODO

* Implement all tests as per http://ec.europa.eu/taxation_customs/vies/faq.html#item_11
* Make regulax expressions more specific so they can check more state-specific things such are check digits.
* Add interface with VIES to provide, at which, an online verification of the existence of a VAT ID.
