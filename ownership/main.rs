// rust中的每个值都有一个owner，也就是存储它的变量
// 只能同时存在一个owner
// 当owner超出作用域，值会被丢弃

fn main () {
                     // s is not valid
    let s = "hello"; // s comes to the scope, it's valid
                     // s is valid 
}                    // s is no longer valid             


fn main () {
    let s1 = String::from("Hello");
    let s2 = s1;
    println!("s1 is {}", s1); // error，对于引用值，为了避免对于同一个引用执行两次清理导致错误，rust会将第一个值取消，只保留最新的引用变量
}

fn main () {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2); // 使用内置的clone方法，不会报错，成本要高一些
}



fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// 通过上面的函数，我们可以这样理解，对于引用值，传递给函数，之后的代码再也不能访问到


fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}



fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}


// 上面是为了获取s1长度同时不会丢失s1值的写法，看起来就很繁琐
// 这个机制在内存回收方面比较理想，但是带来了很多额外的问题，要用另外的写法来避免

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // 学过c语言的话，对于指针、&、*不会陌生
 
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}


// 这种写法在rust中叫做借用，这样写不会让函数drop掉传进来的引用，但是对于传进来的引用是不可以修改的
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world"); // error
}

// 这样写就可以修改值了

fn main() {
    let s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world"); 
}

// 但是有一条额外限制，对于同一个变量不能有两个可变引用
// 如果允许这么做，你可能会有两个以上的指针同时访问并且修改同一块数据，可能还会有其他指针在读取该数据
// 没有什么机制来处理同步等操作，非常危险

fn main () {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; // error

    println!("{}, {}", r1, r2);

}

// 可变和不可变引用混用，你肯定不希望读取的一个值突然变化了对不对
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);

// 你可以这样做，只要保证不会同时访问并且修改

fn main () {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}

// 引用的周期是从引入到最后一次使用，所以下面的代码是完全可以的
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{} and {}", r1, r2);
// r1 and r2 are no longer used after this point

let r3 = &mut s; // no problem
println!("{}", r3);

// 所以说我们要么只用一个可变指针，要么使用多个不可变指针
// rust这种特点有时会显得让人很烦恼，但正是这样的机制为你指出了潜在的问题。


// Dangling References 
fn main () {
    let reference_to_nothing = dangle();
}

fn dangle () -> &String {
            //  ^ this function's return type contains a borrowed value, but there is no value for it to be borrowed from.
    let s = String::from("hello");
    
    &s
} // 这个函数实际上返回的是指向s的引用，按照我们之前的理解，这里s已经被丢弃，内存也就不再保留
fn dangle () -> String {
    let s = String::from("hello");

    s
}
// 通过上面试图创建空指针的代码演示，我们可以知道rust结合自己的作用域和内存回收机制，保证不会出现悬空指针