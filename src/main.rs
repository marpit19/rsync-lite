use std::{fs, process};

use clap::{Command, arg};

fn main() {
    let matches = Command::new("rsync-lite")
        .version("0.1")
        .about("copies a single file from source to destination")
        .arg(arg!(--src <VALUE>).required(true))
        .arg(arg!(--dest <VALUE>).required(true))
        .get_matches();

    let src = matches.get_one::<String>("src").expect("src is required");
    let dest = matches.get_one::<String>("dest").expect("dest is required");

    match transfer_file(src, dest) {
        Ok(msg) => println!("{}", msg),
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    }
}

fn transfer_file(src: &str, dest: &str) -> Result<String, String> {
    match fs::copy(src,dest) {
        Ok(bytes_copied) => Ok(format!(
            "total bytes copied from {} to {}: {}",
            src, dest, bytes_copied
        )),
        Err(e) => Err(format!("Transfer failed: {}", e)),
    }
}
