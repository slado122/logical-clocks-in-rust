use crate::lamport_clock::*;

#[test]
fn test_clocks_are_updated_on_request() {
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    node1.send(&mut node2);
    assert_eq!(3, node1.lamport_clock.latest_time);
    assert_eq!(2, node2.lamport_clock.latest_time);
}
