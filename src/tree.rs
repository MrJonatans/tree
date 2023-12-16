use std::fmt;

#[derive(Debug)]
pub struct Tree{
    root : Option<Box<Node>>,
} 

#[derive(Debug)]
struct Node{
    left : Option<Box<Node>>,
    right : Option<Box<Node>>,
    val : i64,
}

impl Node {
    fn new(val : i64) -> Self {
        Node{
            left : None,
            right : None,
            val,
        }
    }


    fn print_tree(node :& Option<Box<Node>>, tab_val : i64) -> fmt::Result{
        match &node{
            None => {
                Ok(())
            },
            Some(_node) => {
                let _ = Node::print_tree(&_node.left, tab_val+1);
                for _n in 1..=tab_val {print!(" ")};
                println!("{} ",_node.val);
                let _ = Node::print_tree(&_node.right,tab_val+1);
                Ok(())
            }
        }
    }
}

impl From<Node> for Option<Box<Node>>{
    fn from(node : Node) -> Self {
        Some(Box::new(node))
    }
}

impl fmt::Display for Tree{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.root{
            None =>{
                writeln!(f, "You stupid 🙂 go out NOW")
            },
            Some(_node)=>{
                Node::print_tree(&self.root, 0)
            }
        }
    }
}


impl Tree{
    pub fn new() -> Self {
        Tree{root: None}
    }

    pub fn insert(&mut self, val: i64){
        match &mut self.root{
            None => {
                self.root = Node::new(val).into();
            },
            Some(node) =>{
                 Tree::rec_insert(node, val);
            }
         }
    }

    fn rec_insert(node : &mut Box<Node>, val: i64) {
        if val < node.val {
            match &mut node.left {
                None => {
                    node.left = Node::new(val).into();
                },
                Some(_node) => {
                    Tree::rec_insert(_node, val);
                }
            }
        } else if val > node.val {
            match &mut node.right {
                None => {
                    node.right = Node::new(val).into();
                },
                Some(_node) => {
                    Tree::rec_insert(_node, val);
                }
            }
        }
    }
}


