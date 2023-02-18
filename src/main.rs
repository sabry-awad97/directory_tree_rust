use std::fs::{self};
use std::path::PathBuf;

enum EntryType {
    File,
    Directory,
}

struct Entry {
    name: String,
    entry_type: EntryType,
}

enum LineType {
    Single,
    Last,
}

struct Line {
    line_type: LineType,
    indent_level: usize,
}

struct TreePrinter {
    prefix: String,
    line_char: char,
}

impl TreePrinter {
    fn new(line_type: LineType, indent_level: usize) -> TreePrinter {
        let prefix = "│   ".repeat(indent_level);
        let line_char = match (line_type, indent_level) {
            (LineType::Single, 0) => ' ',
            (LineType::Single, _) => '├',
            (LineType::Last, _) => '└',
        };
        TreePrinter { prefix, line_char }
    }

    fn print_entry(&self, entry: &Entry) {
        let entry_line = format!("{}{}── {}", self.prefix, self.line_char, entry.name);
        println!("{}", entry_line);
    }

    fn print_subtree(&self, path: &PathBuf, line_type: LineType, indent_level: usize) {
        let entries = fs::read_dir(path)
            .unwrap()
            .map(|entry| {
                let entry = entry.unwrap();
                let name = entry.file_name().into_string().unwrap();
                let entry_type = if entry.path().is_dir() {
                    EntryType::Directory
                } else {
                    EntryType::File
                };
                Entry { name, entry_type }
            })
            .collect::<Vec<_>>();

        for (i, entry) in entries.iter().enumerate() {
            let printer = TreePrinter::new(
                if i == entries.len() - 1 {
                    LineType::Last
                } else {
                    LineType::Single
                },
                indent_level + 1,
            );
            printer.print_entry(entry);

            if let EntryType::Directory = entry.entry_type {
                let sub_path = path.join(&entry.name);
                printer.print_subtree(
                    &sub_path,
                    if i == entries.len() - 1 {
                        LineType::Last
                    } else {
                        LineType::Single
                    },
                    indent_level + 1,
                );
            }
        }
    }
}

fn print_directory_tree(path: &PathBuf) {
    let root = Entry {
        name: path.to_string_lossy().into_owned(),
        entry_type: EntryType::Directory,
    };
    let printer = TreePrinter::new(LineType::Last, 0);
    printer.print_entry(&root);
    printer.print_subtree(path, LineType::Last, 0);
}

fn main() {
    let path = PathBuf::from(".");
    print_directory_tree(&path);
}
