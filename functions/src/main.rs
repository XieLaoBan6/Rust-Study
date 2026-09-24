fn main() {
    first_function();
    second_function(9, 'k');
    third_function();

    let a = forth_function();
    println!("function return value : {a}");

    let b = fifth_function(3);
    println!("fifth value: {b}");
}

fn first_function() {
    println!("Hello world!");
}

fn second_function(x: i32, y: char) {
    println!("The value of x is:{x},  y: {y}");
}

fn third_function() {
    let y = {
        let x = 3;
        x + 4
    };

    // println!("value x:{x}, y:{y}"); //表达x的值失败
    println!("value y:{y}");
}

fn forth_function() -> i32 {
    5
}

fn fifth_function(x: i32) -> i32 {
    x * 3
}