fn main() {
    /*不変と可変*/
    // let x = 5;
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    /*定数*/
    const MAX_POINTS: u32 = 100_100;

    /*シャドーイング*/
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }
    println!("The value of y is: {}", y);
    /*
    [エラーが起こらない]
    let spaces = "  ";
    let spaces = spaces.len();
    
    [エラーが起きる]
    let mut spaces = "  ";
    spaces = spaces.len();
    */




}
