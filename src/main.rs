use std::fs::*;
use std::io::stdin;
use std::process::{exit, Command};
use libc;
use std::env;
fn main() {
    unsafe {
        let mut rootbypassed = false;
        let mut osbypassed = false;
        let args: Vec<_> = env::args().collect();
            for arg in args {
                if arg == "--help" {
                    println!("Usage: 
                    --help: prints this message
                    --uhh: bypasses root requirements
                    --binbows: bypasses linux requirements
                    This program reads from the /etc/pactheman/config.ptm file to install software and manage systemd services.
                    Syntax:
                    *           Start a systemd service
                    +           Enable a systemd service
                    +*         Enable and start a systemd service
                    &           Stop a systemd service
                    -*           Disable and stop a systemd service
                    -              Disable a systemd service
                    #             Install one or multiple packages
                    ");
                    exit(1);
                }
                if arg == "--uhh" {
                    rootbypassed = true;
                }
                if arg == "--binbows" {
                    osbypassed = true;
                }
            }
            if libc::getgid() != 0 && rootbypassed != true{
                println!("Most of these operations require root permissions, like sudo, so it would be wise to run me as root. Pass --uhh to override.");
                exit(1);
            }
            if osbypassed != true && env::consts::OS != "linux"{
                eprintln!("I would recommend using --binbows to run this on other operating systems, even if you aren't using windows!");
            }
    }
    // flags
    let mut enableservice = false;
    let mut install = false;
    let mut enableandstartservice = false;
    let mut startservice = false;
    let mut disableservice = false;
    let mut disableandstopservice = false;
    let mut stopservice = false;
    match DirBuilder::create(&DirBuilder::new()
    .recursive(true) , "/etc/pactheman/") {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Error creating config directory, are you root? Details: {}", err);
            exit(1)
        }
    };
    let file_content = match read_to_string("/etc/pactheman/config.ptm") {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Failed to read the config file at `/etc/pactheman/config.ptm`, do you want me to create it?");
            let mut answer = String::new();
            stdin()
                .read_line(&mut answer)
                .expect("What the [SWEAR WORD] is happening, where is your stdin?");
            if answer
                .to_lowercase()
                .trim()
                == "yes" {
                    println!("Okay, will do!");
                    match File::create_new("/etc/pactheman/config.ptm") {
                        Ok(_) => (),
                        Err(e) => eprintln!("Uhh, no!  {}", e)
                    }
                } else {
                    println!("Uhh, why not?");
                }
            String::new() // Return an empty string on error
        },
    };
    let lines = file_content.split("\n");
    for line in lines.into_iter() {
        let words = line.split_ascii_whitespace();
        for word in words {
            if word == "#" {
                install = true;
                let mut answer = String::new();
                println!("Do you really want to install these packages? [Y/n]");
                stdin()
                    .read_line(&mut answer)
                    .expect("What happened to your standard input?: ");
                if answer.to_lowercase().trim() == "n" || answer.to_lowercase().trim() != "y"{
                    exit(1);
                }
                println!("Building package(s)!");
                continue;
            }
            if word == "+" {
                enableservice = true;
                println!("Enabling service(s)!");
                continue;
            }
            if word == "+*" {
                enableandstartservice = true;
                println!("Enabling and starting service(s)!");
                continue;
            }
            if word == "-" {
                disableservice = true;
                println!("Disabling service(s)!");
                continue;
            }
            if word == "-*" {
                disableandstopservice = true;
                println!("Disabling and stopping service(s)!");
                continue;
            }
            if word == "*" {
               startservice = true;
               println!("Starting a(some) service(s)!");
                continue;
            }
            if word == "&" {
                stopservice = true;
                println!("Stopping service(s)!");
                continue;
            }
            if stopservice == true {
                Command::new("systemctl")
                .args(["stop", word])
                .status()
                .expect("Error stopping systemd service: ");
            }
            if disableandstopservice == true {
                Command::new("systemctl")
                .args(["disable", "--now", word])
                .status()
                .expect("Error disabling and stopping systemd service: ");
            }
            if disableservice == true {
                Command::new("systemctl")
                .args(["disable",  word])
                .status()
                .expect("Error disabling systemd service: ");
            }
            if enableservice == true {
                Command::new("systemctl")
                .args(["enable", word])
                .status()
                .expect("Error enabling systemd service: ");
            }
            if enableandstartservice == true {
                Command::new("systemctl")
                .args(["enable", word, "--now"])
                .status()
                .expect("Error starting and enabling systemd service: ");
            }
            if startservice == true {
                Command::new("systemctl")
                .args(["start", word])
                .status()
                .expect("Error starting  systemd service: ");
            }

            if install == true {
                Command::new(String::from("pacman" ))
                      .args(["--noconfirm", "--needed", "-S", word])
                    .status()
                    .expect("Error installing package: ");
            }
            install = false;
            enableservice = false;
            enableandstartservice = false;
            startservice = false;
            disableandstopservice = false;
            disableservice = false;
        }
        }
    return; 
}
