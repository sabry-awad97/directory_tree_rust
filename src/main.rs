use std::fs;
use std::path::PathBuf;

enum FileSystemNode {
    File(String),
    Directory(String, Vec<FileSystemNode>),
}

struct FileSystemTree {
    root: FileSystemNode,
    line_char: char,
    indent_size: usize,
}

impl FileSystemTree {
    fn new(root_path: &PathBuf) -> Self {
        let root_name = root_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let root = FileSystemNode::Directory(root_name, Vec::new());
        Self {
            root,
            line_char: '├',
            indent_size: 4,
        }
    }

    fn build(&mut self) {
        let root_path = self.root.path();
        let mut stack = vec![(root_path.clone(), &mut self.root)];

        while let Some((path, node)) = stack.pop() {
            let entries = fs::read_dir(&path).unwrap();
            let mut children = Vec::new();

            for entry in entries {
                let entry_path = entry.unwrap().path();
                let entry_name = entry_path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned();

                let child = if entry_path.is_dir() {
                    FileSystemNode::Directory(entry_name, Vec::new())
                } else {
                    FileSystemNode::File(entry_name)
                };

                children.push(child);
            }

            node.set_children(children);

            for child in node.children_mut() {
                if let FileSystemNode::Directory(_, _) = child {
                    let child_path = child.path();
                    stack.push((child_path.clone(), child));
                }
            }
        }
    }

    fn print(&self) {
        self.print_node(&self.root, "");
    }

    fn print_node(&self, node: &FileSystemNode, indent: &str) {
        match node {
            FileSystemNode::File(name) => {
                println!("{}{}── {}", indent, self.line_char, name);
            }
            FileSystemNode::Directory(name, children) => {
                println!("{}{}── {}", indent, self.line_char, name);
                let last_index = children.len() - 1;
                for (i, child) in children.iter().enumerate() {
                    let line_char = if i == last_index { '└' } else { '├' };
                    let child_indent = format!(
                        "{:indent_size$}{}",
                        "",
                        line_char,
                        indent_size = self.indent_size
                    );
                    self.print_node(child, &(indent.to_owned() + &child_indent));
                }
            }
        }
    }
}

impl FileSystemNode {
    fn path(&self) -> PathBuf {
        match self {
            FileSystemNode::File(name) => PathBuf::from(name),
            FileSystemNode::Directory(name, _) => PathBuf::from(name),
        }
    }

    fn set_children(&mut self, children: Vec<FileSystemNode>) {
        if let FileSystemNode::Directory(_, existing_children) = self {
            *existing_children = children;
        }
    }

    fn children_mut(&mut self) -> &mut Vec<FileSystemNode> {
        if let FileSystemNode::Directory(_, children) = self {
            children
        } else {
            panic!("Cannot get children of file node");
        }
    }
}

fn main() {
    let path = PathBuf::from("src");
    let mut tree = FileSystemTree::new(&path);

    tree.build();

    // print the tree to the console
    tree.print();
}
