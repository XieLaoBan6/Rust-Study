fn main() {
    // example_function4();
    // example_function5();
    // example_function6();
    example_function9();
}

enum IpAddrKind
{
    V4,
    V6,
}

fn example_function1()
{
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(ip_kind: IpAddrKind){}

fn example_function2()
{
    enum IpAddr
    {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
}

fn example_function3()
{
    enum IpAddr
    {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}

fn example_function4()
{
    enum Message 
    {
        Quit,
        Move {x: i32, y:i32},
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message 
    {
        fn call(&self) 
        {

        }
    }

    let m = Message::Write(String::from("hello"));

    m.call();
}

fn example_function5()
{
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    //let sum = x + y;      //i8 + Option<i8> 错误
}

fn example_function6()
{
    fn plus_one(x: Option<i32>) -> Option<i32>
    {
        match x         //match必须穷尽所有可能性
        {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    dbg!(five);
    dbg!(six);
    dbg!(none);
}

fn example_function7()
{
    let dice_roll = 9;
    match dice_roll
    {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),  //最后一个涵盖所有其他可能值,该分支必须放置在最后
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}

    // match dice_roll 
    // {
    //     3 => add_fancy_hat(),
    //     7 => remove_fancy_hat(),
    //     _ => reroll(),      //最后一个分支忽略了所有其他值
    // }

    // fn add_fancy_hat() {}
    // fn remove_fancy_hat() {}
    // fn reroll() {}

    // match dice_roll {
    //     3 => add_fancy_hat(),
    //     7 => remove_fancy_hat(),
    //     _ => (),        //不会使用与之前分支中模式不匹配的任何其他值，并且在这种情况下也不想运行任何代码
    // }

    // fn add_fancy_hat() {}
    // fn remove_fancy_hat() {}
}

fn example_function8()
{
    let config_max = Some(3u8);
    // match config_max 
    // {
    //     Some(max) => println!("The maximum is configured to be {max}"),
    //     _ => (),
    // }
    //与上面等同
    if let Some(max) = config_max 
    {
        println!("The maximum is configured to be {max}");
    }


    // let mut count = 0;
    // match coin 
    // {
    //     Coin::Quarter(state) => println!("state quarter from {state:?}"),
    //     _ => count += 1,
    // }
    //与上面等同
    // if let Coin::Quarter(state) = coin 
    // {
    //     println!("State quarter from {state:?}");
    // }
    // else
    // {
    //     count += 1;
    // }

    dbg!(config_max);
}

#[derive(Debug)]

enum UsState 
{
    Alabama,
    Alaska,
}

impl UsState 
{
    fn existed_in(&self, year: u16) -> bool 
    {
        match self 
        {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

enum Coin 
{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// fn describe_state_quarter(coin: Coin) -> Option<String> 
// {
//     if let Coin::Quarter(state) = coin 
//     {
//         if state.existed_in(1900)
//         {
//             Some(format!("{state:?} is pretty old,for America!"))
//         }
//         else 
//         {
//             Some(format!("{state:?} is relatively new."))
//         }
//     }
//     else 
//     {
//         None
//     }
// }

// fn describe_state_quarter(coin: Coin) -> Option<String> 
// {
//     let state = if let Coin::Quarter(state) = coin 
//     {
//         state
//     }
//     else 
//     {
//         return None;
//     };

//     if state.existed_in(1900)
//     {
//         Some(format!("{state:?} is pprwtty old, for America!"))
//     }
//     else 
//     {
//         Some(format!("{state:?} is relatively new."))
//     }
// }
/* 与上面两种方式等同 */
fn describe_state_quarter(coin: Coin) -> Option<String> 
{
    let Coin::Quarter(state) = coin 
    else 
    {
        return None;
    };

    if state.existed_in(1900) 
    {
        Some(format!("{state:?} is pretty old, for Amereca!"))
    }
    else 
    {
        Some(format!("{state:?} is relatively new."))
    }
}

fn example_function9()
{
    if let Some(desc) = describe_state_quarter(Coin::Quarter(UsState::Alaska))
    {
        println!("desc = {desc}");
    }
}