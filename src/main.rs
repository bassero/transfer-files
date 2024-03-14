use std::fs;
use std::io;
use std::path::{Path, PathBuf};
#[allow(dead_code)]
#[allow(unused_imports)]
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
    let destination = "D:\\download";
    // Call the function to copy files from source to destination
    if let Err(error) = copy_files(Path::new(source), Path::new(destination)) {
        eprintln!("Error: {}", error);
    } else {
        println!("Files and directories copied successfully!");
    }
}

fn copy_files(source: &Path, destination: &Path) -> io::Result<()> {
    let file_name = source.file_name().unwrap();
    let destination_path = destination.join(file_name);

    if destination_path.exists() {
        println!("File with the same name already exists in the destination directory.");
        println!("Choose an option:");
        println!("1. Overwrite the existing file");
        println!("2. Skip copying the file");
        println!("3. Rename the file");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");
        let choice = choice.trim();

        match choice {
            "1" => {
                fs::copy(source, &destination_path)?;
                println!("File overwritten successfully!");
            }
            "2" => {
                println!("File skipped!");
            }
            "3" => {
                let new_file_name = get_unique_file_name(&destination_path);
                let new_destination_path = destination.join(new_file_name);
                fs::copy(source, &new_destination_path)?;
                println!("File renamed and copied successfully!");
            }
            _ => {
                println!("Invalid choice. File skipped!");
            }
        }
    } else {
        fs::copy(source, &destination_path)?;
        println!("File copied successfully!");
    }

    Ok(())
}

fn get_unique_file_name(destination_path: &Path) -> String {
    let file_stem = destination_path.file_stem().unwrap().to_string_lossy();
    let file_extension = destination_path.extension().unwrap_or_default().to_string_lossy();

    let mut suffix = 1;
    let mut new_file_name = format!("{}_{}.{}", file_stem, suffix, file_extension);
    let mut new_destination_path = destination_path.with_file_name(&new_file_name);

    while new_destination_path.exists() {
        suffix += 1;
        new_file_name = format!("{}_{}{}", file_stem, suffix, file_extension);
        new_destination_path = destination_path.with_file_name(&new_file_name);
    }

    new_file_name
}
// Handle file conflicts when a file with the same name already exists in the destination directory. Provide options to overwrite, skip, or rename the file.
// Display progress information during the backup process, such as the number of files copied and the total size of the copied files.
// Implement error handling to handle cases such as invalid directories or file permissions.
// Use appropriate data structures and functions to organize your code effectively.