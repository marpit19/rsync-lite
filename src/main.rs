use std::{env, fs, process};

fn main() {
    // GOAL: copy a single file from source to destination
    println!("copy a single file from source to destination");

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <src> <dest>", args[0]);
        process::exit(1);
    }

    match transfer_file(&args[1], &args[2]) {
        Ok(msg) => println!("{}", msg),
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    }
}

fn transfer_file(src: &str, dest: &str) -> Result<String, String> {
    match fs::copy(src, dest) {
        Ok(bytes_copied) => Ok(format!(
            "total bytes copied from {} to {}: {}",
            src, dest, bytes_copied
        )),
        Err(e) => Err(format!("Transfer failed: {}", e)),
    }
}
