use std::path::{PathBuf};
use std::fs::{metadata, File};
use std::io::prelude::*;
use std::io::{stderr, stdout};
use std::io::{BufReader, BufWriter};
use std::string::String;
use std::str::FromStr;
use std::usize;
use std::ops::Add;
mod bar;
use bar::Bar;

extern crate clap;
use clap::{Arg, App, ArgMatches};


fn main() {
    let matches = get_matches();
    let in_path_str = matches.value_of("infile").expect("infile is required");
    let out_path_str = matches.value_of("outfile").expect("outfile is required");
    let bs_str = matches.value_of("blocksize").unwrap_or("1m");
    let target_size = get_in_size(in_path_str.to_string().clone());
    let block_size = compute_block_size(bs_str);
    let full_blocks = target_size / block_size;
    let partial_block = target_size % block_size;
    println!("infile: {}", in_path_str);
    println!("outfile: {}", out_path_str);
    println!("bs: {} ({})", bs_str, block_size);
    println!("infile size: {}", target_size);
    println!("full blocks: {}", full_blocks);
    println!("remaining bytes: {}", partial_block);
    let in_path = PathBuf::from(in_path_str);
    let in_file = File::open(in_path).expect("Unable to open infile");
    let mut in_buf = BufReader::new(in_file);
    let out_path = PathBuf::from(out_path_str);
    let out_file = File::create(out_path).expect("Unable to create outfile");
    let mut out_buf = BufWriter::new(out_file);
    let mut total_written = 0;
    let mut bar = Bar::new(target_size);
    for i in 0..full_blocks {
        let mut buffer_box = Vec::with_capacity(block_size).into_boxed_slice();
        in_buf.read_exact(&mut buffer_box).expect("error in read");
        out_buf.write(&mut buffer_box).expect("error in write");
        total_written += block_size;
        bar.update(block_size);
        // println!("written {:?}\ntarget: {:?}\n{:?}",  total_written, target_size, (total_written as f32 / target_size as f32) * 100.0);
    }
    if partial_block > 0 {
        let mut buffer_box = Vec::with_capacity(partial_block).into_boxed_slice();
        in_buf.read_exact(&mut buffer_box).expect("error in write of partial block");
        out_buf.write(&mut buffer_box).expect("error in write of partial block");
        total_written += partial_block;
        bar.update(partial_block);
        // println!("written {:?}\ntarget: {:?}\n{:?}",  total_written, target_size, (total_written as f32 / target_size as f32) * 100.0);
    }
    
}

fn compute_block_size(block_Size: &str) -> usize {
    let last_i = block_Size.len() -1;
    let last_char = block_Size.get(last_i..last_i+1);
    let num_str = block_Size.get(..last_i).expect("index error");
    let num = usize::from_str(num_str).expect("blocksize not formatted correctly");
    
    match last_char {
        Some(ch) => 
            match ch.to_lowercase().as_str() {
                "k" => num * 1024,
                "m" => num * 1024 * 1024,
                "g" => num * 1024 * 1024 * 1024,
                _ => num
            }
        None => num
    }
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

fn get_in_size(path: String) -> usize {
    let path = PathBuf::from(path);
    let md = metadata(path).expect("Unable to get metadata for infile");
    md.len() as usize
}
