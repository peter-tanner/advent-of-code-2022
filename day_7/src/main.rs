use std::{cell::RefCell, fs::read_to_string, ops::Deref, rc::Rc};

#[derive(Eq, PartialEq)]
enum STATE {
    COMMAND,
    LISTING,
}

// fn process_dir<'a>(cd_path: &'a str, stack: &mut Vec<&'a str>) {
//     if cd_path.eq("..") {
//         stack.pop();
//     // WE CAN ASSUME THE ONLY ABSOLUTE PATH USED IS THE /
//     } else if cd_path.eq("/") {
//         stack.truncate(0);
//     } else {
//         stack.push(cd_path);
//     }
// }

struct File {
    name: String,
    size: u32,
}
struct Directory<'a> {
    name: &'a str,
    parent: Option<Rc<RefCell<Directory<'a>>>>,
    subdirs: Vec<Rc<RefCell<Directory<'a>>>>,
    files: Vec<File>,
}

impl<'a> Directory<'a> {
    fn new(name: &'a str, parent: Option<Rc<RefCell<Directory<'a>>>>) -> Directory<'a> {
        Directory {
            name,
            parent,
            subdirs: Vec::new(),
            files: Vec::new(),
        }
    }

    fn cd_mkdir(&'a mut self, name: &'a str) -> &Box<Directory> {
        for subdir in &self.subdirs {
            let t = subdir.deref().as_ptr().name;
            if subdir.deref().name.eq(name) {
                return subdir;
            }
        }
        let dir = Box::from(Directory::new(name, None));
        self.subdirs.push(dir);
        return &dir;
    }
}

const PATH: &str = "src/input";
fn main() {
    let binding = read_to_string(PATH).expect("Error reading file");
    assert!(binding.is_ascii());
    let history = binding.split("\n").into_iter();

    let root_directory = Box::from(Directory::new("root", None));
    let mut current_directory = root_directory;

    let mut state = STATE::COMMAND;

    for line in history {
        let mut tokens = line.split_ascii_whitespace();
        if tokens.next().unwrap().eq("$") {
            let cmd_token = tokens.next().unwrap();
            if cmd_token.eq("dir") {
                let directory = tokens.next().unwrap();
                let next_state: &Box<Directory>;
                if directory.eq("..") {
                    // next_state = &current_directory.parent.unwrap()
                } else {
                    // next_state = &;
                }
                current_directory = *current_directory.cd_mkdir(directory);
            } else if cmd_token.eq("ls") {
            } else {
                panic!("INVALID STATE");
            }
        }
    }
}
