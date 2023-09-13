use aes::Aes256;
use block_modes::{block_padding::Pkcs7, BlockMode, Cbc};
use sha2::{Digest, Sha256};

type AesCbc = Cbc<Aes256, Pkcs7>;
const SALT: &str = "xxxxxxxxxxxxxxxxxx";

pub fn encrypt(password: &str, data: &str) -> String {
    let key = get_key(password);
    let iv = gen_iv();
    let cipher = AesCbc::new_from_slices(&key, &iv).unwrap();
    let result = cipher.encrypt_vec(data.as_bytes());
    let mut ivres: Vec<u8> = vec![];
    ivres.extend(iv);
    ivres.extend(result);
    base64::encode(ivres)
}

fn gen_iv() -> Vec<u8> {
    let mut res: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    getrandom::getrandom(&mut res).unwrap();
    res
}

fn get_key(password: &str) -> Vec<u8> {
    let pw: String = format!("{}::{}", password, SALT);
    let mut h = Sha256::new();
    h.update(pw.as_bytes());
    h.finalize().to_vec()
}

pub fn decrypt(password: &str, data: &str) -> String {
    let key = get_key(password);
    let bytes = base64::decode(data).unwrap();
    let iv = &bytes[..16];
    let cipher = AesCbc::new_from_slices(&key, iv).unwrap();
    let result = cipher.decrypt_vec(&bytes[16..]).unwrap();
    String::from_utf8(result).unwrap()
}

#[cfg(test)]
mod cipher_tests {
    use super::*;
    #[test]
    fn enc_dec_test() {
        let password = "abcd";
        let data = "穏やかな心は体に良い。";
        let enc = encrypt(password, data);
        println!("暗号化: {}", enc);
        let dec = decrypt(password, &enc);
        println!("復号化: {}", dec);
        assert_eq!(data, dec);
    }
}
