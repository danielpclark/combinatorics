cc = ->(n,k){

  ar = ->(i){
    i.downto(1).to_a
  }
    
  f = ->(a){
    b = a.reduce(:*).to_i
    b.zero? ? 1 : b
  }
    
  m = ar.(n)
  l = ar.(k)
  f.(m-l)/(f.(l-m)*f.(ar.(n-k)))
}
