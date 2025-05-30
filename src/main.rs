#!/usr/bin/env run-cargo-script

use std::env;
use std::io::Error;
use std::process::{Command, Output};

#[path = "manager.rs"]
mod mgr;

fn shell(cmd: &str, args: &[&str]) -> Result<Output, Error> {
    let out = Command::new(cmd)
        .args(args)
        .spawn()
        .expect("Failed to start command")
        .wait_with_output();

    let output = out.expect("Failed to wait for command");
    if output.status.success() {
        Ok(output)
    } else {
        Err(Error::new(
            std::io::ErrorKind::Other,
            format!("Command '{}' failed with status: {}", cmd, output.status),
        ))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: {} <add|remove> <lang> <pkg>...", args[0]);
        return;
    }

    let cmd = &args[1];
    let lang_key = &args[2];
    let pkgs = &args[3..];

    let lang = match mgr::Managers::detect(lang_key) {
        Some(l) => l,
        None => {
            eprintln!("Unknown language: {}", lang_key);
            return;
        }
    };

    let exec = match cmd.as_str() {
        "add" => &lang.add,
        "remove" => &lang.remove,
        _ => {
            eprintln!("Unknown command: {}", cmd);
            return;
        }
    };

    for pkg in pkgs {
        let mut parts = exec.split_whitespace();
        let cmd = parts.next().expect("Empty command");
        let mut exec_args: Vec<&str> = parts.collect();
        exec_args.push(pkg);
        let out = shell(cmd, &exec_args).expect("Failed to execute command");

        if out.status.success() {
            println!("Success: {} {}", exec, pkg);
        } else {
            let utf = String::from_utf8_lossy;

            eprintln!("Fail: {} {}", exec, pkg);
            eprintln!("stdout: {}", utf(&out.stdout));
            eprintln!("stderr: {}", utf(&out.stderr));
        }
    }
}
