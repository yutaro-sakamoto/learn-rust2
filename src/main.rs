use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::LinkedList;

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

    if let Some(n) = maybe_fail() {
        println!("{n}");
    }

    while let Some(n) = maybe_fail() {
        println!("{n}");
        break;
    }

    fn average(v: &[f32]) -> Option<f32> {
        if v.is_empty() {
            return None;
        }
        let mut total = 0.0;
        for n in v {
            total += n;
        }
        Some(total / v.len() as f32)
    }

    let n: f64 = 1.0;
    assert!(n >= 0.0);
    assert!(true);
    let a = 1;
    let b = 1;
    let c = 2;
    assert_eq!(a, b);
    assert_ne!(a, c);

    let n = 56;
    println!("{}", n);
    println!("{:>04}", n);
    println!("{:#x}", n);
    println!("{:#016x}", n);
    println!("{:#o}", n);
    println!("{:#b}", n);

    let v = vec![true, false, false];

    let f = |a, b| a + b;
    let n = f(10, 20);

    let mut s = Storage::SSD(512);
    let mut f = || match &mut s {
        Storage::HDD { size: s, .. } => *s += 64,
        _ => (),
    };

    struct Env_f<'a> {
        storage: &'a mut Storage,
    }

    struct closure_f<'a> {
        ptr_func: fn(),
        ptr_env: Box<Env_f<'a>>,
    }

    let mut s = Storage::SSD(512);
    let mut g = move || match &mut s {
        Storage::HDD { size: s, .. } => *s += 64,
        _ => (),
    };
    g();

    fn get_size(s: &Storage) -> u32 {
        match s {
            Storage::HDD { size: s, .. } => *s,
            Storage::SSD(s) => *s,
        }
    }

    impl Storage {
        fn get_size(&self) -> u32 {
            match self {
                Storage::HDD { size: s, .. } => *s,
                Storage::SSD(s) => *s,
            }
        }

        fn set_size(&mut self, size: u32) {
            match self {
                Storage::HDD { size: s, .. } => *s = size,
                Storage::SSD(s) => *s = size,
            }
        }
    }

    impl PCSpec {
        fn new(cpus: u16, memory: u32, storage: Storage) -> PCSpec {
            PCSpec {
                cpus,
                memory,
                storage,
            }
        }
    }
    let s = Storage::SSD(512);
    let spec = PCSpec::new(8, 32, s);

    let mut list1 = LinkedList::new();
    list1.push_back(0);
    list1.push_back(1);
    list1.push_back(2);

    let mut list2 = LinkedList::new();
    list2.push_back(100);
    list2.push_back(200);
    list2.push_back(300);

    list1.append(&mut list2);

    list1.push_front(-10);

    let mut m = BTreeMap::new();

    m.insert(1, "apple");
    m.insert(1, "apple");
    m.insert(2, "orange");
    m.insert(3, "banana");

    if let Some(old) = m.remove(&2) {
        println!("{}", old);
    }

    if let Some(value) = m.get(&3) {
        println!("{}", value);
    }

    let mut s = BTreeSet::new();
    s.insert(100);
    s.insert(400);
    s.insert(6);
    s.insert(1);

    for n in s.iter() {
        println!("{n}");
    }

    let mut v = Vec::new();
    v.push(10);
    v.push(5);

    let mut s = BTreeSet::new();
    s.insert(100);
    s.insert(400);

    let it = v.iter().chain(s.iter());
    for n in it.clone().map(|n| n * 2) {
        println!("{n}");
    }

    let total = it.clone().fold(0, |acc, x| acc + x);

    let list: LinkedList<_> = it.filter(|n| *n % 2 == 0).collect();

    for (n, m) in v.iter().zip(s.iter()) {
        println!("({n}, {m}");
    }
}

//fn example06() {
//}
