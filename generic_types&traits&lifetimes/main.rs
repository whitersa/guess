// 泛型

// 先看个例子
// 找出列表中最大的值
fn main () {
    let num_list = vec![34, 50, 25, 100, 65];

    let mut largest = num_list[0];

    for number in num_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);
}


// 分别找出两个列表中最大的值
// 可以直接重复上面的代码
// 这里就是前面提到的rust shadow
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}


// 但是这样写又啰嗦还容易出问题，重复逻辑也需要多次修改
// 可以将重复的部分抽象成函数
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn main () {
    let num_list = vec![34, 50, 25, 100, 65];
    let result = largest(&num_list);
    println!("The largest number is {}", result);

    let num_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&num_list);
    println!("The largest number is {}", result);
}


// 上面的largest函数会接受任意i32类型的list数据进行处理，如果我们想要它也处理char类型呢
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}

// 这里可以进一步整合，使用泛型来合并两个基本相同的函数
// 当然实际编译的时候会报错，因为无法确定T一定可以排序
// 稍后解决这个问题
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}



// 在结构体中使用泛型
struct Point<T> {
    x: T,
    y: T,
}

fn main () {
    let integer = Point { x: 5, y: 10 };  // 要注意的是两者必须是相同类型，不同类型编译就会报错
    let float = Point { x: 1.0, y: 4.0 }; 
}

// 如果我们想要让结构体的泛型不局限于一种
struct Point<T, U> {
    x: T,
    y: U,
}
fn main () {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}


// 在枚举中使用泛型
enum Option<T> {
    Some(T),
    None,
}
// 如果我们想要让枚举的泛型不局限于一种
enum Result<T, E> {
    Some(T),
    Err(E),
}


// 在结构体和枚举上实现的方法中使用泛型
struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
fn main () {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}

// 我们也可以仅针对部分类型实现方法
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 一个结构体的泛型参数类型并不总是和结构体方法的签名类型保持相同
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}


// 关于泛型与确定类型的性能问题
// 与确定类型相比，泛型并不会让代码运行的更慢，rust采用单态化（monomorphization ）
// 也就是说，在编译时rust就会将泛型转换为对应的具体类型
let integer = Some(5);
let float = Some(5.0);
// 对于这两行代码，rust编译时会识别到有两种类型应用到Option中，i32和f64
// 然后rust会将泛型Option<T>分别转化为Option_i32和Option_f64
enum Option_i32 {
    Some(i32),
    None,
}
enum Option_f64 {
    Some(f64),
    None,
}
fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}


