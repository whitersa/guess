// rust使用snake方式命名函数，例如some_function
fn main () {
    println!("Hello World!")
    example_function(); // 和c、js等语言类似，函数会被提升
}
fn example_function () {
    println!("This is a new function");
}


// 函数的参数必须规定类型
fn main () {
    another_function(5, 10);
}
fn another_function (x: i32, y: i64) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
} 


// {} 在函数中也算是表达式，如下
fn bracket_function () {
    let x = 5;
    let y = {
        let x = 3;
        x + 4 // 不需要; ！！！
    };
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}


// 使用->来表明返回值类型
fn five() -> i32 {
    5 // 完全有效的写法，但是如果加了; 就是语句而不是表达式了
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}