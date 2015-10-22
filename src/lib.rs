extern crate array_tool;

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

//  pub fn slow_permutations(n: i32, k: i32) -> i32 {
//    if k > n { panic!("Aaah! First parameter must be the larger number!") };
//    factorial(arrayify(n)) / factorial(arrayify(n-k))
//  }

//  pub fn slow_combinatorial_count(n: i32, k: i32) -> i32 {
//    if k > n { panic!("Aaah! First parameter must be the larger number!") };
//    let m = arrayify(n);
//    let vals = array_tool::uniques(
//      m[arrayify(n-k).len()..m.len()].to_vec(),
//      arrayify(k)
//    );
//    factorial(vals[0].to_vec()) / factorial(vals[1].to_vec())
//  }


