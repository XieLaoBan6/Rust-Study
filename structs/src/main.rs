fn main() {
    // example_function1();
    // example_function2();
    // example_function4();
    // example_function5();
    // example_function6();
    // example_function7();
    // example_function8();
    // example_function9();
    example_function10();
}

struct User
{
    active: bool,           //bool值，false和true
    username: String,       //字符串
    email: String,
    sign_in_count: u64,
}

// fn build_user(email: String, username: String) -> User
// {
//     User {
//         active: true,
//         username: username,
//         email: email,
//         sign_in_count: 1,
//     }
// }
/* 与上面函数等同，在传参与结构体参数名称一致时，可将后续参数进行省略 */
fn build_user(email: String, username: String) -> User
{
    User{
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn example_function1()
{
    // let user1 = User{
    //     active: true,
    //     username: String::from("someusename123"),
    //     email:String::from("someone@example.com"),
    //     sign_in_count:1,
    // };

    let mut user1 = User{
        active: true,
        username: String::from("someusename123"),
        email:String::from("someone@example.com"),
        sign_in_count:1,
    };

    // let user1 = User{
    //     mut active: true,       //错误写法
    //     username: String::from("someusename123"),
    //     mut email:String::from("someone@example.com"),       //错误写法
    //     sign_in_count:1,
    // };

    /* 由于user2中email、username都被重新定义，只使用了user1中的active、sign_in_count
        这种使用相当于实现copy操作，user1还可以继续被使用*/
    // let user2 = User {
    //     email: String::from("another@example.com"),
    //     username:String::from("anothername"),
    //     ..user1
    // };

    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    /*与上面等同
    由于使用了user1的username，这种使用会导致user1中的数据“move”到user2中，
    这就导致user1的数据不再有效，user1不能再被使用*/
    let user2 = User {
        email: String:: from("another@example.com"),
        ..user1     //必须放在最后
    };

    user1.email = String::from("anotheremail@example.com");

}

//元组结构体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
/* 接受 Color 类型参数的函数不能接受 Point 作为参数 */
fn example_function2()
{
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

//单元结构体
struct AlwaysEqual;
fn example_function3()
{
    let subject = AlwaysEqual;
}

//rectangles "计算矩形面积"

fn example_function4()
{
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {} square pixels.", area(width1, height1));
}

fn area(width: u32, height: u32) -> u32
{
    width * height
}

//元组重构
fn example_function5()
{
    let rect1 = (30,50);

    println!("The area of the rectangle is {} square pixels.", re_area(rect1));
}

fn re_area(dimensions: (u32, u32)) -> u32
{
    dimensions.0 * dimensions.1
}

//结构体重构
#[derive(Debug)]
struct Rectangle
{
    width: u32,
    height: u32,
}

fn example_function6()
{
    let rect1 = Rectangle
    {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", re_re_area(&rect1));

    println!("rect1 is {rect1:?}");     //输出结构体内部内容，需要全局调用#[derive(Debug)]

    println!("rect1 is {rect1:#?}");

    let scale = 2;

    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);
}

fn re_re_area(rectangle: &Rectangle) -> u32
{
    rectangle.width * rectangle.height
}

impl Rectangle
{
    fn re_re_re_area(&self) -> u32
    {
        self.width * self.height
    }
}

fn example_function7()
{
    let rect1 = Rectangle 
    {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", rect1.re_re_re_area());
}

impl Rectangle
{
    fn width(&self) -> bool
    {
        self.width > 0
    }
}

fn example_function8()
{
    let rect1 = Rectangle
    {
        width: 30,
        height: 50,
    };

    if rect1.width()
    {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}

impl Rectangle
{
    fn area(&self) -> u32
    {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool
    {
        self.width > other.width && self.height > other.height
    }
}

fn example_function9()
{
    let rect1 = Rectangle
    {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle
    {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle
    {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

//关联函数,"::" 语法用于关联函数以及模块创建的命名空间
impl Rectangle
{
    fn square(size: u32) -> Self
    {
        Self
        {
            width: size,
            height:size,
        }
    }
}

fn example_function10()
{
    let sq = Rectangle::square(3);

    dbg!(&sq);
}
