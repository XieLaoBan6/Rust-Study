/*
1、Cargo 遵循一个约定：src/main.rs 是与包同名的二进制 Crate 的 Crate 根

*/
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() 
{
    let plant = Asparagus {};

    println!("I'm growing {plant:?}!");
}

/*
mod front_of_house
{
    pub mod hosting
    {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant()
{
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();
}

fn deliver_order()
{}

mod back_of_house
{
    fn fix_incorrect_order()
    {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
*/

/*
mod back_of_house 
{
    pub struct Breakfast 
    {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast 
    {
         //由于 back_of_house::Breakfast 包含一个私有字段，所以该结构体需要提供一个公共关联函数来构造 Breakfast 的实例
         //结构体有任何一个私有字段,外部代码就不能用结构体字面量 Breakfast { ... } 构造它,必须由模块自己提供
        pub fn summer(toast: &str) -> Breakfast 
        {
            Breakfast 
            {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant()
{
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries");
}
*/

/*
mod back_of_house 
{
    //枚举变体的默认是公共
    pub enum Appetizer 
    {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant()
{
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
*/

/*
mod front_of_house 
{
    pub mod hosting 
    {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant()
{
    hosting::add_to_waitlist();
}
*/

/*
mod front_of_house 
{
    pub mod hosting 
    {
        pub fn add_to_waitlist() {}
    }
}

/*
use crate::front_of_house::hosting;

mod customer 
{
    pub fn eat_at_restaurant()
    {
        //super:: 是路径,路径能跨模块
        //也同时因为使用了use，将hosting和front_of_house、customer都放在了crate根模块中，才使得super能够找到hosting
        super::hosting::add_to_waitlist();
    }
}
*/

mod customer
{
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant()
    {
        hosting::add_to_waitlist();
    }
}
*/
/*
use std::collections::HashMap;

fn example_function1()
{
    let mut map = HashMap::new();

    map.insert(1, 2);
}*/

/*
use std::fmt;
use std::io;

fn function1() -> fmt::Result 
{
    Ok(())
}

fn function2() -> io::Result<()> 
{
    Ok(())
}
*/

/*
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result 
{
    Ok(())
}

//as 只改本作用域里的名字,类型本身不变——IoResult<T> 就是 std::io::Result<T>
fn function2() -> IoResult<()> 
{
    Ok(())
}
*/

/*
mod front_of_house 
{
    pub mod hosting 
    {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant()
{
    hosting::add_to_waitlist();
}
*/

/*
//通过指定路径的公共部分，后跟两个冒号，然后在花括号中列出路径中不同的部分来实现
use std::{cmp::Ordering, io};

/*
use std::io;
use std::io::Write;

use std::io::{self, Write};
*/

//将一个路径中定义的所有公共项都引入作用域
use std::collections::*;

*/

/*
目录架构
src/
├── lib.rs                // mod front_of_house; mod customer;
├── front_of_house.rs     // pub mod hosting;
├── front_of_house/
    └── hosting.rs        // pub fn add_to_waitlist() {}
*/

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant()
{
    hosting::add_to_waitlist();
}
