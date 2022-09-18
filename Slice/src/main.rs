use std::string;

fn main1() {
    let s="hello world";
    let mut str=String::from("abcdef Hello ");
    let num=second_world(&str[0..6]);
    let num=second_world(&str[..]);
    let num=second_world(&str[..str.len()]);
    let num1=second_world(num);

    println!("{}",num1);

}

fn main() {
    // let s = String::from("hello world");

    // let hello = &s[0..5];
    // let world = &s[6..11];

    //非字符串切片
    println!("非字符串切片----->");
    let arr = [1, 3, 5, 7, 9];
    let part = &arr[0..3];
    for (i,&elem) in arr.iter().enumerate() {
        println!("{}->{}", i,elem);
    }
    
}

fn first_world(string:&String)->usize
{
    //其实x就是索引值，elem就是这个字符所对应的ASCII码
    let str=string.as_bytes();
    for (x,&elem) in str.iter().enumerate()
    {
        if elem==b' '       //b' '的ASCII码为32 ，判断每个字符是否为32，如果遇到了空格，则退出 
        {
            return x;
        }
    }
    string.len()
}

fn second_world(string:&str)->&str
{
    let len=string.as_bytes();
    for (i,&elem) in len.iter().enumerate()
    {
        if elem==b' '
        {
            //到达空格，返回在这以前的单词
            return &string[..i];
        }
    }
    //没有找到
    &string[..]    //原地返回
}