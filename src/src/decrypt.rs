use tindercrypt::cryptors::RingCryptor;
pub fn decrypt_text(text_enc: Vec<u8> ,passphrase: String) -> String {
    let pass_b = passphrase.as_bytes();
    let cryptor = RingCryptor::new();
    let plain_b = cryptor.open(pass_b, &text_enc).expect("unable to decrypt");
String::from_utf8(plain_b).expect("could not decode bytes")


}
