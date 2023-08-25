use std::fs::File;
use std::io::Write;

fn main() {
    let filename = "fizzbuzz_file_result.txt";
    let data = get_fizz_buzz(100);
    let mut fp = File::create(filename).unwrap();
    let bytes = data.as_bytes();
    fp.write_all(bytes).unwrap();
}

fn get_fizz_buzz(max: u32) -> String {
    let mut result = String::new();

    // FizzBuzzを100まで求める
    for i in 1..=max {
        if i % 3 == 0 && i % 5 == 0 {
            result += "FizzBuzz\n";
        } else if i % 3 == 0 {
            result += "Fizz\n";
        } else if i % 5 == 0 {
            result += "Buzz\n";
        } else {
            result += &format!("{}\n", i);
        }
    }

    result
}
