use std::collections::HashSet;

pub struct ChangeMachine {
    denominations: HashSet<usize>,
}

impl ChangeMachine {
    pub fn new(denominations: HashSet<usize>) -> ChangeMachine {
        ChangeMachine{ denominations }
    }

    pub fn get_num_coins(&self, amount: usize) -> u128 {
        let mut subproblems: Vec<u128> = Vec::new();
        subproblems.push(0);

        for i in 1..(amount + 1) {
            let mut potentials = Vec::new();
            
            for d in &self.denominations {
                if d <= &i {
                    let potential = subproblems[i - d] + 1;
                    potentials.push(potential);
                }
            }

            subproblems.push(
                match potentials.iter().min() {
                    Some(p) => *p,
                    None => 0
                }
            );
        }

        *subproblems.last().unwrap()
    }
}

