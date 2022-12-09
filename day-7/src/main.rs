use std::{
    borrow::Borrow,
    cell::{Ref, RefCell},
    collections::HashSet,
    fs,
    rc::Rc,
};

// struct Directory<'a> {
//     parent: &'a Directory<'a>,
//     size: u32,
//     name: &'a str,
//     children: Vec<Directory<'a>>,
// }

struct Node<'a> {
    pub size: Option<u32>,
    pub children: Vec<Rc<RefCell<Node<'a>>>>,
    pub parent: Option<Rc<RefCell<Node<'a>>>>,
    pub name: &'a str,
}

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Input read");
    println!("Input {:?}", contents);

    let root_node = Rc::new(RefCell::new(Node {
        // parent: ,
        name: "/",
        size: None,
        children: vec![],
        parent: None,
    }));
    // let root_node = Node {
    //     // parent: ,
    //     name: "/",
    //     size: None,
    //     children: HashSet::new(),
    //     parent: None,
    // };
    let mut current_node = Rc::clone(&root_node);

    let split_contents: Vec<Vec<Vec<&str>>> = contents
        .split("$")
        .filter(|line| !line.is_empty())
        .map(|execution| {
            let command: Vec<Vec<&str>> = execution
                .split("\n")
                .map(|line| {
                    let split_line: Vec<&str> =
                        line.split(" ").filter(|line| !line.is_empty()).collect();
                    return split_line;
                })
                .filter(|line| !line.is_empty())
                .collect();
            println!("Command {:?}", command);
            if command[0][0] == "cd" {
                if command[0][1] == "/" {
                    current_node = Rc::clone(&root_node);
                    println!("Going to root node");
                } else if command[0][1] == ".." {
                    println!("Going up one level");
                    let parent =
                        Rc::clone(&current_node.as_ref().borrow().parent.as_ref().unwrap());
                    current_node = parent;
                } else {
                    println!("Going to node {:?}", command[0][1]);
                    let next = Rc::clone(
                        current_node
                            .as_ref()
                            .borrow()
                            .children
                            .iter()
                            .filter(|child| child.as_ref().borrow().name == command[0][1])
                            .collect::<Vec<&Rc<RefCell<Node>>>>()
                            .first()
                            .unwrap(),
                    );
                    current_node = next;
                }
            }

            return command;
        })
        .collect();

    println!("Parsed? input {:?}", split_contents);

    let total_size: u32 = 0;

    // get_total_size(&root_node.);
    println!("Total size {:?}", total_size);
}

fn get_total_size(node: &Node) -> u32 {
    if node.size.unwrap_or_default() != 0 {
        return node.size.unwrap();
    }
    return node.children.iter().fold(0, |acc, inner_dir| {
        return acc + get_total_size(inner_dir.as_ref().borrow().borrow());
    });
}
