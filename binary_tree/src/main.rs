#[derive(Debug)]
struct Tree {
    root: Option<Box<Node>>,
}

#[derive(Debug)]
struct Node {
    value: i32,
    left : Option<Box<Node>>,
    right : Option<Box<Node>>,
}

impl From<Node> for Option<Box<Node>> {
    fn from(node: Node) -> Self {
        Some(Box::new(node))
    }
}

impl Tree {
    fn new() -> Self {
        Tree { root: None }
    }

    fn insert(&mut self, value: i32) {
        match &mut self.root {
            None => {
                self.root = Node::new(value).into();
            },
            Some(node) => {
                Tree::insert_recursive(node, value);
            }
        }
    }

    fn insert_recursive(node:&mut Box<Node>, value:i32){
        if value > node.value {
            match &mut node.right {
                None => {
                    node.right = Node::new(value).into();
                }
                Some(n) => {
                    Tree::insert_recursive(n, value)
                }
            }
        } else if value < node.value {
            match &mut node.left {
                None => {
                    node.left = Node::new(value).into();
                }
                Some(n) => {
                    Tree::insert_recursive(n, value)
                }
            }

        }
    }
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None
        }
    }
}



fn main() {
    let mut tree = Tree::new();
    tree.insert(8);
    tree.insert(10);
    tree.insert(3);
    tree.insert(1);
    tree.insert(6);
    tree.insert(4);

    
    println!("The three is: {:?}", tree);
}
