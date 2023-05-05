/*
fn main(){
    let number = 3;
    if number < 5{
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
*/

// let文内でif式を使う
/*
fn main() {
    let condition = true;
    let number = if condition {5} else {6}; // 型が同じにならなければエラー

    println!("The value of number is: {}", number);
}
*/

fn main() {
    let mut count = 0;
    'counting_up: loop{     // ループラベルを使用することで、breakやcontinueは適用されるループを指定することができる
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

