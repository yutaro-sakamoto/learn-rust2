use serde::{Deserialize, Serialize};
use std::any::{Any, TypeId};
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::LinkedList;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::iter::Iterator;
use std::ops::Mul;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{fs::File, io::prelude::*, path::Path};
use std::{sync::RwLock, thread::sleep, time::Duration};

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

    fn worker() -> u32 {
        println!("worker");
        100
    }

    let handler = std::thread::spawn(worker);

    match handler.join() {
        Ok(n) => println!("{n}"),
        Err(e) => println!("{:?}", e),
    }

    let (tx, rx) = std::sync::mpsc::sync_channel(64);

    let handler = std::thread::spawn(move || match rx.recv() {
        Ok((x, y)) => println!("({}, {})", x, y),
        Err(e) => eprintln!("{e}"),
    });

    if let Err(e) = tx.send((10, 20)) {
        eprintln!("{e}");
    }

    if let Err(e) = handler.join() {
        eprintln!("{:?}", e);
    }

    struct XOR64 {
        x: u64,
    }

    impl XOR64 {
        fn new(seed: u64) -> XOR64 {
            XOR64 {
                x: seed ^ 8817264543325252,
            }
        }

        fn next(&mut self) -> u64 {
            let x = self.x;
            let x = x ^ (x << 13);
            let x = x ^ (x >> 7);
            let x = x ^ (x << 17);
            self.x = x;
            return x;
        }
    }

    const NUM: usize = 200000000;

    fn randomized_vec() -> (Vec<u64>, Vec<u64>) {
        let mut v1 = Vec::new();
        let mut v2 = Vec::new();
        let mut generator = XOR64::new(1234);

        for _ in 0..NUM {
            let x1 = generator.next();
            let x2 = generator.next();
            v1.push(x1);
            v2.push(x2);
        }

        (v1, v2)
    }

    fn single_threaded() {
        let (mut v1, mut v2) = randomized_vec();
        let start = std::time::Instant::now();

        v1.sort();
        v2.sort();

        let end = start.elapsed();
        println!(
            "single_threaded: {}.{:03}s",
            end.as_secs(),
            end.subsec_nanos() / 1_000_000,
        );
    }

    fn multi_threaded() {
        let (mut v1, mut v2) = randomized_vec();
        let start = std::time::Instant::now();
        let handler1 = std::thread::spawn(move || {
            v1.sort();
            v1
        });
        let handler2 = std::thread::spawn(move || {
            v2.sort();
            v2
        });
        let _v1 = handler1.join().unwrap();
        let _v2 = handler2.join().unwrap();
        let end = start.elapsed();
        println!(
            "multi_threaded: {}.{:03}s",
            end.as_secs(),
            end.subsec_nanos(),
        );
    }

    //single_threaded();
    //multi_threaded();

    struct H2O {}
    struct O2 {}
    struct H2 {}

    fn burn(_h2_1: H2, _h2_2: H2, _o2: O2) -> (H2O, H2O) {
        (H2O {}, H2O {})
    }

    let h2_1 = H2 {};
    let h2_2 = H2 {};
    let o2 = O2 {};
    let (h2o_1, h2o_2) = burn(h2_1, h2_2, o2);

    struct Coin {}

    let a = Coin {};
    let b = a;
    let c = b;

    let a = 10;
    let b = 20;
    let c = a + b;
    let d = a * b;

    let a;
    {
        let b = 10;
        a = &b;
        println!("{}", a);
    }

    let a: i32 = 10;
    let b: &i32 = &b;

    fn square<'a>(x: &'a i32) -> i32 {
        x * x
    }

    square(b);

    struct Foo<'a> {
        x: &'a i32,
    }

    Foo { x: &a };

    fn add<'a>(x: &'a mut i32, y: &'a i32) {
        *x += *y;
    }

    let mut x = 10;
    {
        let y = 20;
        add(&mut x, &y);
    }
    println!("{x}");

    let a = 10;
    {
        let b = &a;
        let c = &a;
        let d = b;
    }

    fn bar(x: &mut i32) {
        *x += 1;
    }

    let mut a = 10;
    let b = &mut a;
    bar(b);
    *b += 10;

    struct XY {
        x: Vec<i32>,
        y: Vec<i32>,
    }

    let mut xy = XY {
        x: vec![1, 2, 3],
        y: Vec::new(),
    };

    //for elm in xy.x.iter() {
    //    y.push(*elem * *elm);
    //}
    let XY { x, y } = &mut xy;
    for elm in x {
        y.push(*elm * *elm);
    }

    let v = Arc::new(vec![1, 2, 3]);
    let w = v.clone();
    let z = v.clone();

    let x = Arc::new(Mutex::new(100_000));
    let x2 = x.clone();

    let h1 = std::thread::spawn(move || {
        let mut guard = x.lock().unwrap();
        *guard -= 30_000;
    });

    let h2 = std::thread::spawn(move || {
        let mut guard = x2.lock().unwrap();
        *guard -= 30_000;
    });

    h1.join().unwrap();
    h2.join().unwrap();

    //run_rw_lock_example();

    struct ImaginaryNumber {
        real: f64,
        img: f64,
    }

    impl Display for ImaginaryNumber {
        fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
            write!(f, "{} + {}i", self.real, self.img)
        }
    }

    let n = ImaginaryNumber {
        real: 3.0,
        img: 4.0,
    };
    println!("{n}");

    #[derive(Debug, Clone, Serialize, Deserialize)]
    enum MyList<T> {
        Node { data: T, next: Box<MyList<T>> },
        Nil,
    }

    impl<T> MyList<T> {
        fn new() -> MyList<T> {
            MyList::Nil
        }

        fn cons(self, data: T) -> MyList<T> {
            MyList::Node {
                data,
                next: Box::new(self),
            }
        }

        fn iter<'a>(&'a self) -> ListIter<'a, T> {
            ListIter { elm: self }
        }
    }

    struct ListIter<'a, T> {
        elm: &'a MyList<T>,
    }

    impl<'a, T> Iterator for ListIter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            match self.elm {
                MyList::Node { data, next } => {
                    self.elm = next;
                    Some(data)
                }
                MyList::Nil => None,
            }
        }
    }
    let list = MyList::new().cons(1).cons(2).cons(3);

    let js = serde_json::to_string(&list).unwrap();
    println!("JSON* {} bytes", js.len());
    println!("{js}");

    let yml = serde_yaml::to_string(&list).unwrap();
    println!("YAML: {} bytes", yml.len());
    println!("{yml}");

    let msgpack = rmp_serde::to_vec(&list).unwrap();
    println!("MessagePack: {} bytes", msgpack.len());

    let list = serde_json::from_str::<MyList<i32>>(&js).unwrap();
    println!("{:?}", list);

    //let list = serde_yaml::from_str::<MyList<i32>>(&js).unwrap();
    //println!("{:?}", list);

    let list = rmp_serde::from_slice::<MyList<i32>>(&msgpack).unwrap();
    println!("{:?}", list);

    let yml = serde_yaml::to_string(&list).unwrap();

    let path = Path::new("test.yml");
    let mut f = File::create(path).unwrap();
    f.write_all(yml.as_bytes()).unwrap();

    let path = Path::new("test.yml");
    let mut f = File::open(path).unwrap();
    let mut yml = String::new();
    f.read_to_string(&mut yml).unwrap();

    let list = serde_yaml::from_str::<MyList<i32>>(&yml).unwrap();
    println!("{:?}", list);

    fn square2<T>(x: T) -> T
    where
        T: Mul<Output = T> + Copy,
    {
        x * x
    }

    let n = Arc::new(10);
    thread::spawn(move || {
        println!("{n}");
    });

    //let n = Arc::new(Mutex::new(Rc::new(10)));
    //thread::spawn(move || {
    //    let n = n.lock().unwrap();
    //    println!("{n}");
    //});

    trait Foo2 {
        fn foo(&self);
    }

    struct Bar;
    impl Foo2 for Bar {
        fn foo(&self) {
            println!("Bar::foo");
        }
    }

    struct Buzz;
    impl Foo2 for Buzz {
        fn foo(&self) {
            println!("Buzz::foo");
        }
    }

    fn call_foo_static<T: Foo2>(arg: &T) {
        arg.foo();
    }

    fn call_foo_dynamic(arg: &dyn Foo2) {
        arg.foo();
    }

    let bar = Bar;
    let buzz = Buzz;

    call_foo_static(&bar);
    call_foo_static(&buzz);

    call_foo_dynamic(&bar);
    call_foo_dynamic(&buzz);

    #[derive(Debug)]
    struct ErrorA;

    impl Display for ErrorA {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ErrorA")
        }
    }

    impl Error for ErrorA {}

    #[derive(Debug)]
    struct ErrorB;
    impl Error for ErrorB {}

    impl Display for ErrorB {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Error B")
        }
    }

    fn error_a() -> Result<(), ErrorA> {
        Err(ErrorA)
    }

    fn error_b() -> Result<(), ErrorB> {
        Err(ErrorB)
    }

    fn error_ab() -> Result<(), Box<dyn std::error::Error>> {
        error_a()?;
        //error_b()?;
        Ok(())
    }

    trait Location {
        fn address(&self) -> &str;
    }

    trait Person {
        fn name(&self) -> &str;
    }

    trait House: Location + Person {}

    fn print_house_info(house: &dyn House) {
        println!("所有者: {}", house.name());
        println!("住所: {}", house.address())
    }

    struct MyHouse {
        owner: String,
        address: String,
    }

    impl Location for MyHouse {
        fn address(&self) -> &str {
            &self.address
        }
    }

    impl Person for MyHouse {
        fn name(&self) -> &str {
            &self.owner
        }
    }

    impl House for MyHouse {}

    let my_house = MyHouse {
        owner: "かぐや姫".to_string(),
        address: "ムーンベース 3丁目1番地".to_string(),
    };

    print_house_info(&my_house);

    fn closure() -> impl Fn(i32) -> i32 {
        |x| x * x
    }

    let f = closure();
    f(10);

    let f = |x: i32| x * x;
    let g = |x: i32| x * x;

    fn get_type_id<T: Any>(_: &T) -> TypeId {
        TypeId::of::<T>()
    }

    println!("{}", get_type_id(&f) == get_type_id(&g));

    //fn error_ab_impl(flag: bool) -> impl std::error::Error {
    //    if flag { ErrorA } else { ErrorB }
    //}
}

