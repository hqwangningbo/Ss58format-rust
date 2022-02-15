#![allow(non_snake_case)]
use base58::{FromBase58, ToBase58};
use blake2::{Blake2b512, Digest};

#[derive(Debug, PartialEq, Eq)]
pub enum ErrDefine {
    InvalidPublicKey = 1,
    InvalidSS58Address = 2,
}

pub fn public_key_to_ss58_address(public_key: &str, ss58_prefix: u8) -> Result<String, ErrDefine> {
    let raw_account_id_res = hex::decode(public_key);
    match raw_account_id_res {
        Ok(mut raw_account_id) => {
            if raw_account_id.len() == 32 {
                //SS58PRE + ss58 prefix + hex![public_key]
                let ss58pre = b"SS58PRE";
                let mut checksum_pre_image: Vec<u8> = Vec::with_capacity(64);
                checksum_pre_image.extend_from_slice(ss58pre);
                checksum_pre_image.push(ss58_prefix);
                checksum_pre_image.append(&mut raw_account_id.clone());
                //Blake2b hash
                let mut hasher = Blake2b512::new();
                hasher.update(checksum_pre_image.as_slice());
                let check_sum = hasher.finalize();
                //ss58_prefix + hex![public_key] + hash[0..2]
                let mut ss58_pre_image: Vec<u8> = Vec::with_capacity(64);
                ss58_pre_image.push(ss58_prefix);
                ss58_pre_image.append(&mut raw_account_id);
                ss58_pre_image.extend_from_slice(&check_sum[0..2]);
                //Base58是用于比特币（Bitcoin）中使用的一种独特的编码方式，主要用于产生Bitcoin的钱包地址
                Ok(ss58_pre_image[..].to_base58())
            } else {
                Err(ErrDefine::InvalidPublicKey)
            }
        }
        _ => Err(ErrDefine::InvalidPublicKey),
    }
}
pub fn ss58_address_to_public_key(ss58_address: &str) -> Result<String, ErrDefine> {
    let res = ss58_address.from_base58();
    match res {
        Ok(address_bytes) => {
            //address_bytes = ss58_prefix + hex![public_key] + hash[0..2]
            let len = address_bytes.len();
            if len == 35 {
                //hex![public_key]
                let hex_public_key = &address_bytes[1..33];
                let public_key = hex::encode(hex_public_key);
                Ok(public_key)
            } else {
                Err(ErrDefine::InvalidSS58Address)
            }
        }
        _ => Err(ErrDefine::InvalidSS58Address),
    }
}

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
