#![feature(libc)]

pub mod rust {
  pub fn arrayify(i: i32) -> Vec<i32> {
     (1..(i+1)).collect::<Vec<i32>>()
  }

  pub fn factorial(i: Vec<i32>) -> i32 {
    let x = i.iter().fold(1, |a, b| a * b);
    match x {
      0 => 1,
      _ => x
    }
  }

  pub fn permutations(n: i32, k: i32) -> i32 {
    if k > n { panic!("Aaah! First parameter must be the larger number!") };
    let m = arrayify(n);
    factorial(m[arrayify(n-k).len()..m.len()].to_vec())
  }

  pub fn combinatorial_count(n: i32, k: i32) -> i32 {
    if k > n { panic!("Aaah! First parameter must be the larger number!") };
    permutations(n,k) / factorial(arrayify(k))
  }
}

//  #![feature(cstr_memory)]
extern crate libc;
//
//  use std::ffi::CString;
//  use std::mem;
//
//  #[repr(C)]
//  pub struct RubyArray {
//    len: libc::size_t,
//    data: *const libc::c_void,
//  }
//
//  impl RubyArray {
//    fn from_vec<T>(vec: Vec<T>) -> RubyArray {
//      let array = RubyArray {
//        data: vec.as_ptr() as *const libc::c_void,
//        len: vec.len() as libc::size_t
//      };
//      mem::forget(vec);
//      array
//    }
//  }
//
//
//  #[no_mangle]
//  pub extern fn arrayify(i: i32) -> RubyArray {
//  }
//
//  #[no_mangle]
//  pub extern fn factorial(i: RubyArray) -> i32 {
//  }
//
//  pub struct TwoNumbers {
//    n: i32,
//    k: i32,
//  }

#[no_mangle]
pub extern fn permutations(n: i32, k: i32) -> i32 {
  rust::permutations(n, k)
}

#[no_mangle]
pub extern fn combinatorial_count(n: i32, k: i32) -> i32 {
  rust::combinatorial_count(n, k)
}
