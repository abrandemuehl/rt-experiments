use clap::Parser;
use libc::geteuid;
use rlimit::{setrlimit, Resource};
use std::env::args;
use std::ffi::OsString;
// use std::os::unix::process::CommandExt;
use std::path::Path;
// use std::process::Command;

// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, trailing_var_arg=true)]
struct CliArgs {
    /// Command to run
    command: String,
}

fn main() {
    let cli_args = CliArgs::parse();
    println!("Command {}!", cli_args.command);

    let euid: u32;
    unsafe {
        euid = geteuid();
    }
    if euid != 0 {
        match args().next() {
            Some(arg) => println!(
                "{} must be run as root",
                Path::new(OsString::from(arg).as_os_str())
                    .file_name()
                    .unwrap_or(OsString::from("rtwrapper").as_os_str())
                    .to_str()
                    .unwrap()
            ),
            None => println!("Must be run as root"),
        }
        std::process::exit(1);
    }

    const DEFAULT_SOFT_LIMIT: u64 = 4 * 1024 * 1024;
    const DEFAULT_HARD_LIMIT: u64 = 8 * 1024 * 1024;

    match setrlimit(Resource::MEMLOCK, DEFAULT_SOFT_LIMIT, DEFAULT_HARD_LIMIT) {
        Err(err) => println!("{}", err.to_string()),
        Ok(_) => (),
    }

    match setrlimit(Resource::RTPRIO, DEFAULT_SOFT_LIMIT, DEFAULT_HARD_LIMIT) {
        Err(err) => println!("{}", err.to_string()),
        Ok(_) => (),
    }

    // Fork the process to run the application as unprivileged user.
    // unsafe {
    //     // Switch user to unprivileged user.
    //     let child: pid_t = fork();
    //     if child == 0 {
    //         // Child process. Exec the requested command.
    //         C
    //     }
    // }
}
