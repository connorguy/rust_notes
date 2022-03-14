// stevedonovan.github.io/rust-gentle-intro
use std::env;
use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let answer = -42;
    println!("Hello, world! {}", abs(answer as f64));

    let fact = 10;
    println!("{}", factorial(fact));

    let arr = [1, 2, 3, 4, 5];
    let arr_res = count_array(&arr);
    println!("arr:{}", arr_res);

    println!("Sum {}", sum_to(10));

    // Vectors are dynamically allocated arrays that only store one type
    let mut test_vec = Vec::new(); // have to use mut if you want to push things onto the vector
    test_vec.push(104);
    test_vec.push(200);
    test_vec.push(30);
    test_vec.push(12);
    test_vec.push(34);
    test_vec.push(34);
    test_vec.push(40);
    // In place dedup and sort
    test_vec.dedup();
    test_vec.sort();
    window_view(&test_vec);

    string_fun();

    more_str_stuff();
}

fn read_in_some_file(filename: &str) -> Result<String, io::Error> {
    // this can be called like
    // let file = env::args().nth(1).expect("please supply a filename"); // Get command line arg
    // let text = read_in_some_file(&file).expect("bad file man!"); // expect unwraps result and prints if panic

    // Result is used to return two things either OK or Err a pattern below is used to do this
    // match is used as a error handling pattern
    let mut file = match File::open(&filename) {
        // Note that the file is closed at the end of the function automatically when the file var is dropped
        Ok(f) => f, // if we can get the file we return it to the mut file we just created
        Err(e) => return Err(e), // If it's Err it returns the error, rewrapped as an Err.
    };
    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Ok(_) => Ok(text), // the Ok is unimportant, so we ignore it with _. This line will return if hit
        Err(e) => return Err(e),
    }

    // This whole thing can be a little more cleanly written as:
    // fn read_to_string(filename: &str) -> io::Result<String> { // where we use io::Result<String>
    //     let mut file = File::open(&filename)?; // ? operator does almost exactly what the match on File::open does;
    //                                            // if the result was an error, then it will immediately return that error
    //     let mut text = String::new();
    //     file.read_to_string(&mut text)?;
    //     Ok(text) // still have to wrap it at the end
    // }
}

fn more_str_stuff() {
    let text = "the red fox and the lazy dog";
    let words: Vec<&str> = text.split_whitespace().collect();
    // functionally equivalent to:
    // let mut words = Vec::new();
    // words.extend(text.split_whitespace());
    for s in words {
        print!("|{}", s);
    }
}

fn string_fun() {
    let mut s = String::new();
    // initially empty!
    s.push('H');
    s.push_str("ello");
    s.push(' ');
    s += "World!"; // short for `push_str`
                   // remove the last char
    s.pop();

    println!("{}", s);
    assert_eq!(s, "Hello World");
}

fn abs(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        -x
    }
}

fn factorial(n: u64) -> u64 {
    if n == 0 {
        1 // do not need a return for this
    } else {
        n * factorial(n - 1)
    }
}

fn count_array(arr: &[i32]) -> i32 {
    let mut res = 0;
    for i in 0..arr.len() {
        res += arr[i]
    }
    res
}

fn sum_to(n: i32) -> i32 {
    (1..n).sum()
}

fn window_view(arr: &[i32]) {
    // Windows are slices of arrays at a given # of items
    for s in arr.windows(2) {
        println!("{:?}", s)
    }
}
