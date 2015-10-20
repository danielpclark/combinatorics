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

// WORKS
//pub fn permutations(n: i32, k: i32) -> i32 {
//  factorial(arrayify(n)) / factorial(arrayify(n-k))
//}

pub fn permutations(n: i32, k: i32) -> i32 {
  let m = arrayify(n);
  factorial(m[arrayify(n-k).len()..m.len()].to_vec())
}

pub fn combinatorial_count(n: i32, k: i32) -> i32 {
  // n must always be larger than k

//  let arrayify = |i: i32| -> Vec<i32> {
//    (1..i).collect::<Vec<i32>>()
//  };

//  let factorial = |i: Vec<i32>| -> i32 {
//    let x = i.iter().fold(1, |a, b| a * b);
//    match x {
//      0 => 1,
//      _ => x
//    }
//  };

  //assert_eq!(factorial(arrayify(4)), 24);
  //assert_eq!(factorial(arrayify(5)), 120);
  
//  let m = arrayify(n);
//  let l = arrayify(k);

//  let permutations = {
//    let slice: Vec<i32> = m[l.len()..m.len()].to_vec();
//    factorial(slice)
//  };

  permutations(n,k) / factorial(arrayify(k)) // factorial(l)
}
