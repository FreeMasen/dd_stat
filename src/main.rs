use std::process::{Command, Stdio};
use std::path::{PathBuf};
use std::fs::{metadata};
use std::io::prelude::*;
use std::io::{BufReader};
use std::string::String;
extern crate clap;
use clap::{Arg, App, ArgMatches};
extern crate nix;

fn main() {
    let matches = get_matches();
    let in_file = matches.value_of("infile").unwrap_or_else(|| panic!("input is required"));
    let out_file = matches.value_of("outfile").unwrap_or_else(|| panic!("output is required"));
    let bs = matches.value_of("blocksize").unwrap_or("1m");
    let target_size = get_in_size(in_file.to_string().clone());
    println!("infile is {} bytes", target_size);
    let mut dd = Command::new("dd")
                        .args(&[
                            format!("if={}", in_file),
                            format!("of={}", out_file),
                            format!("bs={}", bs),
                            ])
                        .stderr(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .stdin(Stdio::piped())
                        .spawn()
                        .expect("Error from dd");
    println!("pre-loop");
    let dd_pid = nix::unistd::Pid::from_raw(dd.id().clone() as i32);
    loop {
        println!("loop");
        match dd.try_wait() {
            Ok(status) => {
                match status {
                    Some(status) => {
                        println!("dd is done {}", status);
                        break;
                    },
                    None => {
                        println!("No status");
                        // let stdo = dd.stdout.as_mut().expect("Couldn't take stdout as mut");
                        println!("took stdout as mut");
                        // let mut stdout = BufReader::new(stdo);
                        println!("converted stdout to BufReader");
                        nix::sys::signal::kill(dd_pid, nix::sys::signal::SIGSTOP).unwrap();
                        println!("sent SIGSTOP");
                        // let mut line = String::new();
                        // stdout.read_line(&mut line).expect("Unable to read to line");
                        // println!("stdout: {}", line);
                        // let buf = stdout.fill_buf();
                        // println!("Filled buffer");
                        // match buf {
                        //     Ok(buffer) => {
                        //         let text = String::from_utf8(buffer.to_vec()).expect("Unable to convert buffer to text");
                        //         println!("stdout: {}", text);
                        //     },
                        //     Err(e) => println!("error: {}", e)
                        // }
                    }
                }
            },
            Err(e) => println!("Error in dd:\n{}", e)
        }
    }
    dd.wait().expect("dd error");
}

fn get_matches() ->  ArgMatches<'static> {
    App::new("dd_stat")
            .version("0.1.0")
            .arg(Arg::with_name("infile")
                .short("i")
                .long("infile")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("outfile")
                .short("o")
                .long("outfile")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("blocksize")
                .short("b")
                .long("blocksize")
                .default_value("4m")
                .required(false))
            .get_matches()
}

fn get_in_size(path: String) -> i32 {
    let path = PathBuf::from(path);
    let md = metadata(path).expect("Unable to get metadata for infile");
    md.len() as i32
}

// fn calculate_progress(target: i32, bytes_transfered: String) -> i32 {
//     let progress = bytes_transfered.parse::<i32>().unwrap();
//     progress / target
// }