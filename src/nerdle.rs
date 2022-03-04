use std::cmp;
use rand::Rng;

// Terms
// nod: num of digits

#[derive(Debug, PartialEq, Copy, Clone)]
struct Bounds {
    min: i32,
    max: i32,
}

impl Bounds {

    fn from_nod(nod: i32) -> Bounds {
        Bounds {
            min: (10 as i32).pow((nod - 1) as u32) - (if nod == 1 {1} else {0}),
            max: (10 as i32).pow(nod as u32) - 1,
        }
    }

    fn from_number(n: i32) -> Bounds {
    //    return Bounds::from_nod(n.log10() as i32);
       return Bounds::from_nod(n as i32);
    }

    fn print(&self) {
        println!("Min: {}, Max: {}", self.min, self.max);
    }

    /// Returns bounds that the target and source bounds share
    fn intersect(&self, b: &Bounds) -> Bounds {
        Bounds {
            min: cmp::max(self.min, b.min),
            max: cmp::min(self.max, b.max),
        }
    }

    fn union_many(bounds: &Vec<&Bounds>) -> Bounds {
        let min = bounds.into_iter().map(|b| {b.min}).min().unwrap();
        let max = bounds.into_iter().map(|b| {b.max}).max().unwrap();
        Bounds {
            min,
            max
        }
    }

    fn union(&self, b: &Bounds) -> Bounds {
        Bounds {
            min: cmp::min(self.min, b.min),
            max: cmp::max(self.max, b.max),
        }
    }

    fn is_valid(&self) -> bool {
        self.min < self.max
    }


    fn restrict_by_rest_and_result(&self, rest: Bounds, result: Bounds) -> Bounds {
        Bounds {
            min: 0,
            max: 0,
        }
    }

}

#[derive(Clone)]
struct Expression {
    nods: Vec<i32>,
}

enum Op {
    Add,
    Subtract,
    Multipy,
    Divide
}

impl Expression {

    fn to_borrowed(&self) -> Expression {
        Expression {
            nods: self.nods.to_owned(),
        }
    }

    fn gen_all_of_length(len: i32) -> Vec<Expression> {
        let mut exps = vec![];

        let mut queue = vec![];

        for n in 1..len-2 {
            let ex = Expression {
                nods: vec![n]
            };
            queue.push(ex);
        }


        loop {
            let mut next_gen_queue = vec![];
            for ex in queue.to_owned() {
                let current_len = ex.char_length();
                let left = len - current_len;
                for nod in 1..left-2 {
                    let new_exp = ex.append(nod);
                    if new_exp.char_length() == len {
                        exps.push(new_exp.clone());
                    }
                    if new_exp.char_length() < len {
                        next_gen_queue.push(new_exp);
                    }
                }
            }
            if next_gen_queue.len() == 0 {
                break;
            }
            queue = next_gen_queue
        }

        exps.to_owned()
    }

    // TODO
    // Generate random structure of size len
    fn from_length(len: i32) -> Expression {
        Expression {
            nods: vec![],
        }
    }

    fn append(&self, nod: i32) -> Expression {
        let mut new = Expression {
            nods: self.nods.to_owned()
        };
        new.nods.push(nod);
        new
    }

    fn print(&self) {
        for nod in self.nods.to_owned() {
            print!("{}", nod);
        }
    }

    fn char_length(&self) -> i32 {
        let mut sum = 0;
        for nod in self.nods.to_owned() {
            sum += nod
        }
        sum + (self.nods.len() as i32)
    }

    fn output_bounds(&self) -> Bounds {
       if self.nods.len() < 2 {
            print!("Malformed Equality");
       }
       let nod1 = self.nods[0];
       let nod2 = self.nods[1];

       let bounds_add = add_value_bounds(nod1, nod2);
       let bounds_sub = sub_value_bounds(nod1, nod2);
       let bounds_mul = mul_value_bounds(nod1, nod2);


       Bounds::union_many(&vec![&bounds_add, &bounds_sub, &bounds_mul])
    }


    //TODO
    // We need to restrict the output bounds of self to something that when oped with the rest
    // yeilds answer bounds
    // self op rest \inside_of answer_bound
    fn restrict_output_bounds(&self, rest_bounds: Bounds, answer_bounds: Bounds) -> Bounds {
        Bounds {
            min: 0,
            max: 0
        }
    }

    // TODO
    // Remove the first {amount} of elements from self
    fn slice(&self, amount: i32) -> Expression {
        Expression {
            nods: vec![]
        }
    }

    //TODO
    fn do_op(&self, output_bounds: Bounds) -> Equation {
        Equation {
            numbers: vec![],
            ops: vec![]
        }
    }

