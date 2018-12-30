fn main() {
    let v: Vec<f64> = vec![0.0, 0.7, 1.0, 0.7];
    let sv: &[f64] = &v;
    println!("{:?}: {:?}", v, sv);
    let s: [f64; 4] = [0.0, -0.7, -1.0, 0.7];
    let ss: &[f64] = &s;
    println!("{:?}: {:?}", s, ss);

}
