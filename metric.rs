use std::io;

fn main() {
    println!("tp: ");
    let tp = read_line().parse::<f64>().unwrap();

    println!("tn: ");
    let tn = read_line().parse::<f64>().unwrap();

    println!("fp: ");
    let fp = read_line().parse::<f64>().unwrap();

    println!("fn: ");
    let fn_ = read_line().parse::<f64>().unwrap();

    println!("\n");

    println!("accuracy: {}", accuracy(tp, tn, fp, fn_));
    println!("precision: {}", precision(tp, fp));
    println!("recall: {}", recall(tp, fn_));
    println!("specificity: {}", specificity(tn, fp));
    println!("f1_score: {}", f1_score(tp, fp, fn_));
    println!("\n");
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn accuracy(tp: f64, tn: f64, fp: f64, fn_: f64) -> f64 {
    (tp + tn) / (tp + tn + fp + fn_)
}

fn precision(tp: f64, fp: f64) -> f64 {
    tp / (tp + fp)
}

fn recall(tp: f64, fn_: f64) -> f64 {
    tp / (tp + fn_)
}

fn specificity(tn: f64, fp: f64) -> f64 {
    tn / (tn + fp)
}

fn f1_score(tp: f64, fp: f64, fn_: f64) -> f64 {
    2.0 * precision(tp, fp) * recall(tp, fn_) / (precision(tp, fp) + recall(tp, fn_))
}
