use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("---猜数字游戏---");

    //生成一个随机数字
    let secret_number=rand::thread_rng().gen_range(1..101);

    loop {
        println!("请输入你猜的数字: ");
        //生成一个String类型的字符串
        let mut guess=String::new();
        io::stdin().read_line(&mut guess).expect("输入错误\n");

        //shadow隐藏
        let guess:u32=match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你输入的数字是: {}",guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small.\n"),
            Ordering::Greater => println!("Too big.\n"),
            Ordering::Equal => break,   //游戏结束
        }
    }

}
