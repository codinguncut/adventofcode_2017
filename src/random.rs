
#[allow(dead_code)]
fn random() {
    let mut guess = String::new();
    let v = vec![1, 2, 3]; // Vec<i32>

    enum Abc {
        Int(i32),
        Float(f64),
        Text(String)
    }

    fn asdf(a: i32, b: i32) -> i32 {
        return a + b;
    }

    // std::collections.HashMap
    
    fn fact(n: i32) -> i32 {
        match n {
            1 => 1,
            t => t * fact(t-1)
        }
    }

    let x = 5;
    let y = match x {
        t @ 5 => 5,
        3 => 3,
        _ => 1
    };

    struct Point {
        x: i32,
        y: i32
    }
    let v = Point {x:1, y:2};
    let s = v.x + v.y;

    type Pair<a,b> = (a,b);
    type Inch = i32;

    fn id<t>(x : t) -> t { x }

    enum Option<T> {
       None,
       Some(T)
    }

    impl<T> Option<T> {
        fn hello(&self) -> i32 {
            5
        }
    }
    const ASDF: i32 = 10;

    /*
    fn ff(f:(int,int -> int), x:int) -> int {
        f (x, x)
    }
    */
    (0..).filter(|n| n % 2 == 0).map(|n| n + 1);
    // fold

    // trait
    // macro_rules
    
    
    /*
       Iterators
       map, filter, collect, find
       vec.iter()
       iterators are lazy, so be careful re: side effects

       chain, cycle
       iter_mut
       into_iter (turns x into iter (ownership))
       itertools crate
    */
    let one_to_one_hundred = (1..101).collect::<Vec<_>>();
    let greater_than_forty_two = (0..100)
                                 .find(|x| *x > 42);

}

