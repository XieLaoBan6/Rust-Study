#![allow(unused)]

fn main() 
{
    // example_function1();
    // example_function2();

    example_function3();
}

fn example_function1()
{
    let mut s = String::from("hello world");

    // let word = first_word(&s);

    // println!("s = {}", s);

    // s.clear();       //word返回的是一个usize值，不影响s可变属性

    // println!("word = {}", word);

    let r_word = first_word_review2(&s);

    // s.clear();      //r_word是一个&str不可变引用，所以s无法被clear

    println!("r_word = {}", r_word);

    println!("s = {}", s);

    let my_string = String::from("hello world");

    let string_word = first_word_review2(&my_string[0..6]);

    println!("string_word = {}", string_word);

    let string_word = first_word_review2(&my_string[..]);

    println!("string_word = {}", string_word);

    let string_word = first_word_review2(&my_string);

    println!("string_word = {}", string_word);

    let diff_string = "hello world";

    let string_word = first_word_review2(&diff_string[0..6]);

    println!("string_word = {}", string_word);

    let string_word = first_word_review2(&diff_string[..]);

    println!("string_word = {}", string_word);

    let string_word = first_word_review2(diff_string);

    println!("string_word = {}", string_word);
}

fn first_word(s: &String) -> usize
{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            return i;
        }
    }

    s.len()
}

//first_word_review1可以用first_word_review2进行代替，但first_word_review2不能用first_word_review1进行
fn first_word_review1(s: &String) -> &str
{
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            return &s[..i];
        }
    }

    &s[..]
}

fn first_word_review2(s: &str) -> &str
{
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            return &s[..i];
        }
    }

    &s[..]
}

fn example_function2()
{
    let s = String::from("hello world");

    let a = &s[0..3];
    let b = &s[..3];

    let len = s.len();

    let c = &s[3..len];
    let d = &s[3..];

    let e = &s[0..len];
    let f = &s[..];

    println!("a = {}, b = {}, c = {}, d = {}, e = {}, f = {}", a, b, c, d, e, f)
}

fn example_function3()
{
    let a = [1,2,3,4,5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]); // assert_eq断言比较slice和&[2,3]是否相同，失败返回断言
}