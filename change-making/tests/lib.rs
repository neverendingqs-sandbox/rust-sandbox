use std::collections::HashSet;

use change_making;

fn create_denominations(coins: Vec<usize>) -> HashSet<usize> {
    let mut denominations: HashSet<usize> = HashSet::new();
    for c in coins {
        denominations.insert(c);
    }

    denominations
}

#[test]
fn test_returns_denominations() {
    let denominations = create_denominations(vec![1, 5, 10, 25, 100]);
    let sut = change_making::ChangeMachine::new(denominations);

    assert_eq!(7, sut.get_num_coins(124));
}

#[test]
fn amountiszero_returns_zero() {
    let denominations = create_denominations(vec![1, 5, 10, 25, 100]);
    let sut = change_making::ChangeMachine::new(denominations);

    assert_eq!(0, sut.get_num_coins(0));
}
