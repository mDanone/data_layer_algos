pub struct Node {
    val: String,
    child: Box<Node>
}

impl Node {
    fn new(val: String, child: Node) -> Self{
        Node {
            val,
            child: Box::new(child)
        }
    }
}



// fn build_states_map(number_of_regis)