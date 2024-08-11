use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Node {
    id: usize,
    color: usize,
    stopped: bool,
    iteration: usize,
    neighbors: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(id: usize) -> Self {
        let mut rng = rand::thread_rng();
        let color = rng.gen_range(1..=3);

        Node {
            id,
            color,
            stopped: false,
            iteration: 0,
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

    pub fn get_iteration(&self) -> usize {
        self.iteration
    }

    pub fn pick_and_send_color(&mut self) {
        if self.stopped {
            return;
        }

        let mut rng = rand::thread_rng();
        self.color = rng.gen_range(1..=3);
        self.iteration += 1;

        self.send_color();
    }

    pub fn send_color(&self) {
        // this should be api call, on other node this would trigger receive color
    }

    pub fn receive_color(&mut self, sender: Rc<RefCell<Node>>) {
        let sender_borrowed = sender.borrow();
        // Ensure the sender's iteration is aligned with the current node's iteration
        if !self.stopped && sender_borrowed.get_iteration() != self.iteration {
            return;
        }

        // Check if all neighbors are either stopped or in the same iteration
        let all_synced = self.neighbors.iter().all(|neighbor| {
            let neighbor = neighbor.borrow();
            neighbor.has_stopped() || neighbor.get_iteration() == self.iteration
        });

        // If all neighbors are synchronized, process the neighbors
        if all_synced {
            self.process_neighbors();
        }
    }

    pub fn process_neighbors(&mut self) {
        if self.stopped {
            return;
        }

        // Check for conflicts with any neighbor
        let conflict = self.neighbors.iter().any(|neighbor| neighbor.borrow().get_color() == self.color);

        if !conflict {
            self.stopped = true;
        }
    }

    pub fn print_state(&self) {
        println!("Node {}: color = {}, stopped = {}, iteration = {}", self.id, self.color, self.stopped, self.iteration);
    }
}