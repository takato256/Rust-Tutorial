/*
Rustの関数と変数の命名規則はスネークケースをを使うのが慣例
スネークケースとは、全文字を小文字にし、単語区切りにアンダースコアを使うこと
*/

/*
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
*/

/*
fn main(){
    print_labeled_measurement(5, 'h')
}

fn print_labeled_measurement(value: i32, uint_label: char){
    println!("The measurement is : {}{}", value, uint_label);
}
*/

/*
Rustは、式指向言語
文・・何らかの動画をして値を返さない命令
式・・結果値に評価される
*/

/*
fn main(){
    let x = (let y = 6);    // エラー let y = 6　という文は値を返さない
}
*/


fn main(){
    let y = {
        let x = 3;
        x + 1   // 式は終端にセミコロンを付けない
    };

    println!("The value of y is: {}", y);
}