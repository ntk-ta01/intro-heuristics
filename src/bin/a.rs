use proconio::input;
use rand::Rng;

fn main() {
    let mut timer = Timer::new();
    input! {
        d: usize,
        c: [i32; 26],
        s: [[i32; 26]; d],
        s_temp: f64,
        e_temp: f64,
    }
    let input = Input { d, c, s };
    let mut out = vec![];
    greedy(&mut out, &input);
    simulated_annealing(&mut out, &input, &mut timer, s_temp, e_temp);
    // for d in out {
    //     println!("{}", d);
    // }
}

fn simulated_annealing(
    out: &mut Vec<i32>,
    input: &Input,
    timer: &mut Timer,
    s_temp: f64,
    e_temp: f64,
) {
    const TIMELIMIT: f64 = 2.15;
    // const STARTTEMP: f64 = 2e3;
    // const ENDTEMP: f64 = 6e2;
    let mut temp = s_temp;
    let mut prob;

    let mut rng = rand_pcg::Pcg64Mcg::new(202_106_140);
    let mut count = 0;
    let mut now_score = compute_score(out, input);

    let mut best_out = out.clone();
    let mut best_score = now_score;
    loop {
        if count >= 100 {
            let passed = timer.get_time() / TIMELIMIT;
            if passed >= 1.0 {
                break;
            }
            temp = s_temp.powf(1.0 - passed) * e_temp.powf(passed);
            count = 0;
        }
        let mut new_out = out.clone();
        if rng.gen_bool(0.5) {
            // update近傍
            let new_type = rng.gen_range(0, 26) + 1;
            let update_day = rng.gen_range(0, input.d);
            new_out[update_day] = new_type;
        } else {
            // swap近傍
            let swap_day1 = rng.gen_range(0, input.d);
            let swap_day2 = rng.gen_range(0, input.d);
            let out1 = new_out[swap_day1];
            let out2 = new_out[swap_day2];
            new_out[swap_day2] = out2;
            new_out[swap_day2] = out1;
        }
        let new_score = compute_score(&new_out, input);
        prob = f64::exp((new_score - now_score) as f64 / temp);
        if now_score < new_score || rng.gen_bool(prob) {
            now_score = new_score;
            *out = new_out;
        }
        if best_score < now_score {
            best_score = now_score;
            best_out = out.clone();
        }
        count += 1;
    }
    println!("{}", best_score);
    *out = best_out;
}

fn greedy(out: &mut Vec<i32>, input: &Input) {
    for d in 0..input.d {
        // d日目に行うコンテストを決める
        let mut now_score = i32::min_value();
        let mut now_type = 0;
        out.push(0);
        for i in 0..26 {
            out[d] = i + 1;
            let score = compute_score(out, input);
            if now_score < score {
                now_score = score;
                now_type = i + 1;
            }
        }
        out[d] = now_type;
    }
}

fn compute_score(out: &[i32], input: &Input) -> i32 {
    let mut score = 0;
    let mut last = vec![0; 26];
    #[allow(clippy::clippy::needless_range_loop)]
    for d in 0..out.len() {
        let type_i = (out[d] - 1) as usize;
        last[type_i] = d + 1;
        score += input.s[d][type_i];
        for i in 0..26 {
            score -= input.c[i] * (d + 1 - last[i]) as i32;
        }
    }
    score
}

#[derive(Debug)]
struct Input {
    d: usize,
    c: Vec<i32>,
    s: Vec<Vec<i32>>,
}

pub fn get_time() -> f64 {
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9
}

struct Timer {
    start_time: f64,
}

impl Timer {
    fn new() -> Timer {
        Timer {
            start_time: get_time(),
        }
    }

    fn get_time(&self) -> f64 {
        get_time() - self.start_time
    }
}
