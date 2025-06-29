use std::{
    fs::{self, read_dir},
    io::Error,
    path::Path,
    process,
};

use clap::{Command, arg};

fn main() {
    let matches = Command::new("rsync-lite")
        .version("0.1")
        .about("copy entire directories recursively")
        .arg(arg!(--src <VALUE>).required(true))
        .arg(arg!(--dest <VALUE>).required(true))
        .get_matches();

    let src = Path::new(
        matches
            .get_one::<String>("src")
            .expect("source path is required"),
    );
    let dest = Path::new(
        matches
            .get_one::<String>("dest")
            .expect("destination path is required"),
    );

    let result = if src.is_dir() {
        if let Err(e) = fs::create_dir_all(dest) {
            eprintln!("Error creating destination directory: {}", e);
            process::exit(1);
        }
        copy_directory_structure(src, dest)
    } else {
        transfer_file(src, dest)
    };

    match result {
        Ok(()) => println!("copy completed!"),
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    }
}

fn transfer_file(src: &Path, dest: &Path) -> Result<(), Error> {
    fs::copy(src, dest)?;
    println!("Copied file: {} -> {}", src.display(), dest.display());
    Ok(())
}

fn copy_directory_structure(src: &Path, dest: &Path) -> Result<(), Error> {
    for entry in read_dir(src)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let dir_name = path.file_name().ok_or_else(|| {
                Error::new(std::io::ErrorKind::InvalidInput, "Invalid directory name")
            })?;
            let dest_dir = dest.join(dir_name);

            fs::create_dir_all(&dest_dir)?;
            copy_directory_structure(&path, &dest_dir)?;
        } else {
            let file_name = path
                .file_name()
                .ok_or_else(|| Error::new(std::io::ErrorKind::InvalidInput, "Invalid file name"))?;
            let dest_file = dest.join(file_name);
            transfer_file(&path, &dest_file)?;
        }
    }
    Ok(())
}
