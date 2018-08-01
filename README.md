# eu_vat_id

This crate provides validation of European Union *VAT IDs*, which are
(basically, even though there are exceptions) the company registration
numbers within the EU.

Please have a look at the function definitions for usage examples.

All countries which have Value Added Tax have VAT IDs, so not only
European Union states. This crate works with **EU VAT IDs only**.

We implement and pass all tests for the sample VIES IDs:
[http://ec.europa.eu/taxation_customs/vies/faq.html#item_11](http://ec.europa.eu/taxation_customs/vies/faq.html#item_11)

### TODO

* Make regulax expressions more specific so they can check more state-specific things such are check digits.
* Add interface with VIES to provide, at which, an online verification of the existence of a VAT ID. *This may be a stand-alone crate, as it will need to pull in a few dependencies and I'd prefer this crate to stay light*.

License: MPL-2.0
