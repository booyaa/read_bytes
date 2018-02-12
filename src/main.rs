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
    // let path = "/Users/booyaa/coding/own/rust/learn-intermezzos/nickos/boot_sect.bin";
    let path = "/Users/booyaa/coding/own/rust/learn-intermezzos/build/os.iso";

    let mut f = File::open(path)?;
    let mut buffer = [0; 16]; // 16 bytes

    let mut last_byte = String::new();

    for i in 0..32 {
        print!("{:02} ", i); // Wondering about {:02}? To see all the possible permutations go to std::fmt
        f.read(&mut buffer[..])?;
        for byte in &buffer {
            let byte_hex = format!("{:02x}", byte); // turns 2 to 02
            
            print!("{} ", &byte_hex);

            if byte_hex == "aa" && last_byte == "55" {
                    print!("found magic!");    
            }

            last_byte = byte_hex.to_owned();
        }
        println!("");
    }

    Ok(())
}

quick_main!(run);
