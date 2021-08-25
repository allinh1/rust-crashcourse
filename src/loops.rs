pub fn run() {
    let mut count = 0;
  

    //infinite loop
    // loop {
    //     count += 1;
    //     println!("Number: {}", count);

    //     if count == 10 {
    //         break;
    //     }
    // }

    // while loop ( FizzBuzz )
    while count <= 15 {
        if count % 3 == 0 && count % 5 == 0 {
            println!("FizzBuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count)
        }
        count += 1;
    }
}


//   while count <= 100 {
//     if count % 15 == 0 {
//       println!("fizzbuzz");
//     } else if count % 3 == 0 {
//       println!("fizz");
//     } else if count % 5 == 0 {
//       println!("buzz")
//     } else {
//       println!("{}", count);
//     }
// }