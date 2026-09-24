use std::cmp::Ordering;
use std::io;

/*
isize: i8,i16,i32,i64,128
usize:u8,u16,u32,u64,u128
浮点：f32,f64
十进制：98_222
十六进制：0xff
八进制：0o77
二进制：0b1111_0000
字节：b'A'
*/

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x= 6;
    println!("The value of x is: {x}");

    const TIME_VALUE: u32 = 60 * 60 * 3;

    println!("This value is {TIME_VALUE}");

    let y = 8;
    let y = y + 2;
    {
        let y = y * 3;
        println!("This Value of Y in the inner scope is: {y}");
    }

    println!("This Value Of Y is: {y}");

    let spaces = "   ";
    let spaces = spaces.len();

    println!("The number of spaces is: {spaces}");

    // let guess = String::new();

    let guess: u32 = "42".parse().expect("Not a number");
    // let guess: String = "42".parse().expect("Not a number");
    // {
    //     Ok(num) => num,
    //     Err(_) => continue,
    // };

    println!("The Number of guess is: {guess}");

    //整型
    let number = 1_999;

    println!("The number is: {number}");
    
    //浮点
    let b = 3.0;
    let c: f32 = 8.0;

    println!("A:{b}, B: {c}");

    //加减乘除
    let sum = 5 + 4;
    println!("sum: {sum}");
    let sub = 98.8 - 33.4;
    println!("sub: {sub}");
    let mul = 30 * 2;
    println!("mul: {mul}");
    let quotient = 77.7/11.1;
    println!("quotient: {quotient}");
    let truncated = -3/2;
    println!("truncated: {truncated}");
    let remainder = 88%3;
    println!("remainder: {remainder}");

    //Bool型
    let Bool = true;
    let f: bool = false;

    println!("Bool: {Bool}, f: {f}");

    //字符型
    let char = 'z';
    let n: char = 'Z';
    let cat = '🐱';

    println!("char: {char}, n: {n}, cat: {cat}");

    //元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // println!("tup.1: {tup.1}, tup.2:{tup.2}, tup.3:{tup.3}");  //生成失败

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("y: {y}");

    let x:(i32,f64,u8) = (500,6.4,1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("x.0: {five_hundred}, x.1:{six_point_four}, x.2:{one}");

    //数组(分配在栈上)
    let x = [1,2,3,4,5];
    let months = ["January", "Febtuary", "March"];
    let y: [i32; 5] = [1,2,3,4,5];
    let z = [3; 5]; //等同于 z = [3,3,3,3,3];

    let first = x[0];
    let second = x[1];

    //println!("x[0]: {x[0]}, x[4]: {x[4]}");//生成失败
    //println!("first: {x[0]}"); //生成失败
    println!("x[0]: {first}, x[1]: {second}");

    //输入一个值后输出数组对应数据
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number!");
    
    let element = x[index];

    println!("The value of the element st index {index} is: {element}");

    first_function();
}

fn first_function() {
    println!("This is the first function!");
}