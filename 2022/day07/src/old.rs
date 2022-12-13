#[derive(Debug, Default, Clone)]
struct Tree<T>
where
    T: PartialEq
{
    tree: Vec<Node<T>>,
}

#[derive(Debug, Clone)]
struct Node<T>
where
    T: PartialEq 
{
    idx: usize,
    val: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T>  Node<T>
where 
    T: PartialEq
{

    fn new(idx: usize, val: T) -> Self {
        Self {
            idx,
            val,
            parent: None,
            children: vec![],
        }
    }
}
impl<T> Tree<T>
where
    T: PartialEq
{
    fn node(&mut self, val: T) -> usize {
        //first see if it exists
        for node in &self.tree {
            if node.val == val {
                return node.idx;
            }
        }
        // Otherwise, add new node
        let idx = self.tree.len();
        self.tree.push(Node::new(idx, val));
        idx
    }
}

#[derive(PartialEq, Default, Debug, Clone)]
struct FileData {
    size: u32,
    name: String,
}

fn sum_childs(tree: Vec<Node<FileData>>, nodes: Node<FileData>) -> u32 {
   let mut sum = 0; 
    for node in nodes.children {
        if tree[node].val.name.starts_with("dir") {
            let mut childrens: Vec<&Node<FileData>> = vec![]; 
            for children in tree[node].children {  
                sum_childs(tree, tree[children]);
            }
        }
        else {
            sum += tree[node].val.size;
        }
    }
   sum 
}

fn sum_dir(nodes: Vec<Node<FileData>>) -> u32 {
    let mut sum = 0; 
    for node in &nodes {
        if node.val.name.starts_with("dir") {
            for children in &node.children {
                if nodes[*children].val.name.starts_with("dir") {
                    sum += sum_childs(nodes, nodes[*children]);
                }
            }
        } else {
            println!("Size files: {:?}", node.val.size);
            sum += node.val.size;
        }
    }
    return sum;

} 

fn first_part(file: String) -> u32 {
    let result = file
        .trim()
        .split("\n")
        .collect::<Vec<&str>>(); 
    
    let mut tree: Tree<FileData> = Tree::default();
    let folder_struct = FileData { size: 0, name: "dir Root".to_string() };

    let folder = tree.node(folder_struct);
    
    let mut current = folder;


    for r in result {
        if !(r.starts_with("$")) {
            let line = r.split(" ").collect::<Vec<&str>>();
            
            if line[0] =="dir" {
                continue
            }
            let file_struct = FileData { size: line[0].parse::<u32>().unwrap(), name: line[1].to_string() };
            let item = tree.node(file_struct);
            tree.tree[current].children.push(item);
            tree.tree[item].parent = Some(current);

            continue
        };
        
        let line = r.split("$").collect::<Vec<&str>>();
        let command_line = line[1].trim().split(" ").collect::<Vec<&str>>();
        
        if command_line.len() >= 2 && command_line[1] != "/" {
            if command_line[1] == ".." {
                current = tree.tree[current].parent.unwrap();
 
               continue
            }
            let folder_struct = FileData { size: 0, name: "dir ".to_owned() + &command_line[1].to_string() };

            let item = tree.node(folder_struct);
            tree.tree[item].parent = Some(current);
            tree.tree[current].children.push(item);
            current = item;
        }
        
    }
    let arr: Vec<u32> = vec![];
    println!("{}", sum_dir(tree.tree));
           //f.val.size += f.children
           //     .iter()
           //     .map(|index| {
           //         let node = &tree.tree[*index];
           //         let number = node.children.iter().map(|index| { tree.tree[*index].val.size }).sum::<u32>();
           //         node.val.size
           //     }).sum::<u32>(); 
    

    2
}

fn main() {
    let file = std::fs::read_to_string("./input.test")
        .expect("Couldn't read file!");
        
    first_part(file);
}
