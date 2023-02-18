use std::fs;
use std::path::Path;

enum EntryType {
    File,
    Directory,
}

struct Entry {
    name: String,
    entry_type: EntryType,
}

#[derive(Clone)]
enum LineType {
    Single,
    Last,
}

#[derive(Clone)]
struct Line {
    line_type: LineType,
    indent_level: usize,
}

fn print_directory_tree(path: &Path, indent_level: usize, line: Line) {
    let mut entries: Vec<Entry> = Vec::new();
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name().into_string().unwrap();
        let entry_type = if entry.path().is_dir() {
            EntryType::Directory
        } else {
            EntryType::File
        };
        entries.push(Entry { name, entry_type });
    }

    for (index, entry) in entries.iter().enumerate() {
        let is_last_entry = index == entries.len() - 1;
        let line_char = match (line.clone().line_type, is_last_entry) {
            (LineType::Single, false) => "├",
            (LineType::Single, true) => "└",
            (LineType::Last, false) => "│",
            (LineType::Last, true) => " ",
        };
        let prefix = "│   ".repeat(indent_level);
        let entry_line = format!("{}{}── {}", prefix, line_char, entry.name);

        println!("{}", entry_line);

        let new_indent_level = indent_level + 1;
        let new_line = match (line.clone().line_type, is_last_entry) {
            (LineType::Single, false) => Line {
                line_type: LineType::Single,
                indent_level: new_indent_level,
            },
            (LineType::Single, true) => Line {
                line_type: LineType::Last,
                indent_level: new_indent_level,
            },
            (LineType::Last, false) => Line {
                line_type: LineType::Single,
                indent_level: new_indent_level,
            },
            (LineType::Last, true) => Line {
                line_type: LineType::Last,
                indent_level: new_indent_level,
            },
        };

        if let EntryType::Directory = entry.entry_type {
            let sub_path = path.join(&entry.name);
            print_directory_tree(&sub_path, new_indent_level, new_line);
        }
    }
}

fn main() {
    let path = Path::new(".");
    print_directory_tree(
        path,
        0,
        Line {
            line_type: LineType::Last,
            indent_level: 0,
        },
    );
}
