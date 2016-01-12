// Copyright 2015-2016 Daniel P. Clark & Other Combinatorics Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![feature(test)]

extern crate test;
use test::Bencher;
extern crate combinatorics;
use combinatorics::*;

#[bench]
fn perm(b: &mut Bencher){
  b.iter(|| { 
    permutations(100000, 3567) 
  })
}

#[bench]
fn cc(b: &mut Bencher){
  b.iter(|| { 
    combinatorial_count(100000, 3567) 
  })
}
