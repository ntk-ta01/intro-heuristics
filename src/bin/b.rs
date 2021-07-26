use proconio::input;

fn main() {
    input! {
        d: usize,
        c: [i32; 26],
        s: [[i32; 26]; d],
        t: [i32; d]
    }
    let input = Input { d, c, s };
    let out; //= vec![0; input.d];
    out = t;
    let _score = compute_score(&out, &input);
    // eprintln!("{}", score);
}

fn compute_score(out: &[i32], input: &Input) -> i32 {
    let mut score = 0;
    let mut last = vec![0; 26];
    #[allow(clippy::clippy::needless_range_loop)]
    for d in 0..input.d {
        let type_i = (out[d] - 1) as usize;
        last[type_i] = d + 1;
        score += input.s[d][type_i];
        #[allow(clippy::clippy::needless_range_loop)]
        for i in 0..26 {
            score -= input.c[i] * (d + 1 - last[i]) as i32;
        }
        println!("{}", score);
    }
    score
}

#[derive(Debug)]
struct Input {
    d: usize,
    c: Vec<i32>,
    s: Vec<Vec<i32>>,
}
