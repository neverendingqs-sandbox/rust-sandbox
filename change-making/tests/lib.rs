use change_making;

#[test]
fn test_returns_denominations() {
    let denominations = [1, 2, 3];
    let sut = change_making::ChangeMachine::new(&denominations);

    assert_eq!(denominations, sut.get_change(0));
}
