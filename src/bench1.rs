fn main() {
    let iter = 10000000;
    let mut sum = 0;

    let mut x: Vec<i64> = Vec::new();
    let mut y: Vec<i64> = Vec::new();

    for i in 0..iter {
      x.push(i);
      y.push(i + 1);
    }

    for i in 0..iter {
      if i % 5 == 0 && x[i as usize] % 2 == 0 {
        sum += x[i as usize]
      }
    }

    for i in 0..iter {
      if i % 5 == 0 && y[i as usize] % 2 == 0 {
        sum += y[i as usize]
      }
    }

    println!("{}",sum);
}