    fn gen_rand(&self, answer_bounds: Bounds) -> Equation {
        // Do the first op
        let nod1 = self.nods[0];
        let nod2 = self.nods[1];

        let eq1 = Expression {
            nods: vec![nod1, nod2],
        };

        let rest: Expression = self.slice(2);

        let restricted_output_bounds = eq1.restrict_output_bounds(rest.output_bounds(), answer_bounds);

        let filled_eq = eq1.do_op(restricted_output_bounds);

        // Loop over the rest, adding to the filled_eq
        for nod in rest.nods {
            filled_eq.do_op(nod, answer_bounds);
        }

        //Procedure
        //Take first two nods, Find what the bounds of the operations that restrict this to the bounds of the answer
        //  Will have to take the rest of the equality in account
        // With that result, get the next op and repeat the procdure.

        filled_eq
    }

}

struct Equation {
    numbers: Vec<i32>,
    ops: Vec<Op>
}

impl Equation {
    // TODO
    fn do_op(&self, nod: i32, answer_bounds: Bounds) {

    }

    // TODO
    fn calc_value(&self) -> i32 {
        0
    }

    fn make_random(len: u8) -> Equation {
        let mut rng = rand::thread_rng();

        let answer_max_nod: u8 = 3;

        let answer_nod = rng.gen_range(1..answer_max_nod+1);

        let answer_bounds = Bounds {
            min: i32::pow(10, (answer_nod - 1) as u32),
            max: i32::pow(10, answer_nod as u32)  - 1,
        };

        let nod_left = len - answer_nod;

        let expression = Expression::from_length(nod_left as i32);

        let equation = expression.gen_rand(answer_bounds);

        equation
    }
}




fn add_value_bounds(nod1: i32, nod2: i32) -> Bounds {
    let nod1_bounds = Bounds::from_nod(nod1);
    let nod2_bounds = Bounds::from_nod(nod2);
    Bounds {
       min: nod1_bounds.min + nod2_bounds.min,
       max: nod1_bounds.max + nod2_bounds.max,
    }
}

fn mul_value_bounds(nod1: i32, nod2: i32) -> Bounds {
    let nod1_bounds = Bounds::from_nod(nod1);
    let nod2_bounds = Bounds::from_nod(nod2);
    Bounds {
       min: nod1_bounds.min * nod2_bounds.min,
       max: nod1_bounds.max * nod2_bounds.max,
    }
}

fn sub_value_bounds(nod1: i32, nod2: i32) -> Bounds {
    let nod1_bounds = Bounds::from_nod(nod1);
    let nod2_bounds = Bounds::from_nod(nod2);
    Bounds {
       min: nod1_bounds.min - nod2_bounds.max,
       max: nod1_bounds.max - nod2_bounds.min,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nod_bounds() {
        let b1 = Bounds::from_nod(1);
        assert_eq!(b1, Bounds {
            min: 0,
            max: 9
        });
        let b2 = Bounds::from_nod(2);
        assert_eq!(b2, Bounds {
            min: 10,
            max: 99
        });
    }

    #[test]
    fn equality_bound() {
        let eq1 = Expression {
            nods: vec![1,1],
        };
        let b1 = eq1.max_bounds();
        assert_eq!(b1, Bounds {
            min: -9,
            max: 81
        });

        let eq2 = Expression {
            nods: vec![3, 2],
        };
        let b2 = eq2.max_bounds();
        assert_eq!(b2, Bounds {
            min: 100 - 99,
            max: 999 * 99,
        });
    }


    #[test]
    fn bounds_intersect() {
        let b1 = Bounds { min: 4, max: 6};
        let b2 = Bounds { min: 3, max: 5};
        let b3 = Bounds { min: 6, max: 9};

        let b4 = b1.intersect(&b2);
        let b5 = b2.intersect(&b3);

        assert_eq!(b4.min, 4);
        assert_eq!(b4.max, 5);
        assert_eq!(b5.is_valid(), false);
    }

    #[test]
    fn bounds_union() {
        let b1 = Bounds { min: 4, max: 6};
        let b2 = Bounds { min: 3, max: 5};

        let b4 = b1.union(&b2);

        assert_eq!(b4.min, 3);
        assert_eq!(b4.max, 6);
    }

    #[test]
    fn test_add_value_bounds() {
        let b1 = add_value_bounds(1, 1);
        assert_eq!(b1, Bounds {
            min: 0,
            max: 18
        });
        let b2 = add_value_bounds(1, 2);
        assert_eq!(b2, Bounds {
            min: 10,
            max: 99 + 9
        });
    }

    #[test]
    fn test_mul_value_bounds() {
        let b1 = mul_value_bounds(1, 1);
        assert_eq!(b1, Bounds {
            min: 0,
            max: 81
        });

        let b2 = mul_value_bounds(1, 2);
        assert_eq!(b2, Bounds {
            min: 0,
            max: 99 * 9,
        })
    }

    #[test]
    fn test_sub_value_bounds() {
        let b1 = sub_value_bounds(1, 1);
        assert_eq!(b1, Bounds {
            min: -9,
            max: 9
        });

        let b2 = sub_value_bounds(2, 1);
        assert_eq!(b2, Bounds {
            min: 1,
            max: 99
        });
    }



}


pub fn main() {
    //let eq = Equation::make_random(8);

    let expresions = Expression::gen_all_of_length(6);
    for exp in expresions {
        exp.print();
    }


}

