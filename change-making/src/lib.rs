pub struct ChangeMachine<'a> {
    denominations: &'a [u128],
}

impl<'a> ChangeMachine<'a> {
    pub fn new(denominations: &[u128]) -> ChangeMachine {
        ChangeMachine{ denominations }
    }

    pub fn get_change(&self, amount: u128) -> &[u128] {
        return self.denominations;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_returns_denominations() {
        let denominations = [1, 2, 3];
        let sut = ChangeMachine::new(&denominations);

        assert_eq!(denominations, sut.get_change(0));
    }
}
