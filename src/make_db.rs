use crate::encrypt::encrypt_text;
use crate::password_file::save;
use crate::PASSWORD_PATH;
use crate::input_wrapper::get_input;


pub fn mk_db() {
println!("please type in your password: \n");
let pass = get_input();
let encrypted_nothing = encrypt_text("".to_string(),pass.to_string());
save(encrypted_nothing,PASSWORD_PATH);
}
