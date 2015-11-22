extern crate hyper;
extern crate rustc_serialize;

use std::io::Read;
use std::env;

use hyper::Client;
use hyper::header::Connection;

use rustc_serialize::json::{self, Json};

static API_URL: &'static str = "https://slack.com/api/";
static TOKEN: &'static str = "YOUR_SLACK_TOKEN";

#[derive(Debug, RustcDecodable)]
pub struct Profile {
    real_name: Option<String>,
    email: Option<String>,
}

#[derive(Debug, RustcDecodable)]
pub struct User {
    id: String,
    name: String,
    presence: String,
    profile: Profile,
    tz: Option<String>
}

fn main() {
    match env::args().nth(1) {
        Some(username) => {
            get_details(username);
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
            format_user_details(user);
        }
        None => {
            println!("No user found with that name.");
            return;
        }
    };
}

fn get_user_details(username: String) -> Option<User> {
    let url = api_url("users.list".to_owned()) + "&presence=1";
    let client = Client::new();

    let mut res = client.get(&url)
        .header(Connection::close())
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    let json_body = Json::from_str(&body).unwrap();
    let json_object = json_body.as_object().unwrap();
    let users = json_object.get("members").unwrap();

    for user in users.as_array().unwrap() {
        let user: User = json::decode(&user.to_string()).unwrap();
        if user.name == username {
            return Some(user);
        }
    }
    None
}

fn format_user_details(user: User) {
    println!("{:?}", user);
    println!("{0: <10} {1: <20} {2: <20} {3: <20} {4: <10}",
             "Login", "Name", "Email", "Time zone", "Presence");

    println!("{0: <10} {1: <20} {2: <20} {3: <20} {4: <10}",
             user.name,
             user.profile.real_name.unwrap(),
             user.profile.email.unwrap(),
             user.tz.unwrap(),
             user.presence);
}
