use crate::input_wrapper;
use crate::decrypt;
use crate::password_file;
use crate::PASSWORD_PATH;
use input_wrapper::get_input;
use decrypt::decrypt_text;
use password_file::get_file_as_byte_vec;
use serde_derive::Deserialize;
use serde_json::from_str;
use std::env;
use std::fs;
use simple_dmenu::dmenu;
use crate::Entry;
use std::{thread, time};
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

pub fn get_entry() {

println!("please type in your password: \n");
let pass = get_input();
let ciphertextread = get_file_as_byte_vec(&PASSWORD_PATH.to_string());
let plaintext_str = decrypt_text(ciphertextread,pass.to_string());
let parse_pts: Vec<&str> = plaintext_str.split("😀😀😀").collect();
let vec_of_enteries: Vec<Entry> = parse_pts.into_iter().map(|entry| serde_json::from_str::<Entry>(&entry).expect("unable to parse json")).collect();

let vec_of_usernames: Vec<&str> = vec_of_enteries.iter().map(|ent| ent.title).collect();
let output = dmenu!(iter vec_of_usernames);

let output_user: Vec<Entry> = vec_of_enteries
        .into_iter()
        .filter(|ent| ent.title == output)
        .collect();
let profile = &output_user[0];

let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();


println!("\n Copying password for title {} and username {} to clipboard for 20 seconds\n",profile.title,profile.username);
ctx.set_contents(profile.password.to_owned()).unwrap();
let twenty_seconds = time::Duration::from_secs(20);

thread::sleep(twenty_seconds);




}
