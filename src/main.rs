use std::{
    env,
    path::Path,
    process::ExitCode,
    ffi::{ OsStr, OsString },
    fmt::Display,
    // collections::VecDeque,
};

use zen_colour::*;

fn main() -> ExitCode {
    if !xattr::SUPPORTED_PLATFORM {
        println!("{BOLD}{RED}This platform does not support {DEFAULT}xattr{RED}.{RESET}");
        return ExitCode::FAILURE;
    }

    let mut noncom = Vec::new();
    let mut verbose = false;
    let mut list = true;
    let mut get = true;

    for arg in env::args().skip(1) {
        if arg == "get" || arg == "g" {
            list = false;
            get = true;
        } else if arg == "verbose" || arg == "v" {
            verbose = true;
        }
        else if arg != "list" && arg != "l" {
            noncom.push(arg);
        }
    }

    if list {
        match &noncom[..] {
            [] => println!("{BOLD}{RED}No {YELLOW}path{RED} provided!{RESET}"),
            [path] => print_list(path, false, verbose),
            _ => for path in &noncom {
                print_list(path, true, verbose);
            },
        }
    } else if get {
        match &noncom[..] {
            [] => println!(
                "{BOLD}{RED}No {YELLOW}attribute{RED} and {YELLOW}path{RED} provided!{RESET}"
            ),
            [_] => println!(
                "{BOLD}{RED}No {YELLOW}attribute{RED} or {YELLOW}path{RED} provided!{RESET}"
            ),
            [attr, path] => print_get(path, attr, false, verbose),
            [attr, paths @ ..] => for path in paths {
                print_get(path, attr, true, verbose);
            },
        }
    }

    ExitCode::SUCCESS
}

fn print_list<P: AsRef<Path> + Display>(path: P, print_filename: bool, verbose: bool) {
    let xattrs = if let Ok(xs) = xattr::list(&path) { xs }
    else {
        println!(
            "{BOLD}{GREEN}{path}{RESET}{RED}{BOLD}: could not {YELLOW}list{RED} attributes.{RESET}"
        );
        return;
    };
    let mut user = Vec::new();
    let mut system = Vec::new();
    let mut trusted = Vec::new();
    let mut security = Vec::new();
    let mut empty = true;
    for attr in xattrs {
        empty = false;
        match get_osstr(&path, &attr) {
            Some(((key, KeyType::User), value)) => user.push((key, value)),
            Some(((key, KeyType::System), value)) => system.push((key, value)),
            Some(((key, KeyType::Trusted), value)) => trusted.push((key, value)),
            Some(((key, KeyType::Security), value)) => security.push((key, value)),
            None => { },
        }
    }
    if (print_filename || verbose) && !empty {
        println!("{BOLD}{GREEN}{path}{RESET}{GREEN}:{RESET}");
    } else if verbose && empty {
        println!("{BOLD}{GREEN}{path}{RESET}{GREEN}: {RED}{BOLD}❌{RESET}");
    }
    for (key, value) in user {
        println!("  {BOLD}{key}{RESET}: {value}");
    }
    for (key, value) in system {
        println!("  {MAGENTA}(system) {RESET}{BOLD}{key}{RESET}: {value}");
    }
    for (key, value) in trusted {
        println!("  {MAGENTA}(trusted) {RESET}{BOLD}{key}{RESET}: {value}");
    }
    for (key, value) in security {
        println!("  {MAGENTA}(security) {RESET}{BOLD}{key}{RESET}: {value}");
    }
}

fn print_get<P: AsRef<Path> + Display>(path: P, key: &str, print_filename: bool, verbose: bool) {
    if let Some(((key, ktype), value)) = get(&path, key) {
        if print_filename {
            print!("{BOLD}{GREEN}{path}{RESET}{GREEN}:{RESET} ");
        }
        match ktype {
            KeyType::User => { },
            KeyType::System => print!("{MAGENTA}(system) {RESET}"),
            KeyType::Trusted => print!("{MAGENTA}(trusted) {RESET}"),
            KeyType::Security => print!("{MAGENTA}(security) {RESET}"),
        }
        println!("{BOLD}{key}{RESET}: {value}");
    } else if !print_filename {
        println!("{BOLD}{RED}Could not {YELLOW}get{RED} attribute {DEFAULT}{key}{RED}.{RESET}");
    } else if verbose {
        println!("{BOLD}{GREEN}{path}{RESET}{GREEN}:{RESET}{BOLD}{key}{RESET}: {RED} ❌{RESET}");
    }
}

fn get<P: AsRef<Path>>(path: P, key: &str) -> Option<((String, KeyType), String)> {
    let mut osstr = OsString::from("user.");
    osstr.push(key);
    get_osstr(path, &osstr)
}

fn get_osstr<P: AsRef<Path>>(path: P, key: &OsStr) -> Option<((String, KeyType), String)> {
    if let Some(key) = key.to_str() {
        let val = xattr::get(path, key);
        if let Ok(Some(val)) = val {
            if let Ok(string) = String::from_utf8(val) {
                let (key, kt) = split_key(key);
                return Some(((key.to_string(), kt), string));
            }
        }
    }
    None
}

fn split_key(key: &str) -> (&str, KeyType) {
    if key.starts_with("user") { (&key[5..], KeyType::User) }
    else if key.starts_with("system") { (&key[7..], KeyType::System) }
    else if key.starts_with("trusted") { (&key[8..], KeyType::Trusted) }
    else if key.starts_with("security") { (&key[9..], KeyType::Security) }
    else { (key, KeyType::User) }
}

#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum KeyType {
    User,
    System,
    Trusted,
    Security,
}

