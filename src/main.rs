extern crate hyper;
extern crate rustc_serialize;

use std::io::Read;
use std::env;

use hyper::Client;
use hyper::header::Connection;

use rustc_serialize::json::{self, Json};

static API_URL: &'static str = "https://slack.com/api/";
static TOKEN: &'static str = "";

#[derive(Debug, RustcDecodable)]
pub struct Profile {
    real_name: Option<String>,
    email: Option<String>,
}

#[derive(Debug, RustcDecodable)]
pub struct Member {
    id: String,
    name: String,
    presence: String,
    profile: Profile,
    tz: Option<String>
}

fn main() {
    match env::args().nth(1) {
        Some(username) => {
            let response = get_details(username);
            println!("- finished {:?}", response);
        }
        None => {
            println!("Usage: fingers <username>");
            return;
        }
    };
}

fn api_url(method: String) -> String {
    API_URL.to_string() + &method + "?token=" + TOKEN
}

fn get_details(username: String) {
    match get_user_details(username) {
        Some(user) => {
            println!("{:?}", user);
        }
        None => {
            println!("No user found with that name.");
            return;
        }
    };
}

fn get_user_details(username: String) -> Option<Member> {
    let url = api_url("users.list".to_owned()) + "&presence=1";
    let client = Client::new();

    let mut res = client.get(&url)
        .header(Connection::close())
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    let json_body = Json::from_str(&body).unwrap();
    let json_object = json_body.as_object().unwrap();
    let members = json_object.get("members").unwrap();

    for member in members.as_array().unwrap() {
        let member: Member = json::decode(&member.to_string()).unwrap();
        if member.name == username {
            return Some(member);
        }
    }
    None
}
