use clap::Parser;
use std::process::exit;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The PAM service name to use
    #[clap(short, long, default_value_t = String::from("system-auth"))]
    service: String,
}

pub fn main() {
    let args = Args::parse();

    let user = std::env::var_os("username").expect("Missing username");
    let password = std::env::var_os("password").expect("Missing password");

    let mut auth = pam::Authenticator::with_password(args.service.as_str()).unwrap();
    auth.get_handler().set_credentials(
        user.to_str().expect("Unable to parse username"),
        password.to_str().expect("Unable to parse password"),
    );
    if auth.authenticate().is_ok() && auth.open_session().is_ok() {
        eprintln!("Successfully authenticated {}", user.to_str().unwrap());
        println!("name = {}", user.to_str().unwrap());
        exit(0);
    } else {
        eprintln!("Authentication failed for {}", user.to_str().unwrap());
        exit(1);
    }
}
