use std::io;

fn main(){
    println!("Guess the number");

    println!("Please input your guess");

    /*
    let apples = 5; <- 不変
    let mut bananas = 5; <- 可変
    */
    let mut guess = String::new();

    /*
    ioライブラリをインポートしなくても
    std::io::stdinのように呼び出すことによって利用可
    */
    io::stdin()
        .read_line(&mut guess)  // 標準入力ハンドルのread_lineメソッドを呼び出し
        .expect("Failed to read line"); // 例外処理

    println!("You guessed: {}", guess);
}