
use crate::PASSWORD_PATH;
use crate::encrypt::encrypt_text;
use crate::decrypt::decrypt_text;
use crate::password_file::get_file_as_byte_vec;
use crate::password_file::save;
use crate::input_wrapper::get_input;


pub fn delete_entry(title: String) {

println!("Please type your db password: \n");
//let db_pass = get_input();
//
////get and decrypt file
//let ciphertextread = get_file_as_byte_vec(&PASSWORD_PATH.to_string());
//let plaintext_str: String = decrypt_text(ciphertextread,db_pass.to_string());
//
////make new entry and append
//let appened_text = format!("{}😀😀😀{{\"title\":\"{}\", \"username\":\"{}\", \"password\":\"{}\" }}",plaintext_str,title,username,password);
//println!("{}",appened_text);
//
//let encrypted_new = encrypt_text(append_text,db_pass);
//save(encrypted_new,PASSWORD_PATH);


}
