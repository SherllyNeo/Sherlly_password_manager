extern crate bincode;
extern crate serde;
extern crate dirs;
extern crate clipboard;
mod encrypt;
mod decrypt;
mod password_file;
mod input_wrapper;
mod show_pass;
mod make_db;
mod add_entry;
mod del_entry;
use encrypt::encrypt_text;
use decrypt::decrypt_text;
use password_file::save;
use password_file::get_file_as_byte_vec;
use serde_derive::Deserialize;
use serde_json::from_str;
use std::env;
use std::fs;
use simple_dmenu::dmenu;
use show_pass::get_entry;
use make_db::mk_db;
use add_entry::add_entry;
use del_entry::delete_entry;
use std::error::Error;
const PASSWORD_PATH: &str = "/home/sherlly/Passwords/password.db";


#[derive(Debug, Deserialize)]
struct Entry<'a> {
    title: &'a str,
    username: &'a str,
    password: &'a str
}
fn main() {

let args: Vec<String> = env::args().collect();
let mode = &args[1];

if mode == "--show_entries" {
get_entry();
}

else if mode == "--make_db" {
    mk_db();
}

else if mode == "--add_entry" {

let title = args[2].parse::<String>().unwrap();
let username = args[3].parse::<String>().unwrap();
let password = args[4].parse::<String>().unwrap();
add_entry(title,username,password);
}

else if mode == "--del_entry" {
let title = args[2].parse::<String>().unwrap();

delete_entry(title);
}
else {
    println!("please type in a mode in the format rust_pwd_manager --mode <options> where the modes are: show_entries, make_db, add_entry, del_entry");


}


}
