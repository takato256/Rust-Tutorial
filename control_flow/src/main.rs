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

/*
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
*/
/*
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
*/

/*
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a{
        println!("the value is: {}", element);
    }
}
*/

fn main(){
    // revメソッドで中身を逆順に取り出す
    for number in (1..4).rev(){
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}