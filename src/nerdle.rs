use std::cmp;
use rand::Rng;
use rand::seq::SliceRandom; // 0.7.2


// Terms
// nod: num of digits


// Notes
// Generate equation by end to start


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

    fn to_borrowed(&self) -> Bounds {
        Bounds {
            min: self.min,
            max: self.max
        }
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

    fn flip_non_valid(&self) -> Bounds {
        Bounds {
            min: cmp::min(self.min, self.max),
            max: cmp::max(self.min, self.max),
        }
    }

    fn make_positive(&self) -> Bounds {
        Bounds {
            min: cmp::max(self.min, 0),
            max: cmp::max(self.max, 0),
        }
    }

    fn is_valid(&self) -> bool {
        self.min <= self.max
    }

    fn random_value(&self) -> i32 {
        rand::thread_rng().gen_range(self.min..self.max + 1)
    }
}

#[derive(Clone)]
struct Expression {
    nods: Vec<i32>,
}

#[derive(Clone, Copy)]
enum Op {
    Add,
    Subtract,
    Multiply,
    Divide
}

pub trait Operation {
    fn get_restricted_bounds(answer_bounds: &Bounds, rest_bounds: &Bounds) -> Bounds;
    fn undo_op();
}

// pub struct AddOperation { }
// impl Operation for AddOperation {
//     fn get_restricted_bounds(answer_bounds: &Bounds, rest_bounds: &Bounds) -> Bounds {
//             let restricted_add_bounds_1 = Bounds {
//                 min: answer_bounds.min - rest_bounds.min,
//                 max: answer_bounds.max - rest_bounds.min,
//             }.flip_non_valid();

//             let restricted_add_bounds_2 = Bounds {
//                 min: answer_bounds.min - rest_bounds.max,
//                 max: answer_bounds.max - rest_bounds.max,
//             }.flip_non_valid();

//             Bounds::union_many(&vec![&restricted_add_bounds_1, &restricted_add_bounds_2])
//     }
// }

// pub struct SubtractOperation { }
// impl Operation for SubtractOperation {
//     fn get_restricted_bounds(answer_bounds: &Bounds, rest_bounds: &Bounds) -> Bounds {
//         let restricted_sub_bounds_1 = Bounds {
//             min: rest_bounds.min - answer_bounds.min,
//             max: rest_bounds.min - answer_bounds.max,
//         }.flip_non_valid();

//         let restricted_sub_bounds_2 = Bounds {
//             min: rest_bounds.max - answer_bounds.min,
//             max: rest_bounds.max - answer_bounds.max,
//         }.flip_non_valid();

//         Bounds::union_many(&vec![&restricted_sub_bounds_1, &restricted_sub_bounds_2])
//     }
// }


impl Expression {

    fn to_borrowed(&self) -> Expression {
        Expression {
            nods: self.nods.to_owned(),
        }
    }

