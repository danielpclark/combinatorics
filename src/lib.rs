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
