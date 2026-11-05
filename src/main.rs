fn main() {
    let state_one = StateNode {state: 0b00001111, next: None};
    let state_two = StateNode {state: 0b11110000, next: Some(Box::new(state_one))};
    let state_three = StateNode::chain_node(0b10101010, state_two);
    println!("{}",state_three.state);
    let state_four = StateNode::new(0b01010101);
        {
        let mut list = LinkedList::new();
        list.add(0b11111101);
        list.add(0b11111100);
        list.add(0b11111010);
        list.add(0b11111001);
        list.add(0b11111000);
        let mut curr = &list.head;
            while let Some(node) = curr{ // Some doesn't take value none so it stops
            println!{"Node value is: {}",node.getValue()};
            curr = &node.next;
        }
    
    }
}




struct StateNode {
    state: u8,
    next: Option<Box<StateNode>>
}


impl StateNode{
    fn new(state: u8) -> StateNode{
        return StateNode {state: state, next: None};
    }
    fn chain_node(state: u8, next: StateNode) -> StateNode{
        return StateNode {state: state, next: Some(Box::new(next))};
    }

    fn getValue(&self) -> u8 {
        return self.state;
    }
}

struct LinkedList {
    head: Option<Box<StateNode>>
}

impl LinkedList{
    fn new() -> LinkedList {
        return LinkedList {head: None};
    }

    fn add(&mut self, state: u8){
        let new_node = StateNode {state: state, next: self.head.take()};
        self.head = Some(Box::new(new_node));
    }
}

impl Drop for LinkedList {
    fn drop(&mut self){
        self.head = None;
        println!("Dropped");
    }
}