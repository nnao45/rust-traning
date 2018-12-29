fn main() {
    let mut sieve = [true; 10000]; // "true"が10000個のlist
    for i in 2..100 {              // iが2から99まで
        if sieve[i] {              // i番目のsieveがtrueなら
            let mut j = i * i;     // jはiの2乗と定義
            while j < 10000 {      // jが10000より小さければ
                sieve[j] = false;  // j番目のsieveはfalse
                j += i;            // j = j+i
            }
        }
    }

    assert!(sieve[211]);
    println!("{}", sieve[211]);
    assert!(!sieve[9876]);
    println!("{}", !sieve[9876]);
}
