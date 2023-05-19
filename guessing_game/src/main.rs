//use std::fmt::Error;
use std::io; //prelude
use std::cmp::Ordering; // Odering 是 enum, Less Greater Equal 是該enum的變體(variant)
//use std::error::Error;
use rand::Rng; // trait

fn main() {
    println!("請猜測一個數字！");

    let secret_number = rand::thread_rng().gen_range(1..=100); //這個範圍會包含下限和上限，所以我們需要指定 1..=100 來索取 1 到 100 之間的數字。
    //let secret_number = rand::thread_rng().gen_range(1, 101); // -->error Only gen_range(low..high) and gen_range(low..=high) are supported.
    println!("祕密數字為：{secret_number}");

    loop {
        println!("請輸入你的猜測數字。");

        let mut guess = String::new();
        //guess = "fuck you".to_string();
        //guess.push_str("fuck you ");

        io::stdin()
            .read_line(&mut guess)
            .expect("讀取該行失敗");

        // shadow 隱藏原來同名的舊變量，從這行開始以後遇到guess都是指這個新的guess，這特性通常用於需要類型轉換
        //let guess: u32 = guess.trim().parse().expect("please type a number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num2) => num2, //guess = num2
            Err(_) => continue, // "_" 代表不關心錯誤訊息，continue代表跳出這次loop並進行下一次循環
        };

        println!("你的猜測數字：{guess}");

        match guess.cmp(&secret_number){
            Ordering::Less => println!("too small"),    //arm
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
        }
    }
    


    // let mut input = String::new();
    // match io::stdin().read_line(&mut input) {
    //     Ok(q) => {
    //         //println!("{} bytes read", n+1);
    //         println!("{q} bytes read");
    //         println!("{input}");
    //     }
    //     Err(error) => println!("error: {error}"),
    //     //Err(std::error) => println!("error: {error}"), --> compiled error: not a unit struct, unit variant, or constant
    // }

    let x: Result<u32, &str> = Err("emergency failure");
    x.expect("Testing expect"); // panics with `Testing expect: emergency failure`

}
