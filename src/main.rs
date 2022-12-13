use std::io::{stdin, BufRead};

use argparse::{ArgumentParser, Store, StoreOption, StoreTrue};
use colored::{ColoredString, Colorize};
use regex::{Captures, Match, Regex};

fn colorize(fg: Option<String>, bg: Option<String>) -> impl Fn(&str) -> ColoredString {
    let rgb_re = Regex::new("#?([\\da-fA-F]+)").unwrap();
    let fg = if let Some(fg) = fg {
        let captures = rgb_re.captures(&fg).unwrap();
        let m = captures.get(1).unwrap();
        let rgb = hex::decode(m.as_str()).unwrap();
        Some(rgb)
    } else {
        None
    };
    let bg = if let Some(bg) = bg {
        let captures = rgb_re.captures(&bg).unwrap();
        let m = captures.get(1).unwrap();
        let rgb = hex::decode(m.as_str()).unwrap();
        Some(rgb)
    } else {
        None
    };
    move |s: &str| {
        let with_fg = if let Some(fg) = fg.clone() {
            s.truecolor(fg[0], fg[1], fg[2])
        } else {
            s.normal()
        };
        if let Some(bg) = bg.clone() {
            with_fg.on_truecolor(bg[0], bg[1], bg[2])
        } else {
            with_fg
        }
    }
}

fn main() {
    colored::control::set_override(true);
    let mut pattern = false;
    let mut capture = false;
    let mut rep_string = String::new();
    let mut sub_string = String::new();
    let mut fg_color_hex: Option<String> = None;
    let mut bg_color_hex: Option<String> = None;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Replace characters, regular expressions or colors read from stdin.");
        ap.refer(&mut pattern).add_option(
            &["-r", "--regex"],
            StoreTrue,
            "Interpret argument as regular expression.",
        );
        ap.refer(&mut capture).add_option(
            &["-c", "--capture"],
            StoreTrue,
            "Inject captured strings into substitute using &n where n is a group number.",
        );
        ap.refer(&mut fg_color_hex).add_option(
            &["-f", "--fg"],
            StoreOption,
            "Set a new foreground color.",
        );
        ap.refer(&mut bg_color_hex).add_option(
            &["-b", "--bg"],
            StoreOption,
            "Set a new background color.",
        );
        ap.refer(&mut rep_string)
            .add_argument(
                "replacement string",
                Store,
                "The string to use for replacing.",
            )
            .required();
        ap.refer(&mut sub_string)
            .add_argument(
                "substitution string",
                Store,
                "The string to substitute with.",
            )
            .required();
        ap.parse_args_or_exit();
    }
    let colorize = colorize(fg_color_hex, bg_color_hex);
    if capture && !sub_string.contains("&") {
        eprintln!("{}", "Warning: received flag for capturing, but no capture groups (&#) are present in substitution string.");
    }
    if capture && !(rep_string.contains("(") || rep_string.contains(")")) {
        eprintln!("{}", "Warning: received flag for capturing, but no captures are present in replacement string.");
    }
    if pattern && capture {
        let replace_re = Regex::new(&rep_string).unwrap();
        for line in stdin().lock().lines() {
            let line = line.unwrap();
            let cloned_line = line.clone();
            let parts: Vec<&str> = replace_re.split(&cloned_line).collect();
            if parts.len() == 1 {
                println!("{}", line.trim_end());
                continue;
            }
            let mut captures: Vec<Captures> = replace_re.captures_iter(&line).into_iter().collect();
            captures.reverse();
            for i in 0..parts.len() - 1 {
                let part_captures: Option<Vec<Match>> = captures
                    .pop()
                    .map(|cs| cs.iter().map(|it| it.unwrap()).collect());
                let mut raw_sub = sub_string.clone();
                if let Some(captures) = part_captures {
                    for i in 0..captures.len() {
                        let index = format!("&{}", i);
                        raw_sub = raw_sub.replace(index.as_str(), captures[i].as_str());
                    }
                }
                let sub_colorized = colorize(&raw_sub);
                print!("{}", format!("{}{}", parts[i], sub_colorized));
            }
            let last_capture: Option<Vec<Match>> = captures
                .pop()
                .map(|cs| cs.iter().map(|it| it.unwrap()).collect());
            let mut raw_sub = sub_string.clone();
            if let Some(captures) = last_capture {
                for i in 0..captures.len() {
                    let index = format!("&{}", i);
                    raw_sub = raw_sub.replace(index.as_str(), captures[i].as_str());
                }
                let sub_colorized = colorize(&raw_sub);
                print!("{}", sub_colorized);
            }
            println!("{}", parts[parts.len() - 1].trim_end())
        }
    } else if pattern {
        let replace_re = Regex::new(&rep_string).unwrap();
        for line in stdin().lock().lines() {
            let line = line.unwrap();
            let sub_colorized = colorize(&sub_string);
            let parts: Vec<&str> = replace_re.split(&line).collect();
            if parts.len() == 1 {
                println!("{}", line.trim_end());
                continue;
            }
            for i in 0..parts.len() - 1 {
                print!("{}", format!("{}{}", parts[i], sub_colorized));
            }
            println!("{}", parts[parts.len() - 1].trim_end());
        }
    } else {
        for line in stdin().lock().lines() {
            let line = line.unwrap();
            let sub_colorized = colorize(&sub_string);
            let parts: Vec<&str> = line.split(&rep_string).collect();
            if parts.len() == 1 {
                println!("{}", line.trim_end());
                continue;
            }
            for i in 0..parts.len() - 1 {
                print!("{}", format!("{}{}", parts[i], sub_colorized));
            }
            println!("{}", parts[parts.len() - 1].trim_end());
        }
    }
}
