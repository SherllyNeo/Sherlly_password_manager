
use crate::PASSWORD_PATH;
use crate::encrypt::encrypt_text;
use crate::decrypt::decrypt_text;
use crate::password_file::get_file_as_byte_vec;
use crate::password_file::save;
use crate::input_wrapper::get_input;
use crate::make_db_safe::encode;


pub fn add_entry(title_dirty: String, username_dirty: String, password_dirty: String) {

println!("Please type your db password: \n");
let db_pass = get_input();
let title =  encode(title_dirty);
let username = encode(username_dirty);
let password =  encode(password_dirty);

//get and decrypt file
let ciphertextread = get_file_as_byte_vec(&PASSWORD_PATH.to_string());
let plaintext_str: String = decrypt_text(ciphertextread,db_pass.to_string()).replace("[", "").replace("]","");

let mut append_text = format!("{{\"title\":\"{}\", \"username\":\"{}\", \"password\":\"{}\" }}",title,username,password);
//make new entry and append
if plaintext_str == "" {

append_text = format!("{{\"title\":\"{}\", \"username\":\"{}\", \"password\":\"{}\" }}",title,username,password);
}
else {
append_text = format!("{}😀😀😀{{\"title\":\"{}\", \"username\":\"{}\", \"password\":\"{}\" }}",plaintext_str,title,username,password);
}
println!("{}",append_text);

let encrypted_new = encrypt_text(append_text,db_pass);
save(encrypted_new,PASSWORD_PATH);


}
