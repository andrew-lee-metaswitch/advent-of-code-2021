// use crate::util;
// use std::str::FromStr;
// //use std::collections::{HashMap, HashSet};
// use std::fmt;

// #[derive(Clone, Debug, PartialEq, Eq)]
// struct SnailFishNumber {
//     lhs: Option<Box<SnailFishNumber>>,
//     lhs_int: Option<i8>,
//     rhs: Option<Box<SnailFishNumber>>,
//     rhs_int: Option<i8>,
// }

// impl fmt::Display for SnailFishNumber {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Write strictly the first element into the supplied output
//         // stream: `f`. Returns `fmt::Result` which indicates whether the
//         // operation succeeded or failed. Note that `write!` uses syntax which
//         // is very similar to `println!`.
//         let lhs_display = match self.lhs_int {
//             Some(i) => format!("{}", i),
//             None => match &self.lhs {
//                 Some(x) => format!("{}", x),
//                 None => "None".to_string(),
//             },
//         };
//         let rhs_display = match self.rhs_int {
//             Some(i) => format!("{}", i),
//             None => match &self.rhs {
//                 Some(x) => format!("{}", x),
//                 None => "None".to_string(),
//             },
//         };

//         write!(f, "[ {},  {}]", lhs_display, rhs_display)
//     }
// }

// impl SnailFishNumber {
//     fn add(&self, other: &SnailFishNumber) -> SnailFishNumber {
//         let mut b = SnailFishNumber {
//             lhs: Some(Box::new(self.clone())),
//             lhs_int: None,
//             rhs:  Some(Box::new(other.clone())),
//             rhs_int: None,
//         };
//         b.reduce(1);
//         b

//     }

//     fn kids(&self) -> Vec<SnailFishNumber> {
//         let rc:Vec<SnailFishNumber>  = vec![];
//         if let Some(lhs) = self.lhs {
//             rc.push(*lhs)
//         }
//         if let Some(rhs) = self.rhs {
//             rc.push(*rhs)
//         }
//         rc
//     }

//     fn check_for_explosions(&self, nesting_level: i8) -> [Option(i8);2] {
//         if nesting_level == 4 {
//             if let Some(lhs) = self.lhs {
//                 ///lhs will explode
//                 ///
//                 rc = [lhs.lhs_int.unwrap(), lhs.rhs_int.unwrap()];

//                 self.lhs = None;
//                 self.lhs_int = Some(0);

//                 if let Some(rhs_int) = self.rhs_int {
//                     rhs += rc[1];
//                     return [rc[0], None]
//                 }

//                 return rc
//             }

//         } else {
//             for kid in self.kids.iter() {
//                 kid.check_for_explosions(nesting_level+1)
//             }
//         }

//     }

//     fn reduce(&mut self, nesting_level: i8) {
//         for kid in self.kids.iter() {
//             kid.check_for_explosions(nesting_level+1)
//         }
//         if nesting_level == 4 {
//             if let Some(lhs) = self.lhs {
//                 ///lhs will explode
//             }

//         }

//             // Finally
//             self.lhs = None;
//             self.lhs_int = Some(0);
//         }

//     }

//     fn magnitute(&self) -> i32 {
//         let lhs_magntiude = match &self.lhs {
//             Some(lhs) => lhs.magnitute(),
//             None => self.lhs_int.unwrap() as i32,
//         };
//         let rhs_magntiude = match &self.rhs {
//             Some(rhs) => rhs.magnitute(),
//             None => self.rhs_int.unwrap() as i32,
//         };
//         3 * lhs_magntiude + 2 * rhs_magntiude
//     }
// }

// impl FromStr for SnailFishNumber {
//     type Err = regex::Error;
//     fn from_str(input: &str) -> Result<Self, Self::Err> {
//         let mut lhs: Vec<char> = vec![];
//         let mut rhs: Vec<char> = vec![];
//         let mut stack: Vec<char> = vec![];
//         let mut lhs_complete: bool = false;
//         for (pos, c) in input.chars().enumerate() {
//             if !lhs_complete {
//                 if c == '[' {
//                     stack.push(c);
//                     lhs.push(c);
//                 } else if c == ']' {
//                     if stack.last().unwrap() == &'[' {
//                         stack.pop();
//                     } else {
//                         stack.push(c);
//                     }
//                     lhs.push(c);
//                 } else if c == ',' && stack == vec!['['] {
//                     // we've found the LHS!
//                     lhs_complete = true;
//                     // remove the first '[' from lhs
//                     lhs.remove(0);
//                     rhs = input[pos + 1..input.chars().count() - 1].chars().collect();
//                     break;
//                 } else {
//                     lhs.push(c);
//                 }
//             };
//         }

//         let lhs_as_str = lhs.into_iter().collect::<String>();
//         let rhs_as_str = rhs.into_iter().collect::<String>();

//         let lhs_int: Option<i8> = match i8::from_str(&lhs_as_str) {
//             Ok(i) => Some(i),
//             Err(_e) => None,
//         };
//         let rhs_int: Option<i8> = match i8::from_str(&rhs_as_str) {
//             Ok(i) => Some(i),
//             Err(_e) => None,
//         };

//         let lhs_opt: Option<Box<SnailFishNumber>> = match lhs_int {
//             Some(_i) => None,
//             None => Some(Box::new(SnailFishNumber::from_str(&lhs_as_str).unwrap())),
//         };

//         let rhs_opt: Option<Box<SnailFishNumber>> = match rhs_int {
//             Some(_i) => None,
//             None => Some(Box::new(SnailFishNumber::from_str(&rhs_as_str).unwrap())),
//         };

//         Ok(SnailFishNumber {
//             lhs: lhs_opt,
//             lhs_int,
//             rhs: rhs_opt,
//             rhs_int,
//         })
//     }
// }

// fn part_one(mut numbers: Vec<SnailFishNumber>) {
//     let mut sum_sf = numbers.pop().unwrap();
//     for sf in numbers.iter() {
//         sum_sf = sum_sf.add(sf);
//     }
//     println!("The answer to part one is {}", sum_sf.magnitute());
// }

// pub(crate) fn day18() {
//     // Load inputs from input directory
//     let snail_fish_numbers: Vec<SnailFishNumber> = util::load_inputs("18".to_string())
//         .iter()
//         .map(|v| SnailFishNumber::from_str(v).unwrap())
//         .collect();
//     for sf in snail_fish_numbers {
//         println!("{}", sf);
//     }
// }
// //part_one(snail_fish_numbers);
