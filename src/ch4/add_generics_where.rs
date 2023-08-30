fn add<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    a + b
}

fn x2<T>(n: T) -> T
where
    T: std::ops::Add<Output = T> + Copy,
{
    n + n
}

fn main() {
    println!("{}", add(10, 25));
    println!("{}", add(10.0, 25.0));
    println!("{}", x2(10));
    println!("{}", x2(13.3));
}
