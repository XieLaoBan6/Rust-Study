fn main() {
    // example_function1();
    // example_function2();
    example_function3();
}

/*
vector
*/
fn example_function1()
{
    let v: Vec<i32> = Vec::new();

    let x = vec![1, 2, 3];

    let mut y = Vec::new();

    y.push(5);
    y.push(6);
    y.push(7);
    y.push(8);

    let z = vec![1, 2, 3, 4,5];

    let a: &i32 = &z[2];

    println!("The a element is {a}");

    let third: Option<&i32> = z.get(2);

    match third 
    {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let b = vec![100, 32, 57];

    for i in &b 
    {
        println!("{i}");
    }

    let mut c = vec![100, 32, 57];

    for i in &mut c 
    {
        *i += 500;
        println!("{}", i);
    }

    enum SpreadsheetCell 
    {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec! 
    [
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(100.23),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    {
        let v = vec![1, 2, 3, 4];       //vector 在超出作用域时会被释放
    }
}

/*
String
*/
fn example_function2()
{
    let mut a = String::new();

    let data = "aaaaaaaaa";

    let b = data.to_string();

    let c = "aaaaaaaaa".to_string();

    let d = String::from("aaaaaaaaa");

    let mut e = String::from("foo");
    e.push_str("bar");
    println!("{}", e);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut f = String::from("lo");
    f.push('l');
    println!("{}", f);

    let x = String::from("Hello, ");
    let y = String::from("world!");
    let z = x + &y;

    println!("{}, {}",y ,z);

    let m = String::from("tic");
    let n = String::from("tac");
    let o = String::from("toe");

    let p = format!("{m}-{n}-{o}");      //`format!` 宏生成的代码使用引用，因此此调用不会取得任何参数的所有权
    println!("{}",p);

    let hello = "Здравствуйте";

    let q = &hello[0..4];

    println!("{}", q);

    for l in "Зд".chars() 
    {
        println!("{c}");
    }

    for r in "Зд".bytes()
    {
        println!("{r}");
    }
}

/*
Hash_Map
HashMap 的键类型是 String，值类型是 i32
*/

fn example_function3()
{
    use std::collections::HashMap;

    let mut scares = HashMap::new();

    scares.insert(String::from("Blue"), 10);
    scares.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let scare = scares.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scares 
    {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");

    let mut a = HashMap::new();
    a.insert(String::from("Blue"), 10);

    a.entry(String::from("Yellow")).or_insert(50);
    a.entry(String::from("Blue")).or_insert(50);

    println!("{a:?}");

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}