fn main(){
    let s1 = String::from("Hello");

    let len = calculate_length(&s1);    // &s1をcalculate_lengthに渡す

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize{   // &Stringを受け取っている
    s.len()
}   // ここでsはスコープ外となる、しかし、参照しているものの所有権を持っているわけではないので、何も起こらない。

/*
&s1という記法によって、s1の値を参照する参照を生成することができるが、これは所有することではない。
所有していないということは、指している値は、参照がスコープを抜けてもドロップされないということである。
*/