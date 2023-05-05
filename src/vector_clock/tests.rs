use crate::vector_clock::*;

#[test]
fn test_clock_can_increment() {
    let mut clock = VectorClock::new(5);
    clock.increment(1);
    clock.increment(3);
    assert_eq!(vec![0, 1, 0, 1, 0], clock.clock);
}

#[test]
fn test_clocks_can_merge() {
    let mut clock1 = VectorClock::new(5);
    clock1.increment(1);
    clock1.increment(1);
    clock1.increment(3);

    let mut clock2 = VectorClock::new(5);
    clock2.increment(2);
    clock2.increment(2);
    clock2.increment(3);

    clock1.merge(&clock2);

    assert_eq!(vec![0, 2, 2, 1, 0], clock1.clock);
}

#[test]
fn test_merged_clocks_are_equal() {
    let mut clock1 = VectorClock::new(5);
    clock1.increment(1);
    clock1.increment(1);
    clock1.increment(3);

    let mut clock2 = VectorClock::new(5);
    clock2.increment(2);
    clock2.increment(2);
    clock2.increment(3);

    clock1.merge(&clock2);
    clock2.merge(&clock1);

    assert_eq!(clock1, clock2);
}

#[test]
fn test_clocks_can_be_compared() {
    let mut clock1 = VectorClock::new(5);
    clock1.increment(1);
    clock1.increment(3);

    let mut clock2 = VectorClock::new(5);
    clock2.increment(1);
    clock2.increment(1);
    clock2.increment(3);

    assert!(clock1 < clock2);
    assert!(clock1 <= clock2);
    assert!(clock2 > clock1);
    assert!(clock2 >= clock1);
    assert!(clock1 != clock2);
}

#[test]
fn test_concurrent_clocks_cannot_be_compared() {
    let mut clock1 = VectorClock::new(5);
    clock1.increment(1);
    clock1.increment(3);
    clock1.increment(4);

    let mut clock2 = VectorClock::new(5);
    clock2.increment(1);
    clock2.increment(1);
    clock2.increment(3);

    assert!(!(clock1 < clock2));
    assert!(!(clock1 > clock2));
    assert!(clock1 != clock2);
}
