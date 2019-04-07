#![allow(non_snake_case)]
use itertools;
use itertools::Itertools;
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

    for (amount, expect_num_coins) in testcases {
        assert_eq!(
            expect_num_coins,
            sut.get_num_coins(amount),
            "amount: {}; expected_num_coins: {}", amount, expect_num_coins
        );
    }
}

#[test]
fn GetChange_AmountIsExactlyEqualToADenomination_ReturnsCoin() {
    let coins = vec![1, 5, 10, 25, 100, 200];
    let denominations = create_denominations(coins.clone());
    let mut sut = change_making::ChangeMachine::new(denominations);

    for c in coins {
        let actual = sut.get_change(c);
        assert_eq!(1, actual.len(), "Coin: {}", c);
        assert_eq!(c, actual[0], "Coin: {}", c);
    }
}

#[test]
fn GetChange_CanadianDenominationAndValidAmount_ReturnsCorrectChange() {
    let denominations = create_denominations(vec![1, 5, 10, 25, 100, 200]);
    let mut sut = change_making::ChangeMachine::new(denominations);

    let testcases: Vec<(usize, Vec<usize>)> = vec![
        ( 4, vec![1, 1, 1, 1] ),
        ( 9, vec![5, 1, 1, 1, 1] ),
        ( 42, vec![25, 10, 5, 1, 1] ),
        ( 50, vec![25, 25] ),
        ( 68, vec![25, 25, 10, 5, 1, 1, 1] ),
        ( 230, vec![200, 25, 5] ),
        ( 250, vec![200, 25, 25] ),
        ( 330, vec![200, 100, 25, 5] ),
        ( 500, vec![200, 200, 100] ),
    ];

    for (amount, expected) in testcases {
        let actual = sut.get_change(amount);

        itertools::assert_equal(
            expected.iter().sorted(),
            actual.iter().sorted()
        );
    }
}

#[test]
fn GetChange_IrregularDenominationAndValidAmount_ReturnsCount() {
    let testcases: Vec<(usize, HashSet<usize>, Vec<usize>)> = vec![
        (5, create_denominations(vec![1, 2, 3]), vec![2, 3]),    // contains two valid subproblem paths
        (6, create_denominations(vec![1, 3, 4]), vec![3, 3]),
        (7, create_denominations(vec![1, 3, 4, 5]), vec![4, 3]),
        (194, create_denominations(vec![1, 3, 5, 10, 12, 50, 100]), vec![100, 50, 12, 12, 10, 10]),
    ];

    for (amount, denominations, expected) in testcases {
        let mut sut = change_making::ChangeMachine::new(denominations);
        let actual = sut.get_change(amount);

        itertools::assert_equal(
            expected.iter().sorted(),
            actual.iter().sorted()
        );
    }
}
