/*
1、在任何给定时间，你只能拥有 一个 可变引用，或者 任意数量的不可变引用。
2、引用必须总是有效的
*/

fn main() {
    // examples_function1();
    // example_function2();
    // error_function1();
    error_function2();
}

fn examples_function1()
{
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("s1 = {}", s1);

    change_function(&mut s1);

    println!("s1 = {}", s1);
}

fn calculate_length(s: &String) -> usize
{
    s.len()
}

fn change_function(some_string: &mut String)
{
    some_string.push_str(", world");
}

fn example_function2()
{
    let mut s1 = String::from("hello");

    let s2 = &s1;

    let s3 = &s1;

    // let s4 = &mut s1;     //在s2、s3的作用域未使用前，将s1不可变改为可变赋值给s4，会编译出问题

    println!("s2 = {}, s3 = {}", s2, s3);

    let s4 = &mut s1;       //因为println输出了s2、s3后，s2、s3的作用域已经结束，所以此时再将s1改为可变是可以编译的

    println!("s4 = {}", s4);
}

fn error_function1()
{
    let mut s1 = String::from("hello");

    // let s2 = &mut s1;

    // let s3 = &mut s1;

    // println!("s2 = {}, s3 = {}", s2, s3);    //无法输出s2、s3的值

    // let s2 = &mut s1;
    // {
    //     let s3 = &mut s1;

    //     println!("s3 = {}", s3);     //无法输出s3的值
    // }

    // println!("s2 = {}", s2);

    {
        let s2 = &mut s1;
        println!("s2 = {}", s2);        //可以输出
    }

    let s3 = &mut s1;

    println!("s3 = {}", s3);
}

fn error_function2()
{
    let s = danger_function();
}

fn danger_function() ->String
{
    let s1 = String::from("hello");

    // &s1      //s1的作用域仅在danger_function函数中，在函数结束后，s1被drop，无法返回s1的引用

    s1
}