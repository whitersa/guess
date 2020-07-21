// if 注意 不需要括号
// 条件必须返回布尔值，其他类型不被允许

fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

// if 可以在变量声明时使用
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 }; // 不同分支的值应该会有同样的数据类型，不然会报错

    println!("The value of number is: {}", number);
}

// loops
// loop
fn main() {
    loop {
        println!("again!"); // 无限循环
    }
}

// 带有返回值的loop break
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

// while 
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

// for 
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}


// an example to calculate the nth fibonacci number
fn main () {
    println!("result is {}", fibonacci(10));
}
fn fibonacci (n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    let mut index = 2;
    let mut result = 0;
    loop {
        if index == n {
            break result;
        }
        index = index + 1;
        result = a + b;
        a = b;
        b = result;
    };
    return result;
}


// match control flow
// 允许你用一个值和一些模式对比从而根据每种模式执行不同的行为
// 这些模式可以是文本字符，变量名称，通配符等。
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn value_in_cents (coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// 
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents (coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter (state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
value_in_cents(Coin::Quarter(UsState::Alaska));

// matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);

// rust会在编译时检查代码，确保match覆盖了所有情况，否则就会报错
// 如果你只想处理部分情况，其余可以用_作为占位符处理
// 当然这种情况下 if let （下一部分） 更为合适
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (), // placeholder
}


// if let
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
} // 这是我们之前说到的占位符写法

// rust提供的更简洁的写法
let some_u8_value = Some(0u8);
if let Some(3) = some_u8_value {
    println!("three");
}

// if let - else 
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}

let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}

