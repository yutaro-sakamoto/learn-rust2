fn main() {
    let x: i32 = 10;
    let y = 20;
    let z = mul(x, y);
    println!("z = {z}");

    example01();
    example02();
    example03();
    example04();
}

fn mul(x: i32, y: i32) -> i32 {
    x * y
}

fn example01() {
    let mut n: u64 = 100;
    let a: &u64 = &n;
    println!("*a = {}, addr = {:p}", *a, a);
}

fn example02() {
    let arr: [u32; 4] = [1, 2, 3, 4];
    println!("{}, {}, {}, {}", arr[0], arr[1], arr[2], arr[3]);

    let s: &[u32] = &arr[1..3];
    println!("{:?}", s);
}

fn example03() {
    let a: &str = "  Hello";
    let mut b: String = a.to_string();
    b += ", world!  ";
    let c: &str = b.trim();
    println!("{c}");
}

fn do_it(f: fn(u32, u32) -> u32, a: u32, b: u32) {
    println!("{}", f(a, b));
}

fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn mul2(a: u32, b: u32) -> u32 {
    a * b
}

fn example04() {
    do_it(add, 10, 2);
    do_it(mul2, 10, 2);
}
