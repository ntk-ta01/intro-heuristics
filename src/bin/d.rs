use proconio::input;
fn main() {
    input! {
        d: usize,
        _: [i32; 26],
        _: [[i32; 26]; d],
        x: f64,
    }
    println!("{}", f(x));
}

fn f(x: f64) -> f64 {
    (x - 2.0) * (x - 2.0)
}
