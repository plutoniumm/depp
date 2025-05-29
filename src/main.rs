#!/usr/bin/env run-cargo-script

use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::process::{Command, Output};

mod manager;

fn prog() -> Option<String> {
    env::args()
        .next()
        .as_ref()
        .map(Path::new)
        .and_then(Path::file_name)
        .and_then(OsStr::to_str)
        .map(String::from)
}

fn shell(cmd: &str, args: &[&str]) -> Output {
    Command::new(cmd)
        .args(args)
        .output()
        .expect("failed to execute process")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!(
            "Usage: {} <command> <key:value> | <command:key> <value>...",
            prog().unwrap_or_else(|| "dep".to_string())
        );
        return;
    }

    let mut suffix;
    let mut cmd = args[1].clone();
    let mut transformed;
    if cmd.contains(':') {
        // `add:x a b` => `add x:a x:b`
        let split_index = cmd.find(':').unwrap();
        suffix = cmd[split_index + 1..].to_string();
        cmd = cmd[..split_index].to_string();

        // "args[i]" -> "suffix:args[i]"
        transformed = args[2..]
            .iter()
            .map(|arg| format!("{}:{}", suffix, arg))
            .collect::<Vec<String>>();
        transformed.insert(0, cmd.to_string());
    } else {
        transformed = (&args[1..]).to_vec();
    }
    println!("Command: {}", cmd);
    println!("Args: {:?}", transformed);

    for arg in &transformed[1..] {
        let parts: Vec<&str> = arg.splitn(2, ':').collect();
        if parts.len() == 2 {
            println!("{} and {}", parts[0], parts[1]);
        } else {
            eprintln!("Invalid param: {}", arg);
        }
    }

    // Uncomment to run the actual command
    // let out = shell(cmd, &transformed[1..].iter().map(AsRef::as_ref).collect::<Vec<&str>>());
    // println!("stdout: {}", String::from_utf8_lossy(&out.stdout));
    // eprintln!("stderr: {}", String::from_utf8_lossy(&out.stderr));
}
