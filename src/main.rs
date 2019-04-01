use std::io;

fn main() {
  let mut f0: u64 = 0;
  let mut f1: u64 = 1;
  let mut index = 1;
  let mut result: u64 = 0;

  let mut n = String::new();

  println!("Print the nth Fibonacci number");
  println!("Please enter n:");

  io::stdin().read_line(&mut n)
    .expect("Unable to read input");

  let n: u32 = n.trim().parse()
    .expect("Unable to parse n to u32");

  if n == 0 {
    println!("The f0 fibonacci number is 0");
    return;
  }

  if n == 1 {
    println!("The f1 fibonacci number is 1");
    return;
  }

  while index < n {
    result = f0 + f1;

    if f0 < f1 {
      f0 = result;
    } else {
      f1 = result;
    }

    index = index + 1;
  }

  println!("The f{} fibonacci number is {}", n, result);
}
