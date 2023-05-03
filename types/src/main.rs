fn main() {
    let x = 2.0;    // f64
    let y: f32 = 3.0;   // f32

    let t = true;
    let f: bool = false;    // 明示的型注釈付きで

    // char型はシングルクォート
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻'; 
    println!("{}, {}, {}", c, z, heart_eyed_cat);
    // println!(c, z, heart_eyed_cat); <- エラー

    // タプルの位置ごとに型があり、全てが同じ型である必要が無い
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tupe = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    /*
    配列
    タプルとは異なり、配列の全要素は同じ型でなければならない
    Rustの配列は固定長
    */
    // ベクタ型より配列を使いたくなる例
    let months = ["January", "February", "March", "April", "May", "June", "July",
                    "August", "September", "October", "November", "December"];

    // 角カッコの中に初期値とセミコロン、配列の長さを与えることによって、
    // 各要素に同じ値を持つように配列を初期化することができる
    let three = [3; 5]; // let three = [3,3,3,3,3];と同義

    

}
