mod core;
mod actions;

use actions::*;

use std::{
    env,
    process::ExitCode,
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
        else if (arg == "rank" || arg == "ra") && mode == " " {
            mode = "ra";
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
        ("g" | "r" | "ra", [att, paths @ ..], []) => {
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
        ("g" | "r" | "ra", [], [_]) => println!(
"{BOLD}{RED}No {YELLOW}attribute{RED} provided!{RESET}"
        ),
        ("g" | "r" | "ra", [_], []) => no_path(),
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
        ("ra", [attr], paths) => {
            print_rank(attr, paths, "", "");
        },
        ("ra", [attr, flag], paths) => {
            print_rank(attr, paths, flag, "");
        },
        ("ra", [attr, flag_a, flag_b], paths) => {
            print_rank(attr, paths, flag_a, flag_b);
        },
        _ => { },
    }

    ExitCode::SUCCESS
}

