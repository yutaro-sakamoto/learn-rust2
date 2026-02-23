fn main() {
    let x: i32 = 10;
    let y = 20;
    let z = mul(x, y);
    println!("z = {z}");

    example01();
    example02();
    example03();
    example04();
    example05();
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

fn example05() {
    enum DoW {
        Sunday,
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
    }

    enum Storage {
        HDD { size: u32, rpm: u32 },
        SSD(u32),
    }

    let hdd = Storage::HDD {
        size: 512,
        rpm: 7200,
    };
    let ssd = Storage::SSD(512);
    struct PCSpec {
        cpus: u16,
        memory: u32,
        storage: Storage,
    }

    let spec = PCSpec {
        cpus: 8,
        memory: 16,
        storage: Storage::SSD(1024),
    };

    println!("{}", spec.cpus);

    struct Dim2(u32, u32);
    let d2 = Dim2(10, 20);
    println!("{}", d2.0);

    let r = &spec;
    println!("{}", r.cpus);
    println!("{}", (*r).cpus);

    enum List<T> {
        Node { data: T, next: Box<List<T>> },
        Nil,
    }

    let n1 = List::<u32>::Nil;
    let n2 = List::<u32>::Node {
        data: 10,
        next: Box::<List<u32>>::new(n1),
    };
    let n3 = List::Node {
        data: 40,
        next: Box::new(n2),
    };

    fn make_pair<T1, T2>(a: T1, b: T2) -> (T1, T2) {
        (a, b)
    }

    make_pair::<u8, bool>(40, false);
    make_pair(10, true);

    struct Buffer<const S: usize> {
        buf: [u8; S],
    }

    let buf = Buffer::<128> { buf: [0; 128] };

    let n: i32 = 100;
    let m: i64 = n as i64;

    let s = String::from("abc");
    let s: String = "abc".into();

    struct Msg {
        msg1: &'static str,
        msg2: &'static str,
    }

    fn print_msg(msg: &Msg) {
        println!("{}{}", msg.msg1, msg.msg2);
    }

    let msg = Msg {
        msg1: "Hello, ",
        msg2: "world!",
    };
    print_msg(&msg);

    fn func(a: u32, b: u32) {
        let n: u32 = a + b;
        let m = a + b;
    }
    let n = 10;
    {
        let m = 200;
        let r = m + n;
    }
    // let p = m + n; // error

    fn maybe_fail() -> Option<u32> {
        Some(10)
    }

    let result = maybe_fail();
    let result = result.unwrap();

    fn sumup(n: u64) -> u64 {
        if n == 0 { 0 } else { n + sumup(n - 1) }
    }

    //let b = if n < 0 {
    //    println!("n is negative");
    //} else {
    //    n * n
    //}

    if n < 0 {
        println!("n is negative")
    }

    let b = if n > 0 {
        n + n
    } else if n < 0 {
        n * n
    } else {
        1
    };

    fn sumup_loop(mut n: u64) -> u64 {
        let mut total = 0;
        loop {
            if n == 0 {
                break;
            }
            total += n;
            n -= 1;
        }
        total
    }

    fn sumup_while(mut n: u64) -> u64 {
        let mut total = 0;
        while n > 0 {
            total += n;
            n -= 1;
        }
        total
    }

    fn sumup_for(n: u64) -> u64 {
        let mut total = 0;
        for x in 0..=n {
            total += x;
        }
        total
    }

    'main_loop: loop {
        loop {
            break 'main_loop;
        }
    }

    let v = [3, 8, 11, 15];
    let mut result = 0;
    for x in v.iter() {
        if *x % 2 == 0 {
            continue;
        }
        result += x;
    }

    match maybe_fail() {
        Some(n) => println!("{n}"),
        None => (),
    }
}

//fn example06() {
//}
