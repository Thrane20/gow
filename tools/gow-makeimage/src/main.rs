// use clap::{command, Arg, ArgAction, ArgMatches};
// use dockworker::{container::ContainerFilters, Docker};
// use dotenv::dotenv;
// use serde::{Deserialize, Serialize};
// use std::{collections::HashMap, env, panic};
use std::{process::{Command, Stdio}, fs::File, env, io::{Write, Read}};

fn main() {
    run_winecfg();
}

fn run_winecfg() {

    let mut file = File::create("./log.txt").unwrap();

    println!("Starting winecfg - {:?}", env::current_dir().unwrap());
    let output = Command::new("/mnt/linuxfast/app_images/winege/bin/wine")
    .args(["winecfg"])
    .stdout(Stdio::piped())
    .output();

    // TODO: find out how to write the output to a file!! 

    println!("Winecfg stopped");
}

fn run_process() {
    let output = Command::new("/bin/cat")
    .arg("/home/ians/.bashrc")
    .output()
    .expect("/bin/cat should have run ok, but it didn't");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
