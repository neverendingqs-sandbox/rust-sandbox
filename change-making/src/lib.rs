use std::collections::HashSet;

pub struct ChangeMachine {
    denominations: HashSet<usize>,
    subproblems: Vec<u128>,
}

impl ChangeMachine {
    pub fn new(denominations: HashSet<usize>) -> ChangeMachine {
        if !denominations.contains(&1) {
            panic!("Must contain the denomination '1'.");
        }

        let subproblems: Vec<u128> = vec![0];
        ChangeMachine{ denominations, subproblems }
    }

    pub fn get_num_coins(&mut self, amount: usize) -> u128 {
        let cached_until = self.subproblems.len();
        if amount >= cached_until {
            for i in cached_until..(amount + 1) {
                let mut potentials = Vec::new();

                for d in &self.denominations {
                    if d <= &i {
                        let potential = self.subproblems[i - d] + 1;
                        potentials.push(potential);
                    }
                }

                self.subproblems.push(
                    match potentials.iter().min() {
                        Some(p) => *p,
                        None => 0
                    }
                );
            }
        }

        self.subproblems[amount]
    }

    pub fn get_change(&mut self, mut amount: usize) -> Vec<usize> {
        self.get_num_coins(amount);

        let mut change: Vec<usize> = Vec::new();
        while amount > 0 {
            for d in &self.denominations {
                if d <= &amount && self.subproblems[amount - d] == self.subproblems[amount] - 1 {
                    change.push(d.clone());
                    amount = amount - d;
                }
            }
        }

        change
    }
}

