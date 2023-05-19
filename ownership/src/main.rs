/*
所有権規則
・Rustの各値は、所有者と呼ばれる変数と対応している
・いかなる時も所有者は一つである
・所有者がスコープから外れたら、値は棄却される
*/

/*
fn main(){  // sはここでは有効ではない、まだ宣言されていない
    let s = "Hello";    // sはここから有効になる
    println!("{}", s);
    // sで作業する
}   // このスコープは終わり。もうsは有効では無い
*/

/*
fn main(){
    // from関数を使用して、文字列リテラルからString型を生成
    let s = String::from("hello");  // 二重コロンは、String型直下のfrom関数を特定する働きをする演算子
    println!("{}", s);
}
*/

/*
fn main(){
    let mut s = String::from("Hello");
    s.push_str(", World!"); // push_str()関数は、リテラルをStrnigに付け加える
    println!("{}", s);  // これは、'Hello World!'と出力される
}
*/

/*
文字リテラルの場合、中身はコンパイル時に判明しているため、テキストは最終的なバイナリファイルに直接ハードコードされる。
しかし、この特性はその文字列リテラルの不変性にのみ端を発する。
コンパイル時にサイズが不明だったり、プログラム実行に合わせてサイズが可変なテキスト片用に一塊のメモリをバイナリに確保しておくことは不可能。

String型では、可変かつ伸長可能なテキスト断片をサポートするために、コンパイル時には不明な量のメモリをヒープに確保して内容を保持する。
つまり
・メモリは、実行時にOSに要求される
・String型を使用し終わったら、OSにこのメモリを返還する方法が必要である

ガベージコレクタ(GC)付きの言語では、GCがこれ以上使用されないメモリを検知して片づける。
しかし、GCが無い場合、メモリがもう使用されないことを見計らって、明示的に返還するコードを呼び出さなければならない。
・忘れていた場合、メモリを無駄にする
・タイミングが早い場合、無効な変数を作ってしまう
※二回開放してもバグになる

変数がスコープを抜けるとき、Rustは特別な関数を呼ぶ。
この関数はdropと呼ばれ、Rustは閉じ波括弧で自動的にdrop関数を呼び出す。
*/

/*
両方のデータポインタが同じ場所を指しているため、両方とも同じメモリを解放しようとしている。
これは二重開放エラーといい、メモリ安全性上のバグの一つとなる。
メモリを二回開放することは、memory corruptionにつながり、セキュリティ上の脆弱性を生む可能性がある。
*/

/*
fn main(){
    let s1 = String::from("Hello");
    let s2 = s1;

    println!("{}, World!", s1);
}
*/

/*
fn main(){
    let s = String::from("Hello");  // sがスコープに入る

    takes_ownership(s); // sの値が関数にムーブされ...
                        // ...ここではもう有効ではない

    let x = 5;  // xにスコープが入る

    makes_copy(x);  // xも関数にムーブされるが、i32はCopyなので、この後にxを使っても大丈夫

} // ここでxがスコープを抜け、sもスコープを抜ける。ただし、sの値はムーブされているので、何も特別なことは起こらない。

fn takes_ownership(some_string: String){    // some_stringがスコープに入る
    println!("{}", some_string);
}   // ここでsome_stringがスコープを抜け、'drop'が呼ばれる。後ろ盾していたメモリが解放される。


fn makes_copy(some_integer: i32){   // some_integerがスコープに入る
    println!("{}", some_integer);
}   // ここでsome_integerがスコープを抜ける。何も特別なことはない。
*/

fn main(){
    let s1 = gives_ownership(); // give_ownershipは戻り値をs1にムーブする

    let s2 = String::from("Hello"); // s2がスコープに入る

    let s3 = takes_and_gives_back(s2);  // s2はtakes_and_gives_backにムーブされ、戻り値もs3にムーブされる

}   // ここで、s3はスコープを抜け、ドロップされる。s2もスコープを抜けるが、ムーブされているので何も起きない。s1もスコープを抜け、ドロップされる。

fn gives_ownership() -> String{ // gives_ownershipは、戻り値を呼び出した関数にムーブする

    let some_string = String::from("Hello");    // some_stringがスコープに入る

    some_string // some_stringが返され、呼び出し元関数にムーブされる
}

// takes_and_gives_backは、Stringを1つ受け取り、返す
fn takes_and_gives_back(a_string: String) -> String{    // a_stringがスコープされる
    a_string    // a_stringが返され、呼び出し元関数にムーブされる
}