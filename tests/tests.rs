use dr_core::config::{default, load, Defaults, Profile};
use dr_core::error::Error;
use dr_core::secrets::{Mask, Secret};
use std::collections::HashMap;
use std::env;
use std::path::Path;
use url::Url;

#[test]
fn load_conf() {
    let path: &Path = Path::new("tests/.config/dr.ron");
    let res: Result<HashMap<String, Profile>, Error> = load(path);
    let _: HashMap<String, Profile> = res.expect("Should have been able to load conf");
}

#[test]
fn load_conf_default() {
    env::set_var(Defaults::Config.to_string(), "tests/.config/dr.ron");
    let res: Result<HashMap<String, Profile>, Error> = default();
    env::remove_var(Defaults::Config.to_string());
    let _: HashMap<String, Profile> = res.expect("Should have been able to load conf");
}

#[test]
fn load_profile() {
    let res: Result<Profile, Error> =
        Profile::load_with(Path::new("tests/.config/dr.ron"), "default");
    let _: Profile = res.expect("Should have loaded the profile");
}

#[test]
fn load_try_default() {
    env::set_var(Defaults::Config.to_string(), "tests/.config/dr.ron");
    env::set_var(Defaults::Profile.to_string(), "default");
    let res: Result<Profile, Error> = Profile::try_default();
    env::remove_var(Defaults::Config.to_string());
    env::remove_var(Defaults::Profile.to_string());
    let _: Profile = res.expect("Should have loaded DR_PROFILE from DR_CONFIG");
}

#[test]
fn profile_new() {
    let _ = Profile::new(
        "username",
        Secret::Token(Mask::from("token")),
        Url::parse("https://localhost:8081").expect("Should have been able to parse this test URL"),
    );
}

#[test]
fn profile_from_envars() {
    env::set_var(Defaults::Username.to_string(), "default");
    env::set_var(Defaults::Token.to_string(), "default");
    env::set_var(Defaults::Pwd.to_string(), "default");
    env::set_var(Defaults::Host.to_string(), "https://localhost:8081");
    env::set_var(Defaults::Port.to_string(), "8082");
    let res: Result<Profile, Error> = Profile::from_envars(None, None, None, None, None);
    env::remove_var(Defaults::Username.to_string());
    env::remove_var(Defaults::Token.to_string());
    env::remove_var(Defaults::Pwd.to_string());
    env::remove_var(Defaults::Host.to_string());
    env::remove_var(Defaults::Port.to_string());
    let _: Profile = res.expect("Should have been able to load from environment variables");
}

#[test]
fn profile_from_envars_selective() {
    env::set_var(Defaults::Username.to_string(), "default");
    env::set_var(Defaults::Token.to_string(), "default");
    env::set_var(Defaults::Pwd.to_string(), "default");
    env::set_var(Defaults::Host.to_string(), "https://localhost:8081");
    env::set_var(Defaults::Port.to_string(), "8082");
    let res: Result<Profile, Error> =
        Profile::from_envars(Some("username"), None, None, None, None);
    env::remove_var(Defaults::Username.to_string());
    env::remove_var(Defaults::Token.to_string());
    env::remove_var(Defaults::Pwd.to_string());
    env::remove_var(Defaults::Host.to_string());
    env::remove_var(Defaults::Port.to_string());
    let _: Profile = res.expect("Should have been able to load from environment variables");
}
