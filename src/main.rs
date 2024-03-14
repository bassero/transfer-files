use std::fs;
use std::io;
use std::path::{Path, PathBuf};
#[allow(dead_code)]
fn main() {
    // Prompt the user to enter the source directory
    // println!("Enter the source directory:");
    // let mut source = String::new();
    // io::stdin()
    //     .read_line(&mut source)
    //     .expect("Failed to read input");
    // let source = source.trim(); // Remove trailing newline and whitespace

    // // Prompt the user to enter the destination directory
    // println!("Enter the destination directory:");
    // let mut destination = String::new();
    // io::stdin()
    //     .read_line(&mut destination)
    //     .expect("Failed to read input");
    // let destination = destination.trim(); // Remove trailing newline and whitespace
    let source = "D:\\1.txt";
    let destication = "D:\\download";
    // Call the function to copy files from source to destination
    if let Err(error) = copy_files(Path::new(source), Path::new(destination)) {
        eprintln!("Error: {}", error);
    } else {
        println!("Files and directories copied successfully!");
    }
}

fn copy_files(source: &Path, destination: &Path) -> io::Result<()> {
    if !source.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Source file does not exist.",
        ));
    }

    let destination_path = destination.join(source.file_name().unwrap());
    // let destination_path = Path::new(destination_path);
    // if !destination_path.exists() {
    //     return Err(io::Error::new(
    //         kind: io::ErrorKind::NotFound,
    //         error: "The source file is already exists.",
    //     ));
    // }
    fs::copy(source, &destination_path)?;

    Ok(())
}

// Handle file conflicts when a file with the same name already exists in the destination directory. Provide options to overwrite, skip, or rename the file.
// Display progress information during the backup process, such as the number of files copied and the total size of the copied files.
// Implement error handling to handle cases such as invalid directories or file permissions.
// Use appropriate data structures and functions to organize your code effectively.