mod a {
    struct TypeA;
    mod a_1 {
        struct TypeA1;
    }

    mod a_2 {
        struct TypeA2;
    }
}

mod b {
    struct TypeB;
    mod b_1 {
        struct TypeB1;
    }

    mod b_2 {
        struct TypeB2;
    }
}

fn run_rw_lock_example() {
    let mut gallery = BTreeMap::new();
    gallery.insert("葛飾北斎", "富嶽三十六景 神奈川沖浪裏");
    gallery.insert("ミュシャ", "黄道十二宮");

    let gallery = Arc::new(RwLock::new(gallery));

    let mut hdls = Vec::new();
    for n in 0..3 {
        let gallery = gallery.clone();
        let hdl = std::thread::spawn(move || {
            for _ in 0..8 {
                {
                    let guard = gallery.read().unwrap();
                    if n == 0 {
                        for (key, value) in guard.iter() {
                            print!("{key}:{value}, ");
                        }
                        println!();
                    }
                }
                sleep(Duration::from_secs(1));
            }
        });
        hdls.push(hdl);
    }

    let staff = std::thread::spawn(move || {
        for n in 0..4 {
            if n % 2 == 0 {
                let mut guard = gallery.write().unwrap();
                guard.clear();
                guard.insert("ゴッホ", "星月夜");
                guard.insert("エッシャー", "滝");
            } else {
                let mut guard = gallery.write().unwrap();
                guard.clear();
                guard.insert("葛飾北斎", "富嶽三十六景 神奈川沖浪裏");
                guard.insert("ミュシャ", "黄道十二宮");
            }
            sleep(Duration::from_secs(2));
        }
    });

    for hdl in hdls {
        hdl.join().unwrap();
    }
    staff.join();
}

//fn example06() {
//}
