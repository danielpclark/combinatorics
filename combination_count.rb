<<LICENSE
// Copyright 2015-2016 Daniel P. Clark & Other Combinatorics Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
LICENSE
require 'ffi'

module Rust
  extend FFI::Library
  ffi_lib './target/release/libcombinatorics.so'

  attach_function :permutations, [:int32, :int32], :int32 
  attach_function :combinatorial_count, [:int32, :int32], :int32 
end

arrayify = ->(i){
  i.downto(1).to_a
}

factorial = ->(a){
  b = a.reduce(:*).to_i
  b.zero? ? 1 : b
}

permutations = ->(n, k){
  m = arrayify.(n)
  factorial.(m[arrayify.(n-k).length..-1])
}

cc = ->(n,k){
  permutations.(n,k) / factorial.(arrayify.(k))
}

require 'benchmark/ips'

bench_reps = 1

Benchmark.ips do |x|
  [12500,1723,500,230].each do |qty|
    x.report("permutations of        (#{qty}, 160) in Rust") {
      bench_reps.times do
        Rust.permutations(qty, 160)
      end
    }
    x.report("permutations of        (#{qty}, 160) in Ruby") {
      bench_reps.times do
        permutations.(qty, 160)
      end
    }

    x.report("combinatorial count of (#{qty}, 160) in Rust") {
      bench_reps.times do
        Rust.combinatorial_count(6,4)
      end
    }
    x.report("combinatorial count of (#{qty}, 160) in Ruby") {
      bench_reps.times do
        cc.(qty, 160)
      end
    }
  end
end

require 'minitest/autorun'

