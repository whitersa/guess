mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist () {}
        fn seat_at_table () {}
    }
    mod serving {
        fn take_order () {}
        fn serve_order () {}
        fn take_payment () {}
    }
}
/** 
 * crate
 *   └─front_of_house
 *       ├─hosting
 *       │   ├─add_to_waitlist
 *       │   └─seat_at_table
 *       └─serving
 *           ├─take_order
 *           ├─server_order
 *           └─take_order
 **/

// 可以参考文件系统

// 我们可以有两种方式调用函数
// 绝对路径（比较推荐）
pub fn eat_at_restaurant () {
    crate::front_of_house::hosting::add_to_waitlist();
}
// 相对路径
pub fn eat_at_restaurant () { // 
    front_of_house::hosting::add_to_waitlist();
}
         

// 另外一种相对路径引用
fn serve_order () {
}

mod back_of_house {
    fn fix_incorrect_order () {
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}
}


// struct为public时它里面的属性值还是需要单独设置私有或者公共
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    } 
    impl Breakfast {
        pub fn summer (toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
pub fn eat_at_restaurant () {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}

// 如果使用枚举，枚举设置为公共属性的时候，里面的值也是公共的
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
pub fn eat_at_restaurant () {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// 上面的绝对路径看起来也比较繁琐，可以使用use简化
// 这样的写法仍然会检查引用方法是否为公共方法
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist () {}
    }
}
use crate::front_of_house::hosting;
pub fn eat_at_restaurant () {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// 还可以使用相对路径来进行引用，把crate换成self
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist () {}
    }
}
use self::front_of_house::hosting;
pub fn eat_at_restaurant () {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}


// 上面的写法并不是最简单的，可以进一步简化
// 但是这样写并不推荐，最好指定父级名称，表明这并不是一个本地方法
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist () {}
    }
}
use crate::front_of_house::hosting::add_to_waitlist;
pub fn eat_at_restaurant () {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}

// 如果我们要引用不同库的相同名称的方法，use只要写到父级名称
use std::fmt;
use std::io;
fn function1() -> fmt::Result {
    // --snip--
}
fn function2() -> io::Result<()> {
    // --snip--
}


// 上面的例子还有一种解决方案 - as
// as可以提供一个新的名称
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}


// 使用use引入的时候，引用的方法又会变为private
// 我们需要再次设置为public
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
 

// 如果我们需要从一个包里引用多个项，可以使用简短的语法来引入
// before 
use std::cmp::Ordering;
use std::io;
// after
use std::{cmp::Ordering, io};

// before 
use std::io;
use std::io::Write;
// after
use std::io::{self, Write};


// * - once for all !!!
// 当然这样会让代码透明度降低
use std::collections::*;



// 划分模块到不同文件
// 以下为本文件内的代码
/** 
 * src
 *   ├─front_of_house<folder>
 *   │   └─hosting.rs<file>
 *   ├─front_of_house.rs<file>
 *   └─lib.js<file>
 **/
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}