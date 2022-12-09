use std::{borrow::Borrow, cell::RefCell, fs, ops::AddAssign, rc::Rc};

struct Node<'a> {
    pub size: Option<u32>,
    pub children: Vec<Rc<RefCell<Node<'a>>>>,
    pub parent: Option<Rc<RefCell<Node<'a>>>>,
    pub name: &'a str,
}

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Input read");

    let root_node = Rc::new(RefCell::new(Node {
        name: "/",
        size: None,
        children: vec![],
        parent: None,
    }));
    let mut current_node = Rc::clone(&root_node);

    contents
        .split("$")
        .filter(|line| !line.is_empty())
        .for_each(|execution| {
            let command: Vec<Vec<&str>> = execution
                .split("\n")
                .map(|line| {
                    let split_line: Vec<&str> =
                        line.split(" ").filter(|line| !line.is_empty()).collect();
                    return split_line;
                })
                .filter(|line| !line.is_empty())
                .collect();
            // println!("Command {:?}", command);
            if command[0][0] == "cd" {
                if command[0][1] == "/" {
                    current_node = Rc::clone(&root_node);
                    // println!("Going to root node");
                } else if command[0][1] == ".." {
                    // println!("Going up one level");
                    let parent =
                        Rc::clone(&current_node.as_ref().borrow().parent.as_ref().unwrap());
                    current_node = parent;
                } else {
                    // println!("Going to node {:?}", command[0][1]);
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

            if command[0][0] == "ls" {
                // println!("Listing elements in directory {:?}", command);
                for index in 1..command.len() {
                    // println!("Command {:?}", command[index]);
                    if command[index][0] == "dir" {
                        // println!("{:?} is a dir, creating node", command[index][1]);
                        let node = Rc::new(RefCell::new(Node {
                            name: command[index][1],
                            size: None,
                            children: vec![],
                            parent: Option::Some(Rc::clone(&current_node)),
                        }));
                        current_node.as_ref().borrow_mut().children.push(node);
                        // println!("Node {:?}", node.as_ref().borrow_mut().name);
                    } else {
                        // println!(
                        //     "{:?} is a file, creating node with no children",
                        //     command[index][1]
                        // );
                        let node = Rc::new(RefCell::new(Node {
                            name: command[index][1],
                            size: Option::Some(command[index][0].parse::<u32>().unwrap()),
                            children: vec![],
                            parent: Option::Some(Rc::clone(&current_node)),
                        }));
                        current_node.as_ref().borrow_mut().children.push(node);
                        // println!("Node {:?}", node.as_ref().borrow_mut().name);
                    }
                }
            }
        });
    let total_with_condition: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    let total_size = get_total_size(&root_node, &total_with_condition);
    println!(
        "Total size {:?}, with condition is {:?}",
        total_size,
        total_with_condition.as_ref().borrow_mut()
    );
}

fn get_total_size(node: &Rc<RefCell<Node>>, total_with_condition: &Rc<RefCell<u32>>) -> u32 {
    let cloned_node = Rc::clone(node);
    let borrowed_mut = cloned_node.as_ref().borrow_mut();
    let size = borrowed_mut.size.unwrap_or_default();
    if size != 0 {
        println!("Size for {:?} is {:?}", borrowed_mut.name, size);
        return size;
    }

    let dir_size = borrowed_mut.children.iter().fold(0, |acc, inner_dir| {
        return acc + get_total_size(inner_dir, total_with_condition);
    });
    if borrowed_mut.size.borrow().is_none() {
        println!(
            "Directory {:?} is of size {:?}",
            borrowed_mut.name, dir_size
        );
    }
    if dir_size <= 100000 {
        total_with_condition
            .as_ref()
            .borrow_mut()
            .add_assign(dir_size);
    }
    return dir_size;
}
