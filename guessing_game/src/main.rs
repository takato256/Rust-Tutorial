use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is: {}", secret_number);

    loop{
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

        // 型変換
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue, // 例外処理 「_」は全ての値を受け付ける
        };

        println!("You guessed: {}", guess);

        // guessとsecret_numberの値に対してcmpを呼ぶ
        match guess.cmp(&secret_number){
            /*
            Orderingもenumの1つでLess, Greater, Equalという列挙子を持っている
            */
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}