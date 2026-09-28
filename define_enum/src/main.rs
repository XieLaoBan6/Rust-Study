fn main() {
    // example_function4();
    // example_function5();
    example_function6();
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

    match dice_roll 
    {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),      //最后一个分支忽略了所有其他值
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),        //不会使用与之前分支中模式不匹配的任何其他值，并且在这种情况下也不想运行任何代码
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
}