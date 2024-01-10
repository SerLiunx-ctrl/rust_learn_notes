use std::io; //引入标准库中的io
use rand::Rng; //
use std::cmp::Ordering;

/*
 默认情况下，Rust会将少量标准库中定义的程序项（item）引入到每个程序的作用域中。
 这些项称作 prelude，可以在标准库文档中了解到关于它的所有知识。
*/

fn main() { // rust程序入口
    
    println!("guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=10); // 生成随机数 [1, 101) , 或者使用(1..=100) => [1, 100]

    loop{ //游戏循环

        println!("please input your guess");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("filed to read line!");
    
        let guess: u32 = match guess.trim().parse() { // parse()方法返回的是Result类型，可使用match匹配
            Ok(num) => num,
            Err(_) => continue,
        };//类型的遮蔽(shadow)
    
        println!("the number you guess: {}", guess);
    
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
