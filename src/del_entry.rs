
use crate::PASSWORD_PATH;
use crate::encrypt::encrypt_text;
use crate::decrypt::decrypt_text;
use crate::password_file::get_file_as_byte_vec;
use crate::password_file::save;
use crate::input_wrapper::get_input;
use crate::add_entry::add_entry_mass;
use crate::Entry;


//function to delete an entry, a little verbose. It parses and loads in the whole database, then deletes the database, filters out the user to be deleted and writes to the database again. Very inefficient but it is foolproof.
pub fn delete_entry(title: String) {

println!("Please type your db password: \n");
//get and decrypt file
let pass = get_input();
let ciphertextread = get_file_as_byte_vec(&PASSWORD_PATH.to_string());
let plaintext_str = decrypt_text(ciphertextread,pass.to_string());
let parse_pts: Vec<&str> = plaintext_str.split("😀😀😀").collect();
let vec_of_enteries: Vec<Entry> = parse_pts.into_iter().map(|entry| serde_json::from_str::<Entry>(&entry).expect("unable to parse json")).collect();


//get all files not equal to title
let output_user: Vec<Entry> = vec_of_enteries
        .into_iter()
        .filter(|ent| ent.title != title)
        .collect();


//blank db
let encrypted_nothing = encrypt_text("".to_string(),pass.to_string());
save(encrypted_nothing,&PASSWORD_PATH);


//add new entries
for entry in output_user {
    add_entry_mass(entry.title.to_string(),entry.username.to_string(),entry.password.to_string(),pass.to_string());
}

println!("deleted entry with title {} and refreshed database",title);

}
