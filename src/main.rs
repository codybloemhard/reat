use std::{
    env,
    path::Path,
    process::ExitCode,
    ffi::{ OsStr, OsString },
    fmt::Display,
    collections::HashSet,
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
        else if (arg == "clear" || arg == "cl") && mode == " " {
            mode = "cl";
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
        else if (arg == "rename" || arg == "rn") && mode == " " {
            mode = "rn";
            into_a = true;
        }
        else if (arg == "replace" || arg == "rp") && mode == " " {
            mode = "rp";
            into_a = true;
        }
        else if (arg == "dump" || arg == "d") && mode == " " {
            mode = "d";
            into_a = true;
        }
        else if (arg == "restore" || arg == "rs") && mode == " " {
            mode = "rs";
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
    let mut dump = String::new();

    if stdin {
        let stdin = std::io::read_to_string(std::io::stdin());
        if let Ok(input) = stdin {
            for file in input.split('\n') {
                stdin_refs.push(file.trim().to_string());
            }
        }
        for fref in &stdin_refs[..] {
            if !fref.is_empty() {
                ps.push(fref);
            }
        }
    } else if mode == "rs" {
        let stdin = std::io::read_to_string(std::io::stdin());
        if let Ok(input) = stdin {
            dump = input;
        } else {
            println!("{BOLD}{RED} restore data could not be read into stdin!");
            return ExitCode::FAILURE;
        }
    }

    match (mode, &a[..], &b[..]) {
        ("l" | "cp" | "d" | "rs" | "cl", apaths, bpaths) => {
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
        ("s" | "a" | "c" | "cn" | "cna" | "cnn" | "rn" | "rp", [att, val, paths @ ..], []) => {
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
        ("l" | "d" | "cp" | "rs", _, []) => no_path(),
        ("l", _, paths) => for path in paths {
            print_list(path, paths.len() > 1, verbose);
        },
        ("d", _, paths) => for path in paths {
            print_dump(path);
        },
        ("rs", _, paths) => {
            print_restore(dump, paths, verbose, force);
        },
        ("cl", _, paths) => for path in paths {
            print_clear(path, paths.len() > 1, verbose, force);
        },
        ("cp", _, [_]) => println!("{BOLD}{RED}Need at least 2 {YELLOW}paths{RED}.{RESET}"),
        ("cp", _, [srcp, dstp]) => print_copy(srcp, dstp),
        ("cp", _, _) => println!("{BOLD}{RED}To many {YELLOW}paths{RED}.{RESET}"),
        ("g" | "r", [], []) => println!(
"{BOLD}{RED}No {YELLOW}path{RED} nor {YELLOW}attribute{RED} provided!{RESET}"
        ),
        ("g" | "r", [], [_]) => println!("{BOLD}{RED}No {YELLOW}attribute{RED} provided!{RESET}"),
        ("g" | "r", [_], []) => no_path(),
        ("g", attrs, paths) => for path in paths { for attr in attrs {
            print_get(path, attr, paths.len() > 1, verbose);
        }},
        ("s" | "a" | "c" | "cn" | "cna" | "cnn" | "rn", [], []) => println!(
"{BOLD}{RED}No {YELLOW}path{RED} nor {YELLOW}attribute{RED} nor {YELLOW}value{RED} provided!{RESET}"
        ),
        ("rp", [], []) => println!(
"{BOLD}{RED}No {YELLOW}path{RED} nor {YELLOW}attribute{RED} nor {YELLOW}values{RED} provided!{RESET}"
        ),
        ("s" | "a" | "c" | "cn" | "cna" | "cnn" | "rn", [], [_]) => println!(
"{BOLD}{RED}No {YELLOW}attribute{RED} nor {YELLOW}value{RED} provided!{RESET}"
        ),
        ("rp", [], [_]) => println!(
"{BOLD}{RED}No {YELLOW}attribute{RED} nor {YELLOW}values{RED} provided!{RESET}"
        ),
        ("rp", [_], [_]) => println!(
"{BOLD}{RED}Missing an {YELLOW}attribute{RED} and a {YELLOW}value{RED} or two {YELLOW}values{RED}!{RESET}"
        ),
        ("rp", [_, _], [_]) => println!(
"{BOLD}{RED}Missing an {YELLOW}attribute{RED} or a {YELLOW}value{RED}!{RESET}"
        ),
        ("s" | "a" | "c", [_], []) => println!(
"{BOLD}{RED}No {YELLOW}path{RED} provided and missing {YELLOW}attribute{RED} or {YELLOW}value{RED}!{RESET}"
        ),
        ("s" | "a" | "c" | "cn" | "cna" | "cnn" | "rn" | "rp", [_, _], []) => no_path(),
        ("s" | "a" | "c", [_], [_]) => println!(
"{BOLD}{RED}No {YELLOW}attribute{RED} or {YELLOW}value{RED} provided!{RESET}"
        ),
        ("s", [attrs @ .., value], paths) => for path in paths { for attr in attrs {
            print_set(path, attr, value, paths.len() > 1, force);
        }},
        ("r", attrs, paths) => for path in paths { for attr in attrs {
            print_remove(path, attr, paths.len() > 1, force);
        }},
        ("a", [attrs @ .., value], paths) => for path in paths { for attr in attrs {
            print_add_list(path, attr, value, paths.len() > 1);
        }},
        ("c", [attrs @ .., value], paths) => for path in paths { for attr in attrs {
            print_cut_list(path, attr, value, paths.len() > 1, verbose);
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
        ("rn", [attrs @ .., value], paths) => for path in paths { for attr in attrs {
            print_rename(path, attr, value, paths.len() > 1, force);
        }},
        ("rp", [attrs @ .., old_val, new_val], paths) => for path in paths { for attr in attrs {
            print_replace(path, attr, old_val, new_val, paths.len() > 1, verbose);
        }},
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


fn print_dump<P: AsRef<Path> + Display>(path: P) {
    let xattrs = if let Ok(xs) = xattr::list(&path) { xs }
    else {
        println!("{path}\nfail");
        return;
    };
    if xattrs.clone().next().is_none() {
        return;
    }
    println!("{path}");
    let mut list = Vec::new();
    let mut dropped = 0;
    for attr in xattrs {
        match get_osstr(&path, &attr) {
            Some(((key, KeyType::User), value)) => list.push((key, value)),
            Some(((_, KeyType::System), _)) => dropped += 1,
            Some(((_, KeyType::Trusted), _)) => dropped += 1,
            Some(((_, KeyType::Security), _)) => dropped += 1,
            None => { },
        }
    }
    for (key, val) in &list {
        print!("{} ", key.chars().filter(|c| *c == '\n').count() + 1);
        print!("{} ", val.chars().filter(|c| *c == '\n').count() + 1);
    }
    println!("{dropped}");
    for (key, val) in list {
        println!("{key}");
        println!("{val}");
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
    match set(path, key, value, key == "tags" && !force) {
        Ok(Some(old)) => println!(
            "{GREEN}Attribute {DEFAULT}{key}{GREEN} {YELLOW}overwritten{GREEN} successfully.
  Old value was \"{RESET}{old}{GREEN}\".{RESET}"
        ),
        Ok(None) => println!(
            "{GREEN}Attribute {DEFAULT}{key}{GREEN} {YELLOW}set{GREEN} successfully.{RESET}"
        ),
        Err(true) => println!(
    "{BOLD}{RED}Could not {YELLOW}set{RED} {DEFAULT}tags{RED} without {YELLOW}force{RED}!{RESET}"
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

fn add_list<P: AsRef<Path>>(path: P, key: &str, value: &str) -> Result<Option<String>, bool> {
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
fn set<P: AsRef<Path>>(path: P, key: &str, value: &str, require_empty: bool)
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

fn print_clear<P: AsRef<Path> + Display>(
    path: P, print_filename: bool, verbose: bool, force: bool
) {
    let fn_msg = format!("{BOLD}{GREEN}{path}{RESET}{GREEN}:{RESET}");
    let xattrs = if let Ok(xs) = xattr::list(&path) { xs }
    else {
        if print_filename { print!("{fn_msg} "); }
        println!("{BOLD}{RED}Could not {YELLOW}clear{RED} attributes.{RESET}");
        return;
    };
    if xattrs.clone().next().is_none() {
        if verbose {
            println!("{fn_msg} {RED}{BOLD}❌{RESET}");
        }
        return;
    }
    let mut list = Vec::new();
    let mut printed_fn = false;
    for attr in xattrs {
        match get_osstr(&path, &attr) {
            Some(((key, KeyType::User), _)) => list.push(key),
            Some(_) if verbose && print_filename => {
                if printed_fn {
                    println!("{fn_msg}");
                    printed_fn = true;
                }
                println!(
                    "  {RED}{BOLD}cannot {YELLOW}clear{RED} non user attribute!{RESET}"
                );
            },
            _ => { },
        }
    }
    for key in list {
        let tags_protected = key == "tags" && !force;
        if print_filename && tags_protected && !printed_fn {
            println!("{fn_msg}  ");
            printed_fn = true;
        }
        if tags_protected {
            println!(
"  {BOLD}{RED}Could not {YELLOW}remove{RED} {DEFAULT}tags{RED} without {YELLOW}force{RED}!{RESET}"
            );
            continue;
        }
        let res = remove(&path, &key);
        if (res.is_err() || verbose) && !printed_fn && print_filename {
            println!("{fn_msg}");
            printed_fn = true;
        }
        match res {
            Ok(Some(old)) if verbose => println!(
                "  {GREEN}Attribute {DEFAULT}{key}{GREEN} {YELLOW}removed{GREEN} successfully.
    Old value was \"{RESET}{old}{GREEN}\".{RESET}"
            ),
            Ok(None) if verbose => println!(
                "{GREEN}Attribute {DEFAULT}{key}{GREEN} {YELLOW}removed{GREEN} successfully.{RESET}"
            ),
            Err(_) => println!(
                "{BOLD}{RED}Could not {YELLOW}remove{RED} attribute {DEFAULT}{key}{RED}.{RESET}"
            ),
            _ => { },
        }
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
        let list = avalue.split(',').collect::<Vec<_>>();
        if blanket && (mode == 'o' || mode == 'a') {
            println!("{path}");
        } else if mode == 'o' {
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
        } else if mode == 'n' && !blanket {
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
    } else if blanket && mode == 'n' {
        println!("{path}");
    }
}

fn print_rename<P: AsRef<Path> + Display>(
    path: P, old_att_name: &str, new_att_name: &str, print_filename: bool, force: bool
) {
    if print_filename {
        print!("{BOLD}{GREEN}{path}{RESET}{GREEN}:{RESET} ");
    }
    if let Some((_, value)) = get(&path, old_att_name) {
        match set(&path, new_att_name, &value, !force) {
            Ok(Some(old_val)) => println!(
                "{GREEN}Old value was \"{RESET}{old_val}{GREEN}\".{RESET}"
            ),
            Ok(None) => { },
            Err(true) => {
                println!(
"{BOLD}{RED}Could not {YELLOW}set{RED} {DEFAULT}{new_att_name}{RED} without {YELLOW}force{RED}!{RESET}"
                );
                return;
            },
            Err(_) => {
                println!(
            "{BOLD}{RED}Could not {YELLOW}set{RED} attribute {DEFAULT}{new_att_name}{RED}.{RESET}"
                );
                return;
            },
        }
        if remove(&path, old_att_name).is_err() {
            println!(
        "{BOLD}{RED}Could not {YELLOW}remove{RED} attribute {DEFAULT}{old_att_name}{RED}.{RESET}"
            );
        } else {
            println!(
"{GREEN}Successfully {YELLOW}renamed{GREEN} attribute {DEFAULT}{old_att_name}{GREEN} to {DEFAULT}{new_att_name}{GREEN}.{RESET}"
            );
        }
    } else {
        println!(
            "{BOLD}{RED}Could not {YELLOW}get{RED} attribute {DEFAULT}{old_att_name}{RED}.{RESET}"
        );
    }
}

fn print_replace<P: AsRef<Path> + Display>(
    path: P, key: &str, old_val_name: &str, new_val_name: &str, print_filename: bool, verbose: bool
) {
    let res = replace_list(&path, key, old_val_name, new_val_name);
    if print_filename && (res.is_some() || verbose) {
        print!("{BOLD}{GREEN}{path}{RESET}{GREEN}:{RESET} ");
    }
    match res {
        Some(true) => println!(
            "{GREEN}Successfully {YELLOW}replaced{GREEN} {DEFAULT}{old_val_name}{GREEN} with {DEFAULT}{new_val_name}{GREEN} from {DEFAULT}{key}{GREEN}.{RESET}"
        ),
        Some(false) => println!(
            "{BOLD}{RED}Could not {YELLOW}replace{RED} {DEFAULT}{old_val_name}{RED} from {DEFAULT}{key}{RED}.{RESET}"
        ),
        None if verbose || !print_filename => println!(
            "{GREEN}No {YELLOW}replacement{GREEN} required.{RESET}"
        ),
        None => { },
    }
}


fn replace_list<P: AsRef<Path>>(
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

fn print_restore(dump: String, paths: &[&String], verbose: bool, force: bool) {
    let check = !paths.is_empty();
    let mut paths_set = HashSet::new();
    if check {
        for path in paths {
            paths_set.insert(path.as_str());
        }
    }
    let lines = dump.split('\n');
    let mut mode = "f";
    let mut file = "";
    let mut nums = Vec::<usize>::new();
    let mut nums_i = 0;
    let mut key = String::new();
    let mut val = String::new();
    let mut kvs = Vec::new();
    for line in lines {
        if mode == "f" {
            file = line;
            mode = "n";
        } else if mode == "n" {
            nums.clear();
            kvs.clear();
            let nums_raw = line.split(' ');
            for num in nums_raw {
                let res = num.parse();
                if let Ok(res) = res {
                    nums.push(res);
                }
            }
            nums.pop();
            mode = "k";
            nums_i = 0;
        } else if mode == "k" {
            if !key.is_empty() {
                key.push('\n');
            }
            key.push_str(line);
            nums[nums_i] -= 1;
            if nums[nums_i] == 0 {
                mode = "v";
                nums_i += 1;
            }
        } else if mode == "v" {
            if !val.is_empty() {
                val.push('\n');
            }
            val.push_str(line);
            nums[nums_i] -= 1;
            if nums[nums_i] == 0 {
                mode = "k";
                nums_i += 1;
                kvs.push((std::mem::take(&mut key), std::mem::take(&mut val)));
                if nums_i >= nums.len() {
                    mode = "f";
                    if !check || paths_set.contains(file) {
                        let mut printed = false;
                        if verbose {
                            println!("{BOLD}{GREEN}{file}{RESET}{GREEN}:{RESET}");
                            printed = true;
                        }
                        for (k, v) in &kvs {
                            let res = set(file, k, v, !force);
                            if res.is_err() && !printed {
                                println!("{BOLD}{GREEN}{file}{RESET}{GREEN}:{RESET}");
                                printed = true;
                            }
                            match res {
                                Ok(None) if verbose => println!(
    "  {GREEN}Attribute {DEFAULT}{k}{GREEN} {YELLOW}set{GREEN} successfully.{RESET}"
                                ),
                                Ok(Some(old)) if verbose => println!(
    "  {GREEN}Attribute {DEFAULT}{k}{GREEN} {YELLOW}overwritten{GREEN} successfully.
  Old value was \"{RESET}{old}{GREEN}\".{RESET}"
                                ),
                                Ok(_) => { },
                                Err(true) => println!(
    "  {BOLD}{RED}Could not {YELLOW}set{RED} {DEFAULT}{k}{RED} without {YELLOW}force{RED}!{RESET}"
                                ),
                                Err(false) => println!(
    "  {BOLD}{RED}Could not {YELLOW}set{RED} attribute {DEFAULT}{k}{RED}.{RESET}"
                                ),
                            }
                        }
                    }
                }
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

