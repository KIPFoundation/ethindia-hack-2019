extern crate meval;

fn main() {
    // logic for mathematic operations
    let sum = "(-3) + 5.6";
    let sub = "5.9 - 5.3";
    let multi = "10.4 *6.3";
    let div = "6 / 5";
    let rem = "6 % 6" ;
    let sqt = "sqrt(16)";
    let pow = "exp(3)";
    let nlog = "ln(1)";
    let absolute = "abs(-8)";
    let round = "round(4.8584)";
    let e = "e";
    let pi = "pi";
    let sign ="signum (-0.1)";

    //passing result as a string
    let res = meval::eval_str(sum).unwrap();
    println!("result is: {}", res);
}
