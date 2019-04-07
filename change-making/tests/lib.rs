#![allow(non_snake_case)]
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
#[should_panic]
fn Constructor_NoOneDenomination_Panics() {
    let denominations = create_denominations(vec![2]);
    change_making::ChangeMachine::new(denominations);
}

#[test]
fn GetNumCoins_AmountIsZero_ReturnsZero() {
    let denominations = create_denominations(vec![1]);
    let mut sut = change_making::ChangeMachine::new(denominations);

    assert_eq!(0, sut.get_num_coins(0));
}

#[test]
fn GetNumCoins_AmountIsExactlyEqualToADenomination_ReturnsOne() {
    let coins = vec![1, 5, 10, 25, 100, 200];
    let denominations = create_denominations(coins.clone());
    let mut sut = change_making::ChangeMachine::new(denominations);

    for c in coins {
        assert_eq!(1, sut.get_num_coins(c), "Coin: {}", c);
    }
}

#[test]
fn GetNumCoins_CanadianDenomination_ReturnsValidNumberOfCoins() {
    let denominations = create_denominations(vec![1, 5, 10, 25, 100, 200]);
    let mut sut = change_making::ChangeMachine::new(denominations);

    let testcases: Vec<(usize, u128)> = vec![
        ( 4, 4 ),
        ( 9, 5 ),
        ( 42, 5 ),
        ( 50, 2 ),
        ( 68, 7 ),
        ( 230, 3 ),
        ( 250, 3 ),
        ( 330, 4 ),
        ( 500, 3 ),
    ];

    for (amount, expect_num_coins) in testcases.iter() {
        assert_eq!(
            *expect_num_coins,
            sut.get_num_coins(*amount),
            "amount: {}; expected_num_coins: {}", amount, expect_num_coins
        );
    }
}
