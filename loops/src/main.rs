fn main() {
    first_function();
    second_function();

    let a = third_function();

    println!("Third Loop Result: {a}");

    forth_function();

    fifth_function();

    sixth_function();

    seventh_function();

    eighth_function();
}

fn first_function() {
    loop{
        println!("loop");
        break;
    }
}

fn second_function() {
    let mut counter = 0;
    let result = loop{
        counter +=1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Loop Result: {result}");
}

fn third_function() -> u32{
    let mut counter = 0;
    let result = loop{
        counter +=1;

        if counter == 10 {
            break counter * 2;
        }
    };
    
    println!("Third_Function Loop Result: {result}");
    return result; //（return a;） 与（a)等同
    // result
}

fn forth_function(){
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    println!("End Count = {count}");
}

fn fifth_function() {
    let mut number = 3;

    while number != 0
    {
        println!("{number}");

        number -= 1;
    }

    println!("While Off");
}

fn sixth_function() {
    let x = [1,2,3,4,5];
    let mut index = 0;

    while index < 5 {
        println!("The value is :{}", x[index]);

        index += 1;
    }
}

fn seventh_function() {
    let x = [1,2,3,4,5];

    for element in x {
        println!("value : {element}");
    }
}

fn eighth_function() {
    for number in (1..4).rev() {
        println!("{number}");
    }

    println!("For off");
}