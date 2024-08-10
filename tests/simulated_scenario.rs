use randomized_3_coloring::Node;
use std::cell::RefCell;
use std::rc::Rc;

fn run_coloring_test(num_nodes: usize) {
    // Create `num_nodes` nodes with unique IDs
    let nodes: Vec<Rc<RefCell<Node>>> = (0..num_nodes)
        .map(|id| Rc::new(RefCell::new(Node::new(id))))
        .collect();

    // println!("Initial state of each node:");
    // for node in &nodes {
    //     node.borrow().print_state();
    // }

    // Connect nodes in a linear fashion
    for i in 0..(num_nodes - 1) {
        let node = nodes[i].clone();
        let next_node = nodes[i + 1].clone();
        node.borrow_mut().add_neighbor(next_node.clone());
        next_node.borrow_mut().add_neighbor(node);
    }

    let mut rounds = 0;

    while nodes.iter().any(|node| !node.borrow().has_stopped()) && rounds < 100 {
        //println!("Round {}: ", rounds);

        for node in &nodes {
            node.borrow_mut().pick_and_send_color();
        }

        for node in &nodes {
            node.borrow_mut().process_neighbors();
        }

        // // Print the state of each node after the round
        // for node in &nodes {
        //     node.borrow().print_state();
        // }
        // println!();

        rounds += 1;
    }

    // Verify that no two adjacent nodes have the same color
    for i in 0..(num_nodes - 1) {
        assert_ne!(
            nodes[i].borrow().get_color(),
            nodes[i + 1].borrow().get_color(),
            "Nodes {} and {} have the same color",
            nodes[i].borrow().get_id(),
            nodes[i + 1].borrow().get_id(),
        );
    }

    // Print final colors and number of rounds taken
    println!("Finished coloring {} nodes after {} rounds", num_nodes, rounds);
    // for node in &nodes {
    //     node.borrow().print_state();
    // }
}


#[test]
fn test_coloring_with_2_nodes() {
    run_coloring_test(2);
}

#[test]
fn test_coloring_with_10_nodes() {
    run_coloring_test(10);
}

#[test]
fn test_coloring_with_100_nodes() {
    run_coloring_test(100);
}

#[test]
fn test_coloring_with_1000_nodes() {
    run_coloring_test(1000);
}

#[test]
fn test_coloring_with_10000_nodes() {
    run_coloring_test(10000);
}

