// Copyright 2015-2016 Daniel P. Clark & Other Combinatorics Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate combinatorics;

use combinatorics::rust::*;

#[test]
fn it_arrayifies(){
  assert_eq!(arrayify(4), vec![1,2,3,4]);
  assert_eq!(arrayify(5), vec![1,2,3,4,5])
}

#[test]
fn it_factorials(){
  assert_eq!(factorial(vec![1,2,3,4]), 24);
  assert_eq!(factorial(vec![1,2,3,4,5] ), 120)
}

#[test]
fn it_permutates(){
  assert_eq!(permutations(6, 4), 360);
}

#[test]
fn it_combinatorials(){
  assert_eq!(combinatorial_count(6, 4), 15);
  assert_eq!(combinatorial_count(8, 2), 28);
  assert_eq!(combinatorial_count(18, 3), 816);
  assert_eq!(combinatorial_count(42, 5), 850668)
}

#[test]
#[should_panic]
fn it_barfs(){
  assert_eq!(permutations(4, 6), 360);
  assert_eq!(combinatorial_count(4, 6), 15);
}
