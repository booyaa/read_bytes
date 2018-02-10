#[macro_use]
extern crate error_chain;

// use std::io;
use std::io::prelude::*;
use std::fs::File;

error_chain! {
    foreign_links {
        Io(std::io::Error);
    }
}

fn run() -> Result<()> {
    let path = "/Users/booyaa/coding/own/rust/learn-intermezzos/nickos/boot_sect.bin";

    let mut f = File::open(path)?;
    let mut buffer = [0; 16]; // 16 bytes

    let mut last_byte = String::new();

    for i in 0..32 {
        print!("{:02} ", i);
        f.read(&mut buffer[..])?;
        for byte in &buffer {
            let byte_hex_raw = format!("{:x}", byte);
            let byte_hex = if &byte_hex_raw == "0" {
                "00"
            } else {
                &byte_hex_raw
            }; // pad
            print!("{} ", &byte_hex);

            // if last_byte == "55" &&
            if byte_hex == "aa" {
                // print!("{} ", &last_byte);
                if last_byte == "55" {
                    // print!("{} ", &byte_hex);
                    print!("found magic!");
                }
            } else {
                // print!(".. ");
            }

            last_byte = byte_hex.to_owned();
        }
        println!("");
    }

    Ok(())
}

quick_main!(run);