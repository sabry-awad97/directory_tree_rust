use std::fs;
use std::path::Path;

fn print_directory_tree(path: &Path, prefix: &str) {
    let dir_contents = fs::read_dir(path).unwrap().collect::<Vec<_>>();

    for (index, entry) in dir_contents.iter().enumerate() {
        let entry = entry.as_ref().unwrap();
        let entry_path = entry.path();
        let entry_name = entry_path.file_name().unwrap().to_str().unwrap();

        // Print the prefix for this entry, with appropriate characters depending on whether it's the last entry in the directory
        let (line_char, branch_char) = if index == dir_contents.len() - 1 {
            ("└", " ")
        } else {
            ("├", "│")
        };
        println!("{}{}── {}", prefix, line_char, entry_name);

        // Recursively print the contents of any subdirectories
        if entry_path.is_dir() {
            let new_prefix = format!("{}{}   ", prefix, branch_char);
            print_directory_tree(&entry_path, &new_prefix);
        }
    }
}

fn main() {
    let path = Path::new(".");
    print_directory_tree(&path, "");
}
