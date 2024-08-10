use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Node {
    id: usize,
    color: usize,
    stopped: bool,
    neighbors: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(id: usize) -> Self {
        let mut rng = rand::thread_rng();
        let color = rng.gen_range(1..=3);  // Randomly pick a color from {1, 2, 3}
        
        // If we assume that we can get all required data from another crate, 
        // we would most likely populate list of neighbours on creation 
        Node {
            id,
            color,
            stopped: false,
            neighbors: vec![],
        }
    }

    pub fn add_neighbor(&mut self, neighbor: Rc<RefCell<Node>>) {
        self.neighbors.push(neighbor);
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_color(&self) -> usize {
        self.color
    }

    pub fn has_stopped(&self) -> bool {
        self.stopped
    }

    pub fn pick_and_send_color(&mut self) { 
        if self.stopped {
            return;
        }

        let mut rng = rand::thread_rng();
        self.color = rng.gen_range(1..=3);  // Pick a new random color

        self.send_color()
    }

    pub fn send_color(&self) {
        // here we would inform other nodes that we have updated our color.
        // Also in case of having some central component that would handle synchronization between nodes 
        // Notify neighbors of the new color
        for _neighbor in &self.neighbors {
            // here we would invoke call to crate that manages sending data to other nodes
            //let mut neighbor_borrowed = neighbor.borrow_mut();
            //neighbor_borrowed.receive_color(self.id, self.color);
        }
    }

    pub fn receive_color(&mut self, _sender_id: usize, _sender_color: usize) {
        // Update color directly without nested mutable borrows
        // This code will not be used, but demostrantes how we cound hypotetically react on receiving color
        // in this implementation we could have issues with race conditions

        // if let Some(neighbor) = self.neighbors.iter().find(|n| n.borrow().get_id() == sender_id) {
        //     neighbor.borrow_mut().color = sender_color;
        // }
    
        // // Now process neighbors after receiving color
        // self.process_neighbors();
    }

    pub fn process_neighbors(&mut self) {
        // Processing of neigbours should wait until
        // all nodes that have not stopped pick_and_send_collor, we can discuss how we would approach this 
        if self.stopped {
            return;
        }

        // Check if any neighbor has the same color
        let conflict = self.neighbors.iter().any(|neighbor| neighbor.borrow().get_color() == self.color);

        if !conflict {
            // potentially here we should notify that this node is done, and that it wont be processed in future
            // this would play a part in determingin when not finished nodes can start picking colors again
            self.stopped = true;
        }
    }

    pub fn print_state(&self) {
        println!("Node {}: color = {}, stopped = {}", self.id, self.color, self.stopped);
    }
}
