fn main() {
    first_function();
    second_function();
}

fn first_function() {
    let number = 7;

    if number < 5
    {
        println!("Is True");
    }
    else
    {
        println!("Is False");
    }

    if number != 5
    {
        println!("Real");
    }

    if number % 4 == 0
    {
        println!("Divisible By 4");
    }
    else if number % 7 == 0
    {
        println!("Divisible By 7")
    }
    else
    {
        println!("None Divisible");
    }
}

fn second_function() {
    let condition = true;
    let number = if condition {5} 
                    else {6};
    println!("Value num:{number}");
}
