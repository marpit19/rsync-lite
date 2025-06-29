use std::{
    fs::{self, read_dir},
    io::Error,
    path::Path,
    process,
};

use clap::{Command, arg};
use filetime::FileTime;

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

    let metadata = fs::metadata(src)?;
    fs::set_permissions(dest, metadata.permissions())?;

    let modification_time = FileTime::from_last_modification_time(&metadata);
    filetime::set_file_mtime(dest, modification_time)?;

    println!("Copied file: {} -> {}", src.display(), dest.display());
    Ok(())
}

fn copy_directory_structure(src: &Path, dest: &Path) -> Result<(), Error> {
    for entry in read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;

        if file_type.is_symlink() {
            let file_name = path
                .file_name()
                .ok_or_else(|| Error::new(std::io::ErrorKind::InvalidInput, "Invalid file name"))?;

            let link_target = fs::read_link(&path)?;
            let dest_link = dest.join(file_name);

            #[cfg(unix)]
            std::os::unix::fs::symlink(&link_target, &dest_link)?;

            #[cfg(windows)]
            {
                if link_target.is_dir() {
                    std::os::windows::fs::symlink_dir(&link_target, &dest_link)?;
                } else {
                    std::os::windows::fs::symlink_file(&link_target, &dest_link)?;
                }
            }
            println!(
                "Copied symlink: {} -> {}",
                path.display(),
                dest_link.display()
            );
        } else if path.is_dir() {
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
