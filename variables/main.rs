// 未使用mut
fn main () {
    let x = 5;
    x = 6; // 会报错
}

// 使用mut
fn main () {
    let mut x = 5;
    x = 6;
}

/* 
 * rust不带mut声明变量和其他语言的const常量非常像，但并不是一回事。
 * 
 * const会在下一节解释
 */


// shadow
fn main () {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is {}", x);
}

/* 
 * 在其他语言中这样写肯定是报错的，因为不能重复声明相同变量
 * 这里和mut又有一些不同，这样做主要是为了临时修改一些变量，但在这些必须的改动以后，仍然维持变量不被其他任何意外行为修改
 * 
 * 在其他语言中我们可能会声明一个新变量然后复制该变量的值再对新变量进行操作
 * 
 * 和mut的另一个区别是，rust虽然不要求你声明变量时表明类型，但是在赋值完成后，会推断出该变量应该存储的类型
 */
fn main () {
    let mut x = "str";
    x = x.len(); // error ×
}

fn main () {
    let x = "str";
    let x = x.len(); // no problem √
}
