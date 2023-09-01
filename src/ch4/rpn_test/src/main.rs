fn main() {
    let src = "2 3 4 * +".to_string();
    let ans = kengos_test_rpn_calc::eval(src).unwrap();
    println!("{}", ans);
}
