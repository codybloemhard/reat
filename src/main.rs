use std::path::Path;
use std::ffi::OsStr;
use zen_colour::*;

fn main() {
    print_list("./test");
}

fn print_list<P: AsRef<Path>>(path: P) {
    let xattrs = xattr::list(&path).unwrap();
    let mut user = Vec::new();
    let mut system = Vec::new();
    let mut trusted = Vec::new();
    let mut security = Vec::new();
    for attr in xattrs {
        match get(&path, &attr) {
            Some(((key, KeyType::User), value)) => user.push((key, value)),
            Some(((key, KeyType::System), value)) => system.push((key, value)),
            Some(((key, KeyType::Trusted), value)) => trusted.push((key, value)),
            Some(((key, KeyType::Security), value)) => security.push((key, value)),
            None => { },
        }
    }
    for (key, value) in user {
        println!("  {BOLD}{key}{RESET}: {value}");
    }
    for (key, value) in system {
        println!("  {MAGENTA}(system){RESET}{BOLD}{key}{RESET}: {value}");
    }
    for (key, value) in trusted {
        println!("  {MAGENTA}(trusted){RESET}{BOLD}{key}{RESET}: {value}");
    }
    for (key, value) in security {
        println!("  {MAGENTA}(security){RESET}{BOLD}{key}{RESET}: {value}");
    }
}

fn get<P: AsRef<Path>>(path: P, key: &OsStr) -> Option<((String, KeyType), String)> {
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

