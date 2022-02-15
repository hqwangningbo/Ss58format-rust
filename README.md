## Ss58format-rust
### function
* public_key_to_ss58_address(public_key: &str, ss58_prefix: u8)
* ss58_address_to_public_key(ss58_address: &str)
### run test
``
cargo test
``
```rust
#[test]
fn public_key_to_ss58_address_should_work() {
    let address = public_key_to_ss58_address(
        "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
        42,
    );
    assert_eq!(
        Ok(String::from(
            "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
        )),
        address
    );
}
#[test]
fn ss58_address_to_public_key_should_work() {
    let public_key = ss58_address_to_public_key("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");
    assert_eq!(
        Ok(String::from(
            "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
        )),
        public_key
    );
}
```