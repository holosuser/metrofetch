use std::fs;
use std::env;
//use toml::*;
use sheller::run;
//use colored::*;
use std::io::{self, Write};
use crate::logos::{archl, gentool, nixosl, voidl};

mod logos;

fn logo() {
    let content=fs::read_to_string("/etc/os-release").expect("Error read /etc/os-release");
    let mut osn=String::new();
    for line in content.lines(){
        if line.starts_with("ID="){
            osn=line.replace("ID=", "").replace("\"", "");
            break;
        }
    }
    match osn.as_str(){
        "arch" => archl(),
        "gentoo" => gentool(),
        "nixos" => nixosl(),
        "void" => voidl(),
        _ => println!("unknown distro"),
    }
}
fn os(){
    let content=fs::read_to_string("/etc/os-release")
        .expect("Error read /etc/os-release");
    let pretty_name=content
        .lines()
        .find(|line| line.starts_with("PRETTY_NAME="))
        .and_then(|line| line.split_once('='))
        .map(|(_, value)| value.trim_matches('"'))
        .unwrap();
    println!("os: {}", pretty_name);
}
fn kernel(){
    print!("kernel: ");
    io::stdout().flush();
    run!("uname -rs");
}
fn shell(){
    println!("shell: {}", env::var("SHELL").expect("Error!"));
}
fn term(){
    println!("term: {}", env::var("TERM").expect("Error!"));
}
fn cpu(){
    let content = fs::read_to_string("/proc/cpuinfo").expect("Error read /proc/cpuinfo");
    if let Some(line) = content.lines().find(|l| l.contains("model name")) {
        let cpu_model = &line[13..];
        println!("cpu: {}", cpu_model.trim());
    } else {
        println!("Error model name not found");
    }
}
fn gpu(){
    print!("gpu: ");
    io::stdout().flush();
    sheller::run!("lspci | grep -i vga | cut -b 36- | sed 's/ *(rev.*)//'");
}
fn fs(){
    print!("fs: ");
    io::stdout().flush();
    run!("findmnt -n -o FSTYPE /");
}

fn main(){
    logo();
    os();
    kernel();
    shell();
    term();
    cpu();
    gpu();
    fs();
}
