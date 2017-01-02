extern crate hyper;
extern crate rustc_serialize;
extern crate tabwriter;

use std::io::Read;
use std::env;
use std::process;

use hyper::Client;
use hyper::header::Connection;

use rustc_serialize::json::{self, Json};

use std::io::Write;
use tabwriter::TabWriter;

static API_URL: &'static str = "https://slack.com/api/";

#[derive(Debug, RustcDecodable)]
pub struct Profile {
    real_name: Option<String>,
    email: Option<String>,
    skype: Option<String>,
    phone: Option<String>,
    title: Option<String>
}

#[derive(Debug, RustcDecodable)]
pub struct User {
    id: String,
    name: String,
    presence: Option<String>,
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
    match env::var("SLACK_TOKEN") {
        Ok(token) => API_URL.to_string() + &method + "?token=" + &*token,
        Err(_) => {
            println!("Oops SLACK_TOKEN env variable is missing.
You can generate a token here https://api.slack.com/docs/oauth-test-tokens
then in your teminal run:
export SLACK_TOKEN=the_token_you_generated");
            process::exit(1)
        }
    }
}

fn get_details(username: String) {
    match get_user_details(username) {
        Some(user) => {
            format_user_details(user);
        }
        None => {
            println!("No user found with that name.");
            process::exit(1)
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

    match json_object.get("members") {
        None => {
            println!("Oops looks like your SLACK_TOKEN env variable is not okay.");
            process::exit(1)
        },
        Some(users) => {
            for user in users.as_array().unwrap() {
                let user: User = json::decode(&user.to_string()).unwrap();
                if user.name == username {
                    return Some(user);
                }
            }
            None
        }
    }
}

fn format_user_details(user: User) {
    let mut tw = TabWriter::new(Vec::new());
    write!(&mut tw, "
 Login\t Name\t Title\t Email\t Skype\t Phone\t Timezone\t Presence\t
 {}\t {}\t {}\t {}\t {}\t {}\t {}\t {}
 ",
           user.name,
           user.profile.real_name.unwrap(),
           user.profile.title.unwrap_or_else(|| "".to_owned()),
           user.profile.email.unwrap_or_else(|| "".to_owned()),
           user.profile.skype.unwrap_or_else(|| "".to_owned()),
           user.profile.phone.unwrap_or_else(|| "".to_owned()),
           user.tz.unwrap_or_else(|| "".to_owned()),
           user.presence.unwrap_or_else(|| "".to_owned())).unwrap();
    tw.flush().unwrap();

    let out_table = String::from_utf8(tw.unwrap()).unwrap();
    println!("{}", out_table);
}
