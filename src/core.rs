use std::{
    path::Path,
    ffi::{ OsStr, OsString },
};

pub fn get<P: AsRef<Path>>(path: P, key: &str) -> Option<((String, KeyType), String)> {
    let mut osstr = OsString::from("user.");
    osstr.push(key);
    get_osstr(path, &osstr)
}

pub fn get_osstr<P: AsRef<Path>>(path: P, key: &OsStr) -> Option<((String, KeyType), String)> {
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

pub fn add_list<P: AsRef<Path>>(path: P, key: &str, value: &str) -> Result<Option<String>, bool> {
    if let Some((_, old_value)) = get(&path, key) {
        if old_value.trim() == "" {
            set(path, key, value, false)
        } else {
            set(path, key, &(old_value + "," + value), false)
        }
    } else {
        set(path, key, value, false)
    }
}

// returns Err(true) if failed due to empty requirement.
// returns Err(false) if failed because of other reasons.
pub fn set<P: AsRef<Path>>(path: P, key: &str, value: &str, require_empty: bool)
 -> Result<Option<String>, bool>
{
    let old_val = if let Some((_, value)) = get(&path, key) { Some(value) } else { None };
    if require_empty && old_val.is_some() {
        Err(true)
    } else if set_raw(path, key, value) {
        Ok(old_val)
    } else {
        Err(false)
    }
}

pub fn set_raw<P: AsRef<Path>>(path: P, key: &str, value: &str) -> bool {
    xattr::set(path, "user.".to_string() + key, &Vec::<u8>::from(value)).is_ok()
}

pub fn cut_list<P: AsRef<Path>>(path: P, key: &str, value: &str) -> Option<bool> {
    if let Some((_, old_value)) = get(&path, key) {
        let mut list = old_value.split(',').collect::<Vec<_>>();
        let old_len = list.len();
        list.retain(|item| *item != value);
        let new_len = list.len();
        if old_len == new_len {
            return None;
        }
        let mut res = String::new();
        for item in list {
            res.push_str(item);
            res.push(',');
        }
        res.pop();
        if set_raw(path, key, &res) {
            Some(true)
        } else {
            Some(false)
        }
    } else {
        None
    }
}

pub fn remove<P: AsRef<Path>>(path: P, key: &str) -> Result<Option<String>, ()> {
    let old_val = if let Some((_, value)) = get(&path, key) { Some(value) } else { None };
    if remove_raw(path, key) {
        Ok(old_val)
    } else {
        Err(())
    }
}

pub fn remove_raw<P: AsRef<Path>>(path: P, key: &str) -> bool {
    xattr::remove(path, "user.".to_string() + key).is_ok()
}

pub fn replace_list<P: AsRef<Path>>(
    path: P, key: &str, old_value: &str, new_value: &str
) -> Option<bool> {
    if let Some((_, old_list)) = get(&path, key) {
        let mut list = old_list.split(',').collect::<Vec<_>>();
        if !list.contains(&old_value) {
            return None;
        }
        list.iter_mut().for_each(|item| if *item == old_value { *item = new_value; });
        let mut res = String::new();
        for item in list {
            res.push_str(item);
            res.push(',');
        }
        res.pop();
        if set_raw(path, key, &res) {
            Some(true)
        } else {
            Some(false)
        }
    } else {
        None
    }
}

fn split_key(key: &str) -> (&str, KeyType) {
    if key.starts_with("user") { (&key[5..], KeyType::User) }
    else if key.starts_with("system") { (&key[7..], KeyType::System) }
    else if key.starts_with("trusted") { (&key[8..], KeyType::Trusted) }
    else if key.starts_with("security") { (&key[9..], KeyType::Security) }
    else { (key, KeyType::User) }
}

#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum KeyType {
    User,
    System,
    Trusted,
    Security,
}

