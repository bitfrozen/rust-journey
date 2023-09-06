fn main() {
  // simple
  let number = 1;
  if number < 3 {
    println!("condition was true");
  } else {
    println!("condition was false");
  }

  // flat
  let condition = true;
  let number = if condition { 5 } else { 6 };
  println!("The value of number is: {number}");

  // assign
  let x1;
  let cond = true;
  if cond {
    x1 = 1;
  } else {
    x1 = 2;
  }
  let x2 = if cond { 1 } else { 2 };

  if x1 == x2 {
    println!("x values are the same {x1}");
  } else {
    println!("x values are different {x1} and {x2}");
  }

  // loop
  let mut counter = 0;
  let result = loop {
    counter += 1;
    if counter == 10 {
      break counter * 2;
    }
  };
  println!("The result of loop is {result}");

  // goto
  let mut count = 0;
  'counting_up: loop {
    println!("count = {count}");
    let mut remaining = 10;

    loop {
      println!("remaining = {remaining}");
      if remaining == 9 {
        break;
      }
      if count == 2 {
        break 'counting_up;
      }
      remaining -= 1;
    }

    count += 1;
  }
  println!("End count = {count}");

  // while
  let mut number = 3;
  while number != 0 {
    println!("{number}!");

    number -= 1;
  }
  println!("LIFTOFF!!!");

  // range 1
  let a = [10, 20, 30, 40, 50];
  for element in a {
    println!("the value is: {element}");
  }

  // range 2
  for number in (1..4).rev() {
    println!("{number}!");
  }
  println!("LIFTOFF!!!");

  // sum
  let a = [5; 10];
  let mut sum = 0;
  for x in a {
    sum += x;
  }
  println!("{sum}");
}
