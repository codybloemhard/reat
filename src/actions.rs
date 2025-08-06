use crate::core::*;

use std::{
    path::Path,
    fmt::Display,
    collections::{ HashSet, HashMap },
};

use zen_colour::*;

pub fn print_list<P: AsRef<Path> + Display>(path: P, print_filename: bool, verbose: bool) {
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


pub fn print_dump<P: AsRef<Path> + Display>(path: P) {
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

pub fn print_copy<P: AsRef<Path> + Display>(srcp: P, dstp: P) {
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
    "{BOLD}{RED}Could not {YELLOW}set{RED} attribute {DEFAULT}{key:?}{RED} on destination.{RESET}",
                );
            }
        }
    }
    if ok {
        println!("{BOLD}{GREEN}Successfully copied from source to destination.{RESET}");
    }
}

pub fn print_get<P: AsRef<Path> + Display>(path: P, key: &str, print_filename: bool, verbose: bool) {
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

pub fn print_set<P: AsRef<Path> + Display>(
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

pub fn print_add_list<P: AsRef<Path> + Display>(path: P, key: &str, value: &str, print_filename: bool) {
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

pub fn print_remove<P: AsRef<Path> + Display>(path: P, key: &str, print_filename: bool, force: bool) {
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

pub fn print_clear<P: AsRef<Path> + Display>(
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


pub fn print_cut_list<P: AsRef<Path> + Display>(
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

pub fn print_contains(mode: char, key: &str, values: &[&String], path: &str) {
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

pub fn print_rename<P: AsRef<Path> + Display>(
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

pub fn print_replace<P: AsRef<Path> + Display>(
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

pub fn print_restore(dump: String, paths: &[&String], verbose: bool, force: bool) {
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

pub fn print_rank(key: &str, paths: &[&String], flag_a: &str, flag_b: &str) {
    let mut counts = HashMap::new();
    let mut total = 0;
    let mut present = 0;
    let (mut flip, mut reverse) = (false, false);
    if flag_a == "flip" || flag_b == "flip" {
        flip = true;
    }
    if flag_a == "reverse" || flag_b == "reverse" {
        reverse = true;
    }
    for path in paths {
        total += 1;
        if let Some((_, avalue)) = get(path, key) {
            present += 1;
            let list = avalue.split(',').map(|s| s.to_string()).collect::<Vec<_>>();
            for item in list {
                let count = counts.get(&item).unwrap_or(&0);
                counts.insert(item.clone(), count + 1);
            }
        }
    }
    if reverse {
        println!("{GREEN}{BOLD}total{RESET}{BOLD}:{RESET} {present} / {total}");
    }

    fn do_reverse<T: std::cmp::Ord>(mut v: Vec<T>, reverse: bool) -> Vec<T> {
        v.sort();
        let res: Vec<_> = if reverse {
            v.into_iter().rev().collect()
        } else {
            v.into_iter().collect()
        };
        res
    }
    if !flip {
        let res = counts.iter().map(|(k, f)| (f, k)).collect::<Vec<_>>();
        let res = do_reverse(res, reverse);
        for (freq, item) in res {
            println!("{BOLD}{item}{RESET}: {freq}");
        }
    } else {
        let res = do_reverse(counts.iter().collect::<Vec<_>>(), reverse);
        for (item, freq) in res {
            println!("{BOLD}{item}{RESET}: {freq}");
        }
    };

    if !reverse {
        println!("{GREEN}{BOLD}total{RESET}{BOLD}:{RESET} {present} / {total}");
    }
}
