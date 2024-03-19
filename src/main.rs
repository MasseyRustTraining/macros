macro_rules! square {
    ($x:expr) => {{
        let x = $x;
        x * x
    }};
}

macro_rules! declare_square {
    ($name:ident, $x:expr) => {
        let $name = {
            let x = $x;
            x * x
        };
    };
}

fn main() {
    println!("{}", square!(3));

    let mut x = 0;
    declare_square!(x_squared, {x += 1; x + 1});
    println!("{}", x_squared);
}
