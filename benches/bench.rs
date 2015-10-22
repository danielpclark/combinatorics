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
