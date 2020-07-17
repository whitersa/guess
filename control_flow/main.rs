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


