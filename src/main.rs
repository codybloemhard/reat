use std::{
    env,
    path::Path,
    process::ExitCode,
    ffi::{ OsStr, OsString },
    fmt::Display,
};

use zen_colour::*;

fn main() -> ExitCode {
    if !xattr::SUPPORTED_PLATFORM {
        println!("{BOLD}{RED}This platform does not support {DEFAULT}xattr{RED}.{RESET}");
        return ExitCode::FAILURE;
    }

    let mut verbose = false;
    let mut force = false;
    let mut stdin = false;
    let mut into_a = false;
    let mut mode = " ";
    let mut a = Vec::new();
    let mut b = Vec::new();

    for arg in env::args().skip(1) {
        if (arg == "verbose" || arg == "v") && !verbose {
            verbose = true;
        }
        else if (arg == "force" || arg == "f") && !force {
            force = true;
        }
        else if (arg == "stdin" || arg == "i") && !stdin {
            stdin = true;
        }
        else if arg == "-" {
            into_a = false;
        }
        else if (arg == "list" || arg == "l") && mode == " " {
            mode = "l";
            into_a = true;
        }
        else if (arg == "get" || arg == "g") && mode == " " {
            mode = "g";
            into_a = true;
        }
        else if (arg == "set" || arg == "s") && mode == " " {
            mode = "s";
            into_a = true;
        }
        else if (arg == "rem" || arg == "r") && mode == " " {
            mode = "r";
            into_a = true;
        }
        else if (arg == "add" || arg == "a") && mode == " " {
            mode = "a";
            into_a = true;
        }
        else if (arg == "cut" || arg == "c") && mode == " " {
            mode = "c";
            into_a = true;
        }
        else if (arg == "copy" || arg == "cp") && mode == " " {
            mode = "cp";
            into_a = true;
        }
        else if (arg == "contains" || arg == "cn") && mode == " " {
            mode = "cn";
            into_a = true;
        }
        else if (arg == "contains-all" || arg == "cna") && mode == " " {
            mode = "cna";
            into_a = true;
        }
        else if (arg == "contains-not" || arg == "cnn") && mode == " " {
            mode = "cnn";
            into_a = true;
        }
        else if into_a {
            a.push(arg);
        }
        else {
            b.push(arg);
        }
    }

    if mode == " " {
        mode = "l";
    }

    let mut stdin_refs = Vec::new();
    let mut ps = Vec::new();
    let mut nps = Vec::new();

    if stdin {
        let stdin = std::io::read_to_string(std::io::stdin());
        if let Ok(input) = stdin {
            for file in input.split('\n') {
                stdin_refs.push(file.trim().to_string());
            }
        }
        for fref in &stdin_refs[..] {
            ps.push(fref);
        }
    }

    match (mode, &a[..], &b[..]) {
        ("l" | "cp", apaths, bpaths) => {
            for path in apaths {
                ps.push(path);
            }
            for path in bpaths {
                ps.push(path);
            }
        },
        ("g" | "r", [att, paths @ ..], []) => {
            nps.push(att);
            for path in paths {
                ps.push(path);
            }
        },
        ("s" | "a" | "c" | "cn" | "cna" | "cnn", [att, val, paths @ ..], []) => {
            nps.push(att);
            nps.push(val);
            for path in paths {
                ps.push(path);
            }
        },
        (_, atts, paths) => {
            for att in atts {
                nps.push(att);
            }
            for path in paths {
                ps.push(path);
            }
        },
    }

    let no_path = || println!("{BOLD}{RED}No {YELLOW}path{RED} provided!{RESET}");

    match (mode, &nps[..], &ps[..]) {
        ("l", _, []) => no_path(),
        ("l", _, [path]) => print_list(path, false, verbose),
        ("l", _, paths) => for path in paths {
            print_list(path, true, verbose);
        },
        ("cp", _, []) => no_path(),
        ("cp", _, [_]) => println!("{BOLD}{RED}Need at least 2 {YELLOW}paths{RED}.{RESET}"),
        ("cp", _, [srcp, dstp]) => print_copy(srcp, dstp),
        ("cp", _, _) => println!("{BOLD}{RED}To many {YELLOW}paths{RED}.{RESET}"),
        ("g" | "r", [], []) => println!(
"{BOLD}{RED}No {YELLOW}path{RED} nor {YELLOW}attribute{RED} provided!{RESET}"
        ),
        ("g" | "r", [], [_]) => println!("{BOLD}{RED}No {YELLOW}attribute{RED} provided!{RESET}"),
        ("g" | "r", [_], []) => no_path(),
        ("g", [attr], [path]) => print_get(path, attr, false, verbose),
        ("g", attrs, paths) => for path in paths { for attr in attrs {
            print_get(path, attr, true, verbose);
        }},
        ("s" | "a" | "c" | "cn", [], []) => println!(
"{BOLD}{RED}No {YELLOW}path{RED} nor {YELLOW}attribute{RED} nor {YELLOW}value{RED} provided!{RESET}"
        ),
        ("s" | "a" | "c" | "cn", [], [_]) => println!(
"{BOLD}{RED}No {YELLOW}attribute{RED} nor {YELLOW}value{RED} provided!{RESET}"
        ),
        ("s" | "a" | "c", [_], []) => println!(
"{BOLD}{RED}No {YELLOW}path{RED} provided and missing {YELLOW}attribute{RED} or {YELLOW}value{RED}!{RESET}"
        ),
        ("s" | "a" | "c" | "cn", [_, _], []) => no_path(),
        ("s" | "a" | "c", [_], [_]) => println!(
"{BOLD}{RED}No {YELLOW}attribute{RED} or {YELLOW}value{RED} provided!{RESET}"
        ),
        ("s", [attr, value], [path]) => print_set(path, attr, value, false, force),
        ("s", [attrs @ .., value], [path]) => for attr in attrs {
            print_set(path, attr, value, false, force);
        },
        ("s", [attrs @ .., value], paths) => for path in paths { for attr in attrs {
            print_set(path, attr, value, true, force);
        }},
        ("r", [attr], [path]) => print_remove(path, attr, false, force),
        ("r", attrs, paths) => for path in paths { for attr in attrs {
            print_remove(path, attr, true, force);
        }},
        ("a", [attr, value], [path]) => print_add_list(path, attr, value, false),
        ("a", [attrs @ .., value], [path]) => for attr in attrs {
            print_add_list(path, attr, value, false);
        },
        ("a", [attrs @ .., value], paths) => for path in paths { for attr in attrs {
            print_add_list(path, attr, value, true);
        }},
        ("c", [attr, value], [path]) => print_cut_list(path, attr, value, false, verbose),
        ("c", [attrs @ .., value], [path]) => for attr in attrs {
            print_cut_list(path, attr, value, false, verbose);
        },
        ("c", [attrs @ .., value], paths) => for path in paths { for attr in attrs {
            print_cut_list(path, attr, value, true, verbose);
        }},
        ("cn", [_], []) => no_path(),
        ("cn", [attr], paths) => for path in paths {
            print_contains('o', attr, &[], path);
        },
        ("cn", [attr, values @ ..], paths) => for path in paths {
            print_contains('o', attr, values, path);
        },
        ("cna", [attr], paths) => for path in paths {
            print_contains('a', attr, &[], path);
        },
        ("cna", [attr, values @ ..], paths) => for path in paths {
            print_contains('a', attr, values, path);
        },
        ("cnn", [attr], paths) => for path in paths {
            print_contains('n', attr, &[], path);
        },
        ("cnn", [attr, values @ ..], paths) => for path in paths {
            print_contains('n', attr, values, path);
        },
        _ => { },
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
    user.sort();
    system.sort();
    trusted.sort();
    security.sort();
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

fn print_copy<P: AsRef<Path> + Display>(srcp: P, dstp: P) {
    let xattrs = if let Ok(xs) = xattr::list(&srcp) { xs }
    else {
        println!(
    "{BOLD}{GREEN}{srcp}{RESET}{RED}{BOLD}: could not {YELLOW}copy{RED} from attributes.{RESET}"
        );
        return;
    };
    let mut ok = true;
    for key in xattrs {
        let val = xattr::get(&srcp, &key);
        if let Ok(Some(val)) = val {
            if xattr::set(&dstp, &key, &val).is_err() {
                ok = false;
                println!(
        "{BOLD}{RED}Could not {YELLOW}set{RED} attribute {DEFAULT}{:?}{RED} on destination.{RESET}",
                    key
                );
            }
        }
    }
    if ok {
        println!("{BOLD}{GREEN}Successfully copied from source to destination.{RESET}");
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

fn print_set<P: AsRef<Path> + Display>(
    path: P, key: &str, value: &str, print_filename: bool, force: bool
) {
    if print_filename {
        print!("{BOLD}{GREEN}{path}{RESET}{GREEN}:{RESET} ");
    }
    if key == "tags" && !force {
        println!(
    "{BOLD}{RED}Could not {YELLOW}set{RED} {DEFAULT}tags{RED} without {YELLOW}force{RED}!{RESET}"
        );
        return;
    }
    match set(path, key, value) {
        Ok(Some(old)) => println!(
            "{GREEN}Attribute {DEFAULT}{key}{GREEN} {YELLOW}overwritten{GREEN} successfully.
  Old value was \"{RESET}{old}{GREEN}\".{RESET}"
        ),
        Ok(None) => println!(
            "{GREEN}Attribute {DEFAULT}{key}{GREEN} {YELLOW}set{GREEN} successfully.{RESET}"
        ),
        Err(_) => println!(
            "{BOLD}{RED}Could not {YELLOW}set{RED} attribute {DEFAULT}{key}{RED}.{RESET}"
        ),
    }
}

fn print_add_list<P: AsRef<Path> + Display>(path: P, key: &str, value: &str, print_filename: bool) {
    if print_filename {
        print!("{BOLD}{GREEN}{path}{RESET}{GREEN}:{RESET} ");
    }
    match add_list(path, key, value) {
        Ok(_) => println!(
            "{YELLOW}Added{GREEN} list item to {DEFAULT}{key}{GREEN} successfully.{RESET}"
        ),
        Err(_) => println!(
            "{BOLD}{RED}Could not {YELLOW}add{RED} to attribute {DEFAULT}{key}{RED}.{RESET}"
        ),
    }
}

fn add_list<P: AsRef<Path>>(path: P, key: &str, value: &str) -> Result<Option<String>, ()> {
    if let Some((_, old_value)) = get(&path, key) {
        if old_value.trim() == "" {
            set(path, key, value)
        } else {
            set(path, key, &(old_value + "," + value))
        }
    } else {
        set(path, key, value)
    }
}

fn set<P: AsRef<Path>>(path: P, key: &str, value: &str) -> Result<Option<String>, ()> {
    let old_val = if let Some((_, value)) = get(&path, key) { Some(value) } else { None };
    if set_raw(path, key, value) {
        Ok(old_val)
    } else {
        Err(())
    }
}

fn set_raw<P: AsRef<Path>>(path: P, key: &str, value: &str) -> bool {
    xattr::set(path, "user.".to_string() + key, &Vec::<u8>::from(value)).is_ok()
}

fn print_remove<P: AsRef<Path> + Display>(path: P, key: &str, print_filename: bool, force: bool) {
    if print_filename {
        print!("{BOLD}{GREEN}{path}{RESET}{GREEN}:{RESET} ");
    }
    if key == "tags" && !force {
        println!(
    "{BOLD}{RED}Could not {YELLOW}remove{RED} {DEFAULT}tags{RED} without {YELLOW}force{RED}!{RESET}"
        );
        return;
    }
    match remove(path, key) {
        Ok(Some(old)) => println!(
            "{GREEN}Attribute {DEFAULT}{key}{GREEN} {YELLOW}removed{GREEN} successfully.
  Old value was \"{RESET}{old}{GREEN}\".{RESET}"
        ),
        Ok(None) => println!(
            "{GREEN}Attribute {DEFAULT}{key}{GREEN} {YELLOW}removed{GREEN} successfully.{RESET}"
        ),
        Err(_) => println!(
            "{BOLD}{RED}Could not {YELLOW}remove{RED} attribute {DEFAULT}{key}{RED}.{RESET}"
        ),
    }
}

fn print_cut_list<P: AsRef<Path> + Display>(
    path: P, key: &str, value: &str, print_filename: bool, verbose: bool
) {
    let res = cut_list(&path, key, value);
    if print_filename && (res.is_some() || verbose) {
        print!("{BOLD}{GREEN}{path}{RESET}{GREEN}:{RESET} ");
    }
    match res {
        Some(true) => println!(
            "{GREEN}Successfully {YELLOW}cut{GREEN} {DEFAULT}{value}{GREEN} from {DEFAULT}{key}{GREEN}.{RESET}"
        ),
        Some(false) => println!(
            "{BOLD}{RED}Could not {YELLOW}cut{RED} {DEFAULT}{value}{RED} from {DEFAULT}{key}{RED}.{RESET}"
        ),
        None if verbose || !print_filename => println!(
            "{GREEN}No {YELLOW}cut{GREEN} required.{RESET}"
        ),
        None => { },
    }
}

fn cut_list<P: AsRef<Path>>(path: P, key: &str, value: &str) -> Option<bool> {
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

fn remove<P: AsRef<Path>>(path: P, key: &str) -> Result<Option<String>, ()> {
    let old_val = if let Some((_, value)) = get(&path, key) { Some(value) } else { None };
    if remove_raw(path, key) {
        Ok(old_val)
    } else {
        Err(())
    }
}

fn remove_raw<P: AsRef<Path>>(path: P, key: &str) -> bool {
    xattr::remove(path, "user.".to_string() + key).is_ok()
}


fn print_contains(mode: char, key: &str, values: &[&String], path: &str) {
    let blanket = values.is_empty();
    if let Some((_, avalue)) = get(path, key) {
        if blanket {
            println!("{path}");
        }
        let list = avalue.split(',').collect::<Vec<_>>();
        if mode == 'o' {
            'outer: for item in list {
                for value in values {
                    if item.contains(*value) {
                        println!("{path}");
                        break 'outer;
                    }
                }
            }
        } else if mode == 'a' {
            let mut ok = true;
            for value in values {
                let mut lok = false;
                for item in &list {
                    if item.contains(*value) {
                        lok = true;
                        break;
                    }
                }
                if !lok {
                    ok = false;
                    break;
                }
            }
            if ok {
                println!("{path}");
            }
        } else if mode == 'n' {
            let mut ok = true;
            'outer: for value in values {
                for item in &list {
                    if item.contains(*value) {
                        ok = false;
                        break 'outer;
                    }
                }
            }
            if ok {
                println!("{path}");
            }
        }
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
enum KeyType {
    User,
    System,
    Trusted,
    Security,
}

