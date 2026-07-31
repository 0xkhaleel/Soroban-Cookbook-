use super::*;
use soroban_sdk::{symbol_short, Env};

fn setup(env: &Env) -> PriorityQueueContractClient<'_> {
    let contract_id = env.register_contract(None, PriorityQueueContract);
    env.mock_all_auths();
    PriorityQueueContractClient::new(env, &contract_id)
}

#[test]
fn test_push_peek_pop_max_returns_highest_priority() {
    let env = Env::default();
    let client = setup(&env);

    client.push(&symbol_short!("low"), &1);
    client.push(&symbol_short!("medium"), &5);
    client.push(&symbol_short!("high"), &10);

    assert_eq!(client.peek_max(), Some(symbol_short!("high")));
    assert_eq!(client.pop_max(), symbol_short!("high"));
    assert_eq!(client.pop_max(), symbol_short!("medium"));
    assert_eq!(client.pop_max(), symbol_short!("low"));
    assert!(client.is_empty());
}

#[test]
fn test_len_and_is_empty_after_insertions() {
    let env = Env::default();
    let client = setup(&env);

    assert!(client.is_empty());
    client.push(&symbol_short!("first"), &3);
    client.push(&symbol_short!("second"), &2);

    assert_eq!(client.len(), 2);
    assert!(!client.is_empty());
}

/// Heap integrity: whatever order items go in, they must come out by
/// descending priority, and the root must always hold the maximum.
#[test]
fn test_heap_integrity_across_interleaved_pushes_and_pops() {
    let env = Env::default();
    let client = setup(&env);

    let items = [
        (symbol_short!("a"), 4i128),
        (symbol_short!("b"), 9),
        (symbol_short!("c"), 1),
        (symbol_short!("d"), 7),
        (symbol_short!("e"), 3),
        (symbol_short!("f"), 8),
    ];
    for (item, priority) in items.iter() {
        client.push(item, priority);
        // The root is the maximum priority pushed so far.
        let max_so_far = client.all().iter().map(|e| e.priority).max().unwrap();
        assert_eq!(client.all().get(0).unwrap().priority, max_so_far);
    }

    // Draining yields strictly descending priorities.
    let mut previous = i128::MAX;
    while !client.is_empty() {
        let expected_len = client.len() - 1;
        let top = client.all().get(0).unwrap().priority;
        assert!(
            top <= previous,
            "heap order violated: {top} after {previous}"
        );
        previous = top;
        client.pop_max();
        assert_eq!(client.len(), expected_len);
    }
}

#[test]
#[should_panic(expected = "Empty priority queue")]
fn test_pop_max_on_empty_queue_panics() {
    let env = Env::default();
    let client = setup(&env);
    client.pop_max();
}
