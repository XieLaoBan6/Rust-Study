/*
//////////////所有权规则//////////////
1、Rust中的每一个值都有一个所有者
2、在任意给定时间，只能有一个所有者
3、当所有者超出作用域时，该值将被丢弃

//////////////作用域//////////////
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid

//////////////String类型//////////////
*/
// use std::io;
// use std::cmp::Ordering;

fn main() 
{
    // copy_function();
    // string_function1();
    // string_function2();
    // move_function();
    // clone_function();
    // example_function1();
    // example_function2();
    example_function3();
}

fn copy_function()
{
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
}

fn string_function1() 
{
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{s}");
}

fn string_function2() 
{
    let mut s = String::from("hello");
    {
        let mut s = String::from("hello");

        s.push_str(", world!");
    }

    println!("{s}");
}

fn move_function()
{
    let s1 = String::from("hello");

    let s2 = s1;

    // println!{"{s1}"};    //显示失败，s1已被释放
    println!("{s2}");

    let mut s3 = String::from("hello");

    s3 = String::from("ahoy");      //drop原先的数据，覆盖

    println!("{s3}, world!");
}

fn clone_function() 
{
    let s1 = String::from("hello");

    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
}

fn example_function1()
{
    let s = String::from("hello");

    take_ownership(s);

    // println!("{s}");     //s已经在take_ownership中被drop

    let x = 5;

    take_copy(x);

    println!("{}", x);
}

fn take_ownership(some_string: String)
{
    println!("{some_string}");
}

fn take_copy(some_integer: i32)
{
    println!("{some_integer}");
}

fn example_function2()
{
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    println!("s1 = {}", s1);

    // println!("s2 = {}", s2);     s2已经在takes_and_gives_back中被drop

    println!("s3 = {}", s3);
}

fn gives_ownership() ->String 
{
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) ->String
{
    a_string
}

fn example_function3()
{
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    // println!("s1 = {}", s1);     s1已经在calculate_length中被drop

    println!("s2 = {}, len = {}",s2, len);
}

fn calculate_length(s: String) -> (String, usize)
{
    let length = s.len();

    (s, length)
}