    fn gen_all_of_length(len: i32) -> Vec<Expression> {
        let mut exps = vec![];

        let mut queue = vec![];

        for n in 1..len-1 {
            let ex = Expression {
                nods: vec![n]
            };
            queue.push(ex);
        }

        // Len 6
        // First Gen [ [1], [2], [3], [4] ]
        // Second Gen [ [1,1], [1,2], [1,3]  ]

        loop {
            let mut next_gen_queue = vec![];
            for ex in queue.to_owned() {
                let current_len = ex.char_length();
                let left = len - current_len;
                for nod in 1..left {
                    let new_exp = ex.append(nod);
                    let new_exp_len = new_exp.char_length();

                    if  new_exp_len == len {
                        exps.push(new_exp.clone());
                    }
                    if new_exp_len < len {
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

    // Generate random structure of size len
    fn from_length(len: i32) -> Option<Expression> {
        let all_exp = Expression::gen_all_of_length(len);
        let t = all_exp.choose(&mut rand::thread_rng());
        match t {
            Some(i) => Some(i.to_owned()),
            None => None
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
        let mut output = String::new();
        for nod in self.nods.to_owned() {
            for _ in 0..nod {
                output.push_str("x");
            }
            output.push_str("*");
        }
        output.pop();
        println!("{}", output);
    }

    fn char_length(&self) -> i32 {
        let mut sum = 0;
        for nod in self.nods.to_owned() {
            sum += nod
        }
        sum + (self.nods.len() as i32) - 1
    }

    fn output_bounds(&self) -> Bounds {
        if self.nods.len() == 0 {
            print!("Malformed Equality");
        }

        if self.nods.len() == 1 {
            return Bounds::from_nod(self.nods[0]);
        }

        fn op_output_bounds(bounds1: Bounds, bounds2: Bounds) -> Bounds {
            let add_bounds = Bounds {
                min: bounds1.min + bounds2.min,
                max: bounds1.max + bounds2.max,
            };
            let sub_bounds = Bounds {
                min: bounds1.min - bounds2.max,
                max: bounds1.max - bounds2.min,
            };
            let mul_bounds = Bounds {
                min: bounds1.min * bounds2.min,
                max: bounds1.max * bounds2.max,
            };
            Bounds::union_many(&vec![&add_bounds, &sub_bounds, &mul_bounds])
        }

        let mut b1 = Bounds::from_nod(self.nods[0]);
        for i in 1..self.nods.len() {
            b1 = op_output_bounds(b1, Bounds::from_nod(self.nods[i]));
        }
        b1
    }

    // Remove the first {amount} of elements from self
    fn slice(&self, amount: usize) -> Expression {
        Expression {
            nods: self.nods[0..amount].to_vec()
        }
    }

    fn gen_rand(&self, answer_bounds: Bounds) -> Option<Equation> {
        // Generate a number in the chosen_value_bounds
        let mut rng = rand::thread_rng();

        let mut filled_eq = Equation {
            numbers: vec![],
            ops: vec![],
        };

        let mut cum_answer_bounds = answer_bounds.to_borrowed();

        for i in 0..self.nods.len()-1 {

            // Get the nod from the end of the array
            let nod = self.nods[self.nods.len()-(i+1)];

            let rest = self.slice((self.nods.len() - (i+1)) as usize);

            let rest_bounds = rest.output_bounds();

            // Generate a number and op that when applied to rest will yeld a number in the answer range

            let output_bounds = Bounds::from_nod(nod);

            let restricted_sub_bounds_1 = Bounds {
                min: rest_bounds.min - cum_answer_bounds.min,
                max: rest_bounds.min - cum_answer_bounds.max,
            }.flip_non_valid();

            let restricted_sub_bounds_2 = Bounds {
                min: rest_bounds.max - cum_answer_bounds.min,
                max: rest_bounds.max - cum_answer_bounds.max,
            }.flip_non_valid();

            let restricted_add_bounds_1 = Bounds {
                min: cum_answer_bounds.min - rest_bounds.min,
                max: cum_answer_bounds.max - rest_bounds.min,
            }.flip_non_valid();

            let restricted_add_bounds_2 = Bounds {
                min: cum_answer_bounds.min - rest_bounds.max,
                max: cum_answer_bounds.max - rest_bounds.max,
            }.flip_non_valid();

            let restricted_mul_bounds_1 = Bounds {
                min: if rest_bounds.min != 0 {
                    cum_answer_bounds.min / rest_bounds.min
                } else {
                    // Doesn't matter since we are multiplying by 0
                    i32::MIN
                },
                max: if rest_bounds.max != 0 {
                    cum_answer_bounds.max / rest_bounds.max
                } else {
                    i32::MAX
                }
            }.flip_non_valid();

            let restricted_mul_bounds_2 = Bounds {
                min: if rest_bounds.max != 0 {
                    cum_answer_bounds.min / rest_bounds.max
                } else {
                    // Doesn't matter since we are multiplying by 0
                    i32::MIN
                },
                max: if rest_bounds.min != 0 {
                    cum_answer_bounds.max / rest_bounds.min
                } else {
                    i32::MAX
                }
            }.flip_non_valid();

            let restricted_add_bounds = Bounds::union_many(&vec![&restricted_add_bounds_1, &restricted_add_bounds_2]).intersect(&output_bounds);
            let restricted_sub_bounds = Bounds::union_many(&vec![&restricted_sub_bounds_1, &restricted_sub_bounds_2]).intersect(&output_bounds);
            let restricted_mul_bounds = Bounds::union_many(&vec![&restricted_mul_bounds_1, &restricted_mul_bounds_2]).intersect(&output_bounds);

            let mut possible_op_bounds = vec![];

            if restricted_add_bounds.is_valid() {
               possible_op_bounds.push((restricted_add_bounds, Op::Add));
            }

            if restricted_sub_bounds.is_valid() {
                possible_op_bounds.push((restricted_sub_bounds, Op::Subtract));
            }

            if restricted_mul_bounds.is_valid() {
                possible_op_bounds.push((restricted_mul_bounds, Op::Multiply));
            }

            if possible_op_bounds.len() == 0 {
                return None;
            }

            // Choose random bounds from possible
            let (chosen_bounds, op) = possible_op_bounds.choose(&mut rng).unwrap();

            let num = chosen_bounds.random_value();

            // Undo the operation to the answer bounds
            cum_answer_bounds = match op {
                Op::Add => Bounds {
                    min: cum_answer_bounds.min - num,
                    max: cum_answer_bounds.max - num,
                },
                Op::Subtract => Bounds {
                    min: cum_answer_bounds.min + num,
                    max: cum_answer_bounds.max + num,
                },
                Op::Multiply => Bounds {
                    min: cum_answer_bounds.min / num,
                    max: cum_answer_bounds.max / num,
                },
                _ => panic!("Invalid op")
            };

            // Ensure the answer bounds are positive
            cum_answer_bounds = cum_answer_bounds.make_positive();


            filled_eq.numbers.push(num);

            filled_eq.ops.push(*op);
        }

        let nod = self.nods[0];
        let end_bounds = cum_answer_bounds.intersect(&Bounds::from_nod(nod));

        if !end_bounds.is_valid() {
            return None;
        }

        let num = end_bounds.random_value();
        filled_eq.numbers.push(num);

        // We filled the array backwards so it is flipped
        filled_eq.numbers.reverse();

        Some(filled_eq)
    }

}

struct Equation {
    numbers: Vec<i32>,
    ops: Vec<Op>
}

impl Equation {
    fn print(&self) {
        let mut eq_str = String::new();
        for i in 0..self.numbers.len() {
            let num = self.numbers[i];
            let op = self.ops.get(i);
            eq_str.push_str(&num.to_string());
            match op {
                Some(Op::Add) => eq_str.push_str("+"),
                Some(Op::Subtract) => eq_str.push_str("-"),
                Some(Op::Multiply) => eq_str.push_str("*"),
                None => eq_str.push_str("?"),
                _ => (),
            };
        }
        eq_str.pop();
        eq_str.push_str("=");
        eq_str.push_str(&self.calc_value().to_string());
        println!("{}", eq_str);
    }

    fn calc_value(&self) -> i32 {
        let mut val = self.numbers[0];
        for i in 1..self.numbers.len() {
            let num = self.numbers[i];
            let op = self.ops.get(i-1);
            match op {
                Some(Op::Add) => val += num,
                Some(Op::Subtract) => val -= num,
                Some(Op::Multiply) => val *= num,
                None => (),
                _ => (),
            };
        }
        val
    }

    fn try_make_random(len: u8) -> Option<Equation> {
        let mut rng = rand::thread_rng();

        let answer_max_nod: u8 = 3;

        let answer_nod = rng.gen_range(1..answer_max_nod+1);
        // let answer_nod = 1;

        println!("{}", answer_nod);

        let answer_bounds = Bounds::from_nod(answer_nod as i32);

        let nod_left = len - (answer_nod+1);

        let expression = Expression::from_length(nod_left as i32);

        // let expression = Some(Expression {
        //     nods: vec![1,1,1]
        // });

        match expression {
            Some(exp) => {
                exp.print();
                exp.gen_rand(answer_bounds)
            }
            None => None
        }
    }

    fn make_random(len: u8) -> Equation {
        loop {
            match Equation::try_make_random(len) {
                Some(eq) => return eq,
                None => (),
            }
        }
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
}


pub fn main() {
    let eq = Equation::make_random(8);
    eq.print();
}

