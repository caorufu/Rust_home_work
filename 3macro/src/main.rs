macro_rules! myprint {
    ($x: expr) => {
        println!("{}", $x)
    };
    ($x: expr, $($y: expr),+) => {
        let mut vec = Vec::new();
        if $x % 2 != 0 {
            println!("{}", $x);
        } else {
            vec.push($x);
        }
        myprint!($($y),+);
        for v in vec.iter() {
            println!("{}", v);
        }
    };
}

fn main() {
    // we want: 1, 3, 5, 6, 4, 2
    myprint!(1, 2, 3, 4, 5, 6);
    // want: 1, 3, 5, 4, 2
    myprint!(1, 2, 3, 4, 5);
}
