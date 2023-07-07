#[allow(unused)]
enum Color {
    R(i16),
    G(i16),
    B(i16),
}

enum MColor {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

// 枚举值相互嵌套
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(MColor),
}

struct Foo {
    x: (u32, u32),
    y: u32,
}

// 该枚举等价于，所以他们可以被当做函数使用
// fn Color::R(c: i16) -> Color { /* ... */ }
// fn Color::G(c: i16) -> Color { /* ... */ }
// fn Color::B(c: i16) -> Color { /* ... */ }
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn age() -> i32 {
    15
}

fn some_number() -> Option<i32> {
    Some(42)
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    //静态方法
    fn origin() -> Point {
        Point { x: 0, y: 0 }
    }
    //根据指定坐标构造
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // &self 其实是 self: &Self 的语法糖，表示不可变引用
    fn area(&self) -> i32 {
        self.height * self.width
    }

    // &mut self 其实是 self: &mut Self, 表示可变引用
    fn plus_one(&mut self) {
        self.width += 1;
        self.height += 1;
    }

    // self 直接将所有权转移
    fn transfer(self) -> Rectangle {
        self
    }
}

#[cfg(test)]
mod tests {

    use lazy_static::__Deref;

    use super::{add, Foo, MColor, Message, Point, Rectangle};
    use crate::leetcode::basic::{age, some_number, Color};

    use core::num;
    use std::{
        fmt::{Debug, Display, Formatter},
        fs::File,
        io::{Error, ErrorKind, Read},
        mem::{self, size_of_val},
        ops::{Add, Mul},
        rc::Rc,
        vec, cell::RefCell,
    };

    #[test]
    fn loop_test() {
        let mut counter = 0;
        //不同于其他语言，rust 的 loop 循环是可以返回值的，因为 loop 循环是一个表达式，表达式可以求值，这样就可以作为赋值语句使用.
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2;
            }
        };

        assert_eq!(result, 20);
    }

    #[test]
    fn if_let_test() {
        let number = Some(5);
        //由于 match 模式匹配必须要指出所有的可能性，所以在使用上不是很优雅，因此有了 if let，可以说它是 match 的语法糖，可以按需只匹配自己想要的。
        if let Some(value) = number {
            println!("value is {}", value);
        }

        //rust中的 None
        let none: Option<i32> = None;
        if let Some(n) = none {
            println!("value is {}", n);
        } else {
            println!("value is none");
        }
    }

    #[test]
    fn loop_match_test() {
        let mut number = Some(6);
        loop {
            match number {
                Some(value) => {
                    if value > 9 {
                        number = None;
                    } else {
                        number = Some(value + 1);
                        println!("number is {:?}", number)
                    }
                }
                None => break,
            }
        }
        println!("number is none: {}", number.is_none());
    }

    #[test]
    fn while_let_test() {
        let mut number = Some(6);
        //同 if let 类似，while let 可以简化代码的书写方式，使得呈现上更加优雅。
        while let Some(value) = number {
            if value > 9 {
                number = None;
            } else {
                number = Some(value + 1);
                println!("number is {:?}", number);
            }
        }
        println!("number is none: {}", number.is_none());
    }

    #[test]
    fn size_of_val_test() {
        // 同样一个函数，我们再赋值给一个变量时，在指定函数指针类型时，占用8个字节. 不指定时，为函数项类型，占用0字节，函数项类型在必要时可以自动转化为函数指针类型
        let add = add;
        let add_ptr: fn(i32, i32) -> i32 = add;
        println!("add size: {} ", size_of_val(&add));
        println!("add_ptr size: {}", size_of_val(&add_ptr));

        //枚举项占用的大小也是0
        println!("Color::B size: {}", size_of_val(&Color::B));
    }

    #[test]
    fn match_test() {
        let number = 12;

        //match 分支必须覆盖所有可能得情况, rust 提供 match 关键字用于模式匹配，类似于其他语言中的 switch，不同的是 match 必须列出所有可能情况。
        match number {
            //可以匹配单个值
            1 => println!("One"),
            //可以匹配多个值
            2 | 3 | 4 | 5 => println!("2 -> 5"),
            //还可以匹配一个范围
            6..=10 => println!("6 -> 10"),
            _ => println!("others"),
        }
    }

    #[test]
    fn match_enum_test() {
        let msg = Message::ChangeColor(MColor::Hsv(0, 160, 255));
        //match 还可以用于解构枚举 enum
        match msg {
            Message::ChangeColor(MColor::Rgb(r, g, b)) => {
                println!("change the color to red {}, green {}, and blue {}", r, g, b)
            }
            Message::ChangeColor(MColor::Hsv(h, s, v)) => println!(
                "change the color to hue {}, saturation {}, nd value {}",
                h, s, v
            ),
            //匹配剩下所有的情况
            _ => (),
        }

        let pair = (0, 0);
        match pair {
            // 只会匹配到这里
            (0, y) => println!("First is `0` and `y` is `{:?}`", y),
            (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
            _ => (),
        }
    }

    #[test]
    fn match_filter_test() {
        let pair = (2, -2);
        // match 模式匹配可以加上 if 条件语句来过滤分支，提供更加灵活的匹配方式：
        match pair {
            (x, y) if x + y == 0 => println!("{} + {} == 0", x, y),
            (x, y) if x == y => println!("{} == {}", x, y),
            (x, y) if x % y == 0 => println!("{} % {} == 0", x, y),
            _ => (),
        }
    }

    #[test]
    fn match_var_binding_test() {
        println!("tell me type of person you are:");
        //match 提供了 @ 运算符用于将值绑定到变量
        match age() {
            0 => println!("Im not born yet I guess"),
            // 可以直接 `match` 1 ..= 12，但怎么把岁数打印出来呢？相反，在 1 ..= 12 分支中绑定匹配值到 `n` 。现在年龄就可以读取了。
            n @ 1..=12 => println!("I'm a child of age:{}", n),
            n @ 13..=19 => println!("I'm a teen of age:{}", n),
            //其他情况
            n => println!("I'm n old person of age {:?}", n),
        }

        //也可用于枚举
        match some_number() {
            Some(n @ 42) => println!("the answer: {}!", n),
            Some(n) => println!("not instresting... {}", n),
            _ => (),
        }
    }

    #[test]
    fn destructure_test() {
        // 解构结构体的成员，字段x是一个元组，分别解析到a，b；字段y解析到y
        let foo = Foo { x: (2, 3), y: 3 };
        let Foo { x: (a, b), y } = foo;
        println!("a = {}, b = {}, y = {}", a, b, y);

        // 可以解构结构体并重命名变量，成员顺序并不重要；将y解析成i；x解析成j；
        let Foo { x: i, y: j } = foo;
        println!("i = {:?}, j = {:?}", i, j);

        // 也可以忽略某些变量，只解析y，忽略x
        let Foo { y, .. } = foo;
        println!("y = {}", y);
    }

    #[test]
    fn destructure_dereference_test() {
        //对指针来说，解构（destructure）和解引用（dereference）要区分开，因为这两者的概念 是不同的
        //1.解引用使用 *
        //2.解构使用 &、ref、和 ref mut

        //获得一个 `i32` 类型的引用。`&` 表示取引用。
        let reference = &4;

        match reference {
            // 如果用 `&val` 这个模式去匹配 `reference`，就相当于做这样的比较：`&i32`（译注：即 `reference` 的类型）|`&val`（译注：即用于匹配的模式）
            // ^ 我们看到，如果去掉匹配的 `&`，`i32` 应当赋给 `val`。 译注：因此可用 `val` 表示被 `reference` 引用的值 4。
            &val => println!("Got a value via destructuring: {:?}", val),
        }

        //如果不想用 `&`，需要在匹配前解引用。
        match *reference {
            val => println!("Got a value via destructuring: {:?}", val),
        }

        // 如果一开始就不用引用，会怎样？ `reference` 是一个 `&` 类型，因为赋值语句的右边已经是一个引用。但下面这个不是引用，因为右边不是。
        let _not_a_reference = 3;
        // Rust 对这种情况提供了 `ref`。它更改了赋值行为，从而可以对具体值创建引用。下面这行将得到一个引用。
        let ref _is_a_reference = 3;

        // 相应地，定义两个非引用的变量，通过 `ref` 和 `ref mut` 仍可取得其引用。
        let value = 5;
        let mut mut_value = 6;
        // 使用 `ref` 关键字来创建引用。译注：下面的 r 是 `&i32` 类型，它像 `i32` 一样可以直接打印，因此用法上似乎看不出什么区别。但读者可以把 `println!` 中的 `r` 改成 `*r`，仍然能
        // 正常运行。前面例子中的 `println!` 里就不能是 `*val`，因为不能对整数解引用。
        match value {
            ref r => println!("Got a reference to a value: {:?}", r),
        }

        match mut_value {
            ref mut m => {
                // 已经获得了 `mut_value` 的引用，先要解引用，才能改变它的值。
                *m += 10;
                println!("We add 10. mut_value: {:?}", m);
            }
        }

        //& 和 ref 都表示获取引用，只是一个出现在表达式左边一个出现在右边，当 & 出现在右边的时候等价于 ref 出现在左边，& 出现在左边的时候等价于 * 出现在右边：
    }

    #[test]
    fn refutable_irrefutable_test() {
        //模式有两种形式：refutable（可反驳的）和 irrefutable（不可反驳的）。能匹配任何传递的可能值的模式被称为是不可反驳的（irrefutable），
        //反之，对某些可能的值进行匹配会失败的模式被称为是可反驳的（refutable）。
        //举个例子: let x = 5; 中的 x 可以匹配任何值不会失败，所以称为不可反驳。if let Some(x) = a_value 中，如果 a_value 是 None，那么这个表达式就匹配不上，所以称为可反驳。
        //为什么有这么个模式？因为，函数参数，let，for 只能接收不可反驳的模式，也就是说只允许匹配成功，是一种确定性操作。而 if let，或者 while let 表达式被限制为只能接收可反驳的模式，
        //也就是说他们允许出现匹配不上，即匹配失败的情况，再者说，他们的出现就是为了处理成功和失败这两种情况。下面的这段代码就会编译失败，因为没有处理 a_value 为 None 的情况，let 也处理不了：

        //let a_value: Option<i32> = Some(32);
        //let Some(x) = a_value;
        //println!("{}", x);

        //基于此，match 匹配分支必须使用可反驳模式，除了最后一个分支需要使用能匹配任何剩余值的不可反驳模式。Rust 允许我们在只有一个匹配分支的 match 中使用不可反驳模式，不过这么做不是特别有用，
        //并可以被更简单的 let 语句替代。
    }

    //方法通常用于和函数对比，和函数的区别是方法附着于对象，方法分为静态方法和实例方法，静态方法常用语构造对象，实例方法中通过关键字 self 来引用对象中的数据
    #[test]
    fn static_method_test() {
        let origin = Point::origin();
        let other = Point::new(3, 6);
        println!("origin: {:?}, other: {:?}", origin, other);
    }

    #[test]
    fn method_test() {
        let mut rec = Rectangle {
            width: 1,
            height: 2,
        };
        println!("rectangle {:?}, area is: {}", rec, rec.area());

        rec.plus_one();
        println!("rectangle {:?}, area is: {}", rec, rec.area());

        let rec1 = rec.transfer();
        // rec; 编译失败，rec 的所有权已经转移至 rec1
    }

    //闭包是函数式编程中不可获取的一员，rust 对此也提供了支持，也叫 lambda，能够捕获环境中的变量，例如：|val| val + x
    //这种超级简便的语法使得它在临时使用时非常方便，输入和返回值类型都可以自行推导，但是必须指定输入参数名称。在声明参数是，同函数不同，它是使用 || 而不是 () 将参数包裹起来；
    //另外们对于单个表达式的闭包，{} 是可以省略的。
    #[test]
    fn lambda_test() {
        //闭包会自动满足函数功能的要求，使得闭包不需要类型说明就可以工作。这允许变量捕获（capture）灵活地适应使用场合，既可移动（move）又可借用（borrow）变量。
        //闭包可以通过：引用 &T， 可变引用 &mut T，值 T 自动捕获变量，也可以通过 move 强制获得变量的所有权：
        let color = "green";

        // 这个闭包打印 `color`。它会立即借用（通过引用，`&`）`color` 并将该借用和闭包本身存储到 `print` 变量中。`color` 会一直保持被借用状态直到 `print` 离开作用域。
        let print = || println!("color : {}", color);
        print();
        print();

        let mut count = 1;
        // 这个闭包使 `count` 值增加。要做到这点，它需要得到 `&mut count` 或者 `count` 本身。`inc` 前面需要加上 `mut`，因为闭包里存储着一个 `&mut` 变量。
        // 调用闭包时，该变量的变化就意味着闭包内部发生了变化。因此闭包需要是可变的。
        let mut inc = || {
            count += 1;
            println!("count : {}", count);
        };

        inc();
        inc();
        // 不能再次获得 count 的可变引用，因为前面的闭包中已经借用一次了
        // let reborrow = &mut count;
        // reborrow += 1;

        //不可复制类型(non-copy type)
        let movable = Box::new(6);
        // `mem::drop` 要求 `T` 类型本身，所以闭包将会捕获变量的值。这种情况下，可复制类型将会复制给闭包，从而原始值不受影响。不可复制类型必须移动
        // （move）到闭包中，因而 `movable` 变量在这里立即移动到了闭包中。
        let consume = || {
            println!("movable is {:?}", movable);
            mem::drop(movable);
        };

        // `consume` 消耗了该变量，所以该闭包只能调用一次
        consume();

        // 通过 move 关键字强制将 numbers 的所有权移动到闭包中
        let numbers = vec![1, 2, 3];
        let contains = move |needle| numbers.contains(needle);
        println!("numbers include 1 ? {}", contains(&1));
        println!("numbers include 4 ? {}", contains(&4));

        // 由于 numbers 的所有权已经被移入 contains 中，所以这里不能再使用, println!("numbers length is {}", numbers.len());
    }

    fn plus_one<T>(mut f: T)
    where
        T: FnMut(),
    {
        println!("execute plus one.");
        f();
    }

    //该函数将闭包作为参数并调用它, 闭包没有输入值和返回值
    fn apply<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    }

    //虽然闭包可以自动做类型推断，但是在编写函数以闭包作为参数时，还是得必须明确指定类型，可以通过以下三个之一来指定闭包捕获变量的类型，他们的受限程度依次递减：
    //1.Fn：表示捕获方式为通过引用（&T）的闭包
    //2.FnMut：表示捕获方式为通过可变引用（&mut T）的闭包
    //3.FnOnce：表示捕获方式为通过值（T）的闭包
    #[test]
    fn lambda_parameter_test() {
        let mut number = 1;
        plus_one(|| number += 1);
        println!("number is {}", number);

        let greeting = "hello";
        // 不可复制的类型。`to_owned` 从借用的数据创建有所有权的数据。
        let mut farewell = "goodbye".to_owned();
        // 捕获 2 个变量：通过引用捕获 `greeting`，通过值捕获 `farewell`。
        let diary = || {
            // `greeting` 通过引用捕获，故需要闭包是 `Fn`
            println!("I said {}", greeting);

            // 下文改变了 `farewell` ，因而要求闭包通过可变引用来捕获它。现在需要 `FnMut`。
            farewell.push_str("!!!");
            println!("Then I screamed {}", farewell);

            // 手动调用 drop 又要求闭包通过值获取 `farewell`。现在需要 `FnOnce`。
            drop(farewell);
        };

        //以闭包作为参数，调用函数 `apply`
        apply(diary);
    }

    fn create_fn() -> impl Fn() {
        let text = "Fn".to_owned();
        move || println!("This is a {}", text)
    }

    fn create_fnmut() -> impl FnMut() {
        let text = "FnMut".to_owned();
        move || println!("This is a {}", text)
    }

    fn create_fnonce() -> impl FnOnce() {
        let text = "FnOnce".to_owned();
        move || println!("This is a {}", text)
    }

    //闭包可以作为输入参数，也可以作为返回值返回，由于闭包的类型是未知的，所以只有使用 impl Trait 才能返回一个闭包。
    //除此之外，还必须使用 move 关键字，它表明所有的捕获都是通过值进行的。因为在函数退出时，任何通过引用的捕获都被丢弃，在闭包中留下无效的引用。
    #[test]
    fn lambda_return_value_test() {
        let fn_plain = create_fn();
        let mut fn_mut = create_fnmut();
        let fn_once = create_fnonce();

        fn_plain();
        fn_mut();
        fn_once();
    }

    //函数指针是指向代码而非数据的指针。它们可以像函数一样被调用。与引用一样，函数指针被假定为不为空，因此如果想通过 FFI 传递函数指针并能够容纳空指针，需要使用所需的的类型 Option<fn()> 。
    //函数指针的类型是 fn，注意和 Fn 区分，后者是闭包实现的 trait 类型。 函数指针实现了所有三个闭包 trait（Fn、FnMut 和 FnOnce），所以总是可以在调用期望闭包的函数时传递函数指针作为参数。
    //倾向于编写使用泛型和闭包 trait 的函数，这样它就能接受函数或闭包作为参数。Fn 系列 trait 由标准库提供，所有的闭包都实现了 Fn、FnMut 或 FnOnce 中的一个或多个。

    //我们可以将一个闭包转换为函数指针作为参数传入，但是仅限于没有捕获任何环境变量的闭包，这个从闭包和函数的概念上也能区分出来，闭包相对于函数，就是捕获了环境变量。
    //没有捕获任何环境变量的闭包会被编译器重写为匿名独立函数。
    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    #[derive(Debug)]
    struct State(u32);

    #[derive(Debug)]
    struct RGB(i32, i32, i32);

    fn color() -> RGB {
        RGB(1, 1, 1)
    }

    fn show(f: fn() -> RGB) {
        println!("color is {:?}", f());
    }

    #[test]
    fn lambda_fn_point_test() {
        //闭包作为参数
        let list_of_numbers = vec![1, 2, 3];
        let list_of_string: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
        println!("{:?}", list_of_string);

        //函数作为参数
        let list_of_numbers = vec![1, 2, 3];
        let list_of_string: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
        println!("{:?}", list_of_string);

        //元组结构体转为参数
        //在构造元组结构体时使用 () 语法进行初始化，很像是函数调用，实际上它们确实被实现为返回由参数构造的实例的函数，所以它们也被称为实现了闭包 trait 的函数指针。
        let list_of_statuses: Vec<Status> = (0u32..5).map(Status::Value).collect();
        println!("{:?}", list_of_statuses);
        let list_of_statuses: Vec<State> = (0u32..5).map(State).collect();
        println!("{:?}", list_of_statuses);

        //没有捕获任何环境变量的闭包会被编译器重写为匿名独立函数。
        let c = || RGB(2, 2, 2);
        //闭包自动转换为函数指针
        show(c);
        show(color);

        //闭包不能转换为函数, 闭包捕获环境变量之后，就不能再转换为函数了。
        let a = String::from("abcd");
        let x = || println!("{}", a);

        fn wrap(c: fn() -> ()) {
            c()
        }

        //wrap(x); 编译这段代码就会报错，帮我们指出闭包只有在没有捕获任何环境变量的情况下才能转换为函数.
    }

    #[derive(Debug, PartialEq)]
    struct Points {
        x: i32,
        y: i32,
    }

    impl Add for Points {
        //关联类型 Output 指定为 Point
        type Output = Points;

        fn add(self, other: Points) -> Points {
            Points {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    #[derive(Debug, PartialEq)]
    struct Millimeters(u32);
    struct Meters(u32);

    // RHS 默认类型参数指定为：Meters
    impl Add<Meters> for Millimeters {
        // 关联类型 Output 指定为 Millimeters，指定 add 方法返回值类型
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }
    //trait 用于定义共享的行为，trait 告诉 Rust 编译器某个特定类型拥有可能与其他类型共享的功能。可以通过 trait 以一种抽象的方式定义共享的行为，可以使用 trait bounds
    //指定泛型是任何拥有特定行为的类型。trait 定义是一种将方法签名组合起来的方法，目的是定义一个实现某些目的所必需的行为的集合，这里定义的方法可以只是签名说明而没有函数体。
    #[test]
    fn trait_default_test() {
        assert_eq!(
            Points { x: 1, y: 0 } + Points { x: 2, y: 3 },
            Points { x: 3, y: 3 }
        );

        let meters = Meters(1);
        let millimeters = Millimeters(1);
        assert_eq!(Millimeters(1001), millimeters + meters);
    }

    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }

    //Rust 既不能避免一个 trait 与另一个 trait 拥有相同名称的方法，也不能阻止为同一类型同时实现这两个 trait。甚至直接在类型上实现开始已经有的同名方法也是可能的。
    //下面的示例中通过在方法名称前面添加 trait 限定符，我们向 rust 指定我们需要哪个实现。
    #[test]
    fn trait_fully_qualifred_test() {
        let person = Human;
        person.fly();
        Pilot::fly(&person);
        Wizard::fly(&person);

        //<Type as Trait>::function(receiver_if_method, next_arg, ...);
        Human::fly(&person);
        <Human as Pilot>::fly(&person);

        //实现 trait 时需要注意的一个限制是，只有当 trait 或者要实现 trait 的类型位于 crate 的本地作用域时，才能为该类型实现 trait，这个限制是被称为相干性（coherence）
        //的程序属性的一部分，或者更具体的说是孤儿规则（orphan rule）。这条规则确保了其他人编写的代码不会破坏你代码，反之亦然。没有这条规则的话，两个 crate
        //可以分别对相同类型实现相同的 trait，而 Rust 将无从得知应该使用哪一个实现。
    }

    trait Summary {
        fn author(&self) -> String;

        fn summarize(&self) -> String {
            format!("author is {}", self.author())
        }
    }

    struct Article {
        content: String,
        author: String,
    }

    //默认实现指我们在定义 trait 方法时提供默认的实现行为，在为类型实现 trait 时，就可以不用再去实现它的方法了。默认实现的 trait 方法中还允许我们调用相同 trait 的其他方法，
    //即使他们没有实现。
    #[test]
    fn trait_default_impl_test() {
        let article = Article {
            content: "hello".to_string(),
            author: "michael".to_owned(),
        };
        println!("{}", article.summarize());
    }

    //如下，我们定义 notify 函数，指定 item 参数为实现了 Summary 的一个类型。
    trait Summarys {
        fn author(&self) -> String;
        fn summarize(&self) -> String {
            format!("author is {}", self.author())
        }
    }

    fn notify(item: impl Summarys) {
        println!("notify: {}", item.summarize());
    }

    //impl 看起来比较直观，它实际上是一个较长形式的语法糖，称之为 trait bound，所以前面的 impl Summary 等价于如下的形式：
    fn notify_bound<T: Summarys>(item: T) {
        println!("notify: {}", item.summarize());
    }

    //trait bound 可以理解为将 trait 绑定到某个泛型上，当需要将参数声明为实现了多个 trait 的类型时，可以使用 + ：
    fn notify_two_trait_bound<T: Summarys + Display>(item: T) {
        println!("{}", item);
    }

    //使用过多的 trait bound 也有缺点。每个泛型有其自己的 trait bound，所以有多个泛型参数的函数在名称和参数列表之间会有很长的 trait bound 信息，这使得函数签名难以阅读。
    //为此，Rust 有另一个在函数签名之后的 where 从句中指定 trait bound 的语法。
    fn notify_complex_where<T, U>(item: T, item2: U)
    where
        T: Summarys + Display,
        U: Debug + Copy,
    {
        println!("item: {}, item2: {:?}", item, item2);
    }

    impl Summary for Article {
        fn summarize(&self) -> String {
            self.content.clone()
        }

        fn author(&self) -> String {
            self.author.clone()
        }
    }

    struct Tweet {
        content: String,
        author: String,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            self.content.clone()
        }

        fn author(&self) -> String {
            self.author.clone()
        }
    }

    //我们可以将函数的返回值定义为实现了某个 trait 的类型，例如我们指定 returns_summarizable 函数返回实现了 Summary 的类型：
    fn returns_summarizable() -> impl Summary {
        Tweet {
            content: String::from("of course, as you probably already know people."),
            author: "michael".to_string(),
        }
    }
    //但是如果我们想从一个函数中返回多种实现了同一 trait 的类型，就不可以了，如下面这段代码就不能通过编译，因为 rust 需要在编译时期就确定函数返回值的大小。
    //返回不同的类型，意味着函数的返回值大小是不确定的，这对于 rust 来说是不允许的。

    //如果我们确实想这样做，我们可以使用 Box<T> 类型，这个类型将数据实际存储在堆上，保留该数据的指针，所以其大小是固定的，这样就实现了动态分发：
    fn try_return_multile_types(swith: bool) -> Box<dyn Summary> {
        if swith {
            Box::new(Tweet {
                content: String::from("of course, as you probably already know people."),
                author: "michael".to_string(),
            })
        } else {
            Box::new(Article {
                content: String::from("of course, as you probably already know people."),
                author: "michael".to_string(),
            })
        }
    }

    #[test]
    fn trait_return_value_test() {
        let tweet = returns_summarizable();
        println!("{}", tweet.summarize());
        let multi_summary = try_return_multile_types(false);
        println!("{}", multi_summary.summarize());
    }

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    //有时候我们在为某一个泛型结构体实现方法的时候，首先需要它的类型实现某些 trait。如下示例中，类型 Pair<T> 总是实现了 new 方法，不过只有那些为 T 类型实现了 PartialOrd trait
    //（来允许比较） 和 Display trait （来启用打印）的 Pair<T> 才会实现 cmp_display 方法：
    impl<T: PartialOrd + Display> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("the largest member is x = {}", self.x);
            } else {
                println!("the largest member is y = {}", self.y);
            }
        }
    }
    //也可以对任何实现了特定 trait 的类型有条件地实现 trait。对任何满足特定 trait bound 的类型实现 trait 被称为 blanket implementations，他们被广泛的用于 Rust 标准库中。
    //例如，标准库为任何实现了 Display trait 的类型实现了 ToString trait。这个 impl 块看起来像这样：

    #[test]
    fn trait_cond_impl_test() {
        let pair = Pair { x: 1, y: 10 };
        pair.cmp_display();
        //所以可以对任何实现了 Display trait 的类型调用由 ToString 定义的 to_string 方法。
        pair.x.to_string();
    }

    //我们演示过可以在 trait 的默认实现中使用相同 trait 的其他方法，即使该方法未实现。但是，我们有时也需要在当前 trait 中使用其他 trait 中的功能，这就形成了 trait 依赖，
    //被依赖的 trait 的我们称之为当前 trait 的 父 trait。
    trait OutlinePrint: Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            //OutlinePrint 在定义的默认方法 outline_print 调用了 fmt::Display 中的 to_string 方法：
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    impl OutlinePrint for Point {}

    impl Display for Point {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "({}, {})", self.x, self.y)
        }
    }

    #[test]
    fn trait_father_test() {
        let point = Point { x: 12, y: 13 };
        point.outline_print();
    }

    //Copy 的全称是 std::marker::Copy，它的内部其实什么方法都没有，但是实现它必须实现 Clone。一旦一个类型实现 Copy 意味着在任何需要的时候，
    //我们可以简单的通过内存拷贝（C 语言的按位拷贝 memcpy）实现该类型的复制，而不会出现任何问题。在变量绑定、函数参数传递、函数返回值传递等场景下，
    //它都是 copy 语义，而不再是默认的 move 语义

    //i32 实现了 Copy，所以我们在使用 let 表达式的时候，其实是复制而不是所有权转移。String 没有实现 Copy，所以它在使用 let 表达式的时候，是所有权转移
    //并不是所有的类型都可以实现 Copy 。Rust 规定，对于自定义类型，只有所有的成员都实现了 Copy ，这个类型才有资格实现 Copy。例如下面的类型:
    #[derive(Clone, Copy)]
    struct P {
        x: i32,
        y: i32,
    }

    //但是看下面的 PointList 类型，他就不能实现 Copy，因为 Vec<T> 没有实现 Copy。
    struct PointList {
        points: Vec<P>,
    }

    //虽然 PointList 不能实现 Copy，但是是由于共享引用 &T 可以 Copy，所以我们可以实现一个 PointListWrapper，包含 PointList 的一个引用，
    //这样即使 PointList 不能 Copy，PointListWrapper 也可以 Copy。
    #[derive(Clone, Copy)]
    struct PointListWrap<'a> {
        point_list_ref: &'a PointList,
    }

    //Clone 的全称是 std::clone::Clone;，他定义了两个方法，其中 clone_from 默认实现。

    //clone 方法一般用于基于语义的复制操作。所以，它做什么事情，跟具体类型的作用息息相关。比如对于 Box 类型，clone 就是执行的深拷贝，而对于 Rc 类型，clone 做的事情就是把引用计数值加 1。
    //你可以根据情况在 clone 函数中编写任意的逻辑。但是有一条规则需要注意：对于实现了 Copy 的类型，它的 clone 方法应该跟 Copy 语义相容，等同于按位拷贝。

    //String 虽然未实现 Copy，但是它实现了 Clone。
    #[test]
    fn trait_copy_clone_test() {
        let a = "hello world".to_string();
        let b = a.clone();
        println!("{}, {}", a, b);
    }

    //闭包捕获非 Copy 类型，获取其所有权
    //这个例子中，consume_and_return_x 捕获了 x 并获得了其所有权，并且在第一次调用时已经将 x 的所有权转移，所以无法再次调用。
    fn consume_with_relish<F>(func: F)
    where
        F: FnOnce() -> String,
    {
        // func 消耗了它捕获的环境变量，所以它只能被运行一次
        println!("consumed: {}", func());
        println!("Delicious!");
        // 如果再尝试调用，func()，将会出现编译错误
        // func();
    }

    //闭包捕获可 Copy 类型
    fn consume_with_relish_2<F>(func: F)
    where
        F: FnOnce() -> i32,
    {
        println!("consumed: {}", func());
        // 这里调用会出现编译错误，在首次调用时，会将捕获的变量 x 消耗掉
        // func();
    }

    //非 Copy 类型，获取所有权，但是并不消耗
    //consume_and_return_x 获取了变量 x 的所有权，因为 String 不可 Copy。但是我们在使用的时候并没有消耗它的所有权，所以是可以多次使用的。
    //这个时候 consume_and_return_x 其实已经实现了 Fn。
    fn consume_with_relish_3<F>(func: F)
    where
        F: Fn(),
    {
        func();
        func();
    }

    //FnMut 实例可以被重复多次调用，并且可以改变环境变量。它被那些捕获了环境变量可变引用的闭包，所有 Fn 的实现者，以及函数指针自动实现。对于任何实现了 FnMut 的类型 F，
    //&mut F 也实现了 FnMut。

    //另外，因为 FnOnce 是 FnMut 的 父 trait，所以任何需要 FnOnce 的地方都可以传入 FnMut。当你需要对一个类似函数类型的参数限定为，可调用多次并且可改变内部状态时，可以使用 FnMut。
    fn do_twice<F>(mut func: F)
    where
        F: FnMut(),
    {
        func();
        func();
    }

    //Fn 要和 函数指针 fn 区别，Fn 被那些仅捕获环境中变量不可变引用的闭包，或者不捕获任何东西的闭包，或者函数指针自动实现。需要 Fn 或者 FnMut 的地方，都可以传入 Fn。
    //如果类型 F 实现 Fn，那么 &F 也将自动实现 Fn。
    fn call_with_one<F>(func: F) -> usize
    where
        F: Fn(usize) -> usize,
    {
        func(1)
    }

    #[test]
    fn trait_fnonce_test() {
        let x = String::from("x");
        let consume_return_x = move || println!("{}", x);

        let y = 1;
        // 这里 y 移入闭包的时候，由于 y 默认实现了 Copy 类型，所以执行的是 copy 操作，而不是获取所有权
        let consume_return_y = move || y;
        consume_with_relish_2(consume_return_y);
        println!("print y: {}", y);

        consume_return_x();
        consume_return_x();
        consume_with_relish_3(consume_return_x);

        let mut z = 1;
        let add_two_z = || z += 2;
        do_twice(add_two_z);
        println!("z: {}", z);

        let double = |z| z * 2;
        assert_eq!(call_with_one(double), 2)
    }

    //Deref 允许我们重载解引用运算符 *，它包含一个 deref 方法：
    // pub trait Deref {
    // type Target: ?Sized;
    // fn deref(&self) -> &Self::Target;

    //常规引用是一个指针类型，一种理解指针的方式是将其看成指向储存在其他某处值的箭头。下面的示例中创建了一个 i32 值的引用，接着使用解引用运算符来跟踪所引用的数据：

    //定义我们自己的 MyBox 类型，实现 Deref，deref 方法体中写入了 &self.0，这样 deref 返回了我希望通过 * 运算符访问的值的引用。没有 Deref trait 的话，编译器只会解引用 & 引用类型。
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(item: T) -> Self {
            MyBox(item)
        }
    }

    impl<T> __Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    #[test]
    fn tarit_deref_test() {
        let x = 6;
        let y = MyBox::new(x);

        assert_eq!(6, x);
        assert_eq!(6, *y);
        // Rust 将 * 运算符替换为先调用 deref 方法再进行普通解引用的操作，如此我们便不用担心是否还需手动调用 deref 方法了
        assert_eq!(6, *(y.deref()))
    }

    //隐式引用强制转换是 Rust 在函数或方法传参上的一种便利，这仅仅用在实现了 Deref 的 trait，隐式引用强制将这样一个类型转换为另一个类型或者引用。
    //例如，&String 转换为 &str，因为 String 实现了 Deref 返回了 &str。
    fn hello(name: &str) {
        println!("hello: {}", name);
    }

    #[test]
    fn trait_hidden_ref_test() {
        hello("apple");
        hello(&String::from("proto"));
        hello(&MyBox::new("michael"));

        let people = MyBox::new(String::from("hello"));
        // 如果没有隐式引用强制转换，我们就得这样做
        // *people == *(people.deref()) -> String
        // &(*people) -> &String
        // &(*people)[..] -> &str
        hello(&(*people)[..]);
    }

    //类似于如何使用 Deref 重载不可变引用的 * 运算符，Rust 提供了 DerefMut 用于重载可变引用的 * 运算符。
    //Rust 在发现类型和 trait 实现满足三种情况时会自动进行引用强制转换：

    // 当 T: Deref<Target=U> 时从 &T 到 &U；
    // 当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U；
    // 当 T: Deref<Target=U> 时从 &mut T 到 &U；

    //Drop，其允许我们在值要离开作用域时执行一些代码。可以为任何类型提供 Drop 的实现，同时所指定的代码被用于释放类似于文件或网络连接的资源。
    //在 Rust 中，可以指定每当值离开作用域时被执行的代码，编译器会自动插入这些代码。

    //指定在值离开作用域时应该执行的代码的方式是实现 Drop。Drop 要求实现一个叫做 drop 的方法，它获取一个 self 的可变引用。

    //任何程序都不能完全正确地按照开发者的意愿去运行，总会遇到错误，例如打开文件时，文件不存在。Rust 将程序可能出现的错误分为可恢复错误（recoverable）和不可恢复错误（unrecoverable）。
    //可恢复错误通常意味着意料之中的情况，我们可以选择向用户报告错误或者进行重试。不可恢复的错误往往意味着 bug，比如数组访问越界。

    //Rust 中没有异常，如果遇到可恢复错误就返回 Result<T, E> 让开发者处理，遇到不可恢复的错误就 panic!。

    //当程序 panic 时，程序默认会开始展开（unwinding），这意味着 Rust 会回溯栈并清理它遇到的每一个函数的数据，不过这个回溯并清理的过程有很多工作。另一种选择是直接终止（abort），
    //这会不清理数据就退出程序，那么程序所使用的内存需要由操作系统来清理。如果你需要项目的最终二进制文件越小越好，panic 时通过在 Cargo.toml 的 [profile] 部分增加 panic = 'abort'，
    //可以由展开切换为终止。例如，如果你想要在 release 模式中 panic 时直接终止：
    //[profile.release]
    //panic = 'abort'
    //我们可以通过将 RUST_BACKTRACE 设置为一个非 0 的数值，用于在程序 panic 时得到程序的调用栈。

    //*可恢复错误*
    //程序往往不会严重到不能执行，在出现异常情况时，返回一个错误大多是比较合适的处理方式。Rust 中经常通过枚举类型 Result 代表返回一个错误或者一个期望的值。如下面 Result 的定义所示，
    //它被定义为一个泛型，在处理正确时返回 Ok(T)，出现错误时返回错误 Err(E)

    //我们来看一个打开文件的例子，目的是获得操作文件的句柄，在文件不存在时，我们创建新的文件，如果都失败或者其他未知错误，直接 panic：
    #[test]
    fn error_panic_test() {
        let file_name = "hello.txt";
        let f = File::open(file_name);
        let f = match f {
            Ok(file) => file,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => match File::create(file_name) {
                    Ok(fc) => fc,
                    Err(error) => panic!("create file failed: {:?}", error),
                },
                other_error => panic!("open file failed: {:?}", other_error),
            },
        };
    }

    //看着上面层层嵌套的 match，在感叹其强大的匹配功能的同时，也会感慨较深的代码嵌套不易阅读，我们尝试对其进行简化，其中 unwrap_or_else 接受一个闭包，它在前面的返回值没有问题时，
    //直接返回；当遇到错误时，调用我们传入的闭包继续处理，期望返回我们需要的类型。
    #[test]
    fn error_panic_2_test() {
        let file_name = "hello.txt";
        let f = File::open(file_name).unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create(file_name).unwrap_or_else(|error| {
                    panic!("{:?}", error);
                })
            } else {
                panic!("meet unknown error: {:?}", error);
            }
        });
    }

    //看到的是，我们使用 match 完成错误匹配，选择继续执行还是返回。但也展现出语法繁琐，所以就有了 ? 运算符。? 在遇到返回值 OK(value)，将取出 value 继续执行，
    //如果遇到 Err，将会返回当前的错误。我们来改写上面的例子：
    fn read_file_content(file: &str) -> Result<String, Error> {
        let mut content = String::new();
        File::open(file)?.read_to_string(&mut content)?;
        Ok(content)
    }

    //match 表达式与问号运算符所做的有一点不同：? 运算符所使用的错误值被传递给了 from 函数，它定义于标准库的 From trait 中，其用来将错误从一种类型转换为另一种类型。
    //当 ? 运算符调用 from 函数时，收到的错误类型被转换为由当前函数返回类型所指定的错误类型。这在当函数返回单个错误类型来代表所有可能失败的方式时很有用，即使其可能会因很多种原因失败。
    //只要每一个错误类型都实现了 from 函数来定义如何将自身转换为返回的错误类型，? 运算符会自动处理这些转换。`总结就是，? 将收集到错误值自动转换为要返回的错误类型`。

    //另外，由于 main 函数是比较特殊的，它返回什么类型是由限制的，一般情况下它的返回值是 ()，但是为了方便，他也允许返回 Result<(), E>，因此，我们也可以在 main 中使用 ?：

    //* ?运算符 *
    // 除了可以用于 Result 类型之外，还可以用于 Option 类型。如果 x 是 Option，那么若 x 是 Some ，对 x? 表达式求值将返回底层值，否则无论函数是否正在执行都将终止且返回 None 。

    #[derive(Debug, Copy, Clone)]
    struct Person {
        job: Option<Job>,
    }

    impl Person {
        fn work_phone_area_code(&self) -> Option<u8> {
            self.job?.phone_number?.area_code
        }
    }

    #[derive(Debug, Copy, Clone)]
    struct Job {
        phone_number: Option<PhoneNumber>,
    }

    #[derive(Debug, Copy, Clone)]
    struct PhoneNumber {
        area_code: Option<u8>,
        number: i32,
    }

    #[test]
    fn error_panic_3_test() {
        let p = Person {
            job: Some(Job {
                phone_number: Some(PhoneNumber {
                    area_code: Some(128),
                    number: 439222222,
                }),
            }),
        };

        assert_eq!(p.work_phone_area_code(), Some(128));
    }

    //* Option *
    //Option 自己实现了很多有用的方法，可以更快速的完成我们的代码编写。
    #[derive(Debug)]
    enum Food {
        Apple,
        Carrot,
        Potato,
    }

    #[derive(Debug)]
    struct Peeled(Food);
    #[derive(Debug)]
    struct Chopped(Food);
    #[derive(Debug)]
    struct Cooked(Food);

    fn cook(food: Option<Food>) -> Option<Cooked> {
        food.map(|f| Peeled(f))
            .map(|Peeled(f)| Chopped(f))
            .map(|Chopped(f)| Cooked(f))
    }

    fn eat(food: Option<Cooked>) {
        match food {
            Some(f) => println!("Mmm, I love {:?}", f),
            None => println!("Oh no! It wasn't edible"),
        }
    }

    #[test]
    fn option_test() {
        let apple = Some(Food::Apple);
        let carrot = Some(Food::Carrot);
        let potato = None;

        eat(cook(apple));
        eat(cook(carrot));
        eat(cook(potato));
    }

    //and_then 当 Option 是 None 时，返回 None。否则将 Some 中包裹的值传入闭包函数，这个闭包返回一个新的 Option。
    fn sq(x: u32) -> Option<u32> {
        Some(x * x)
    }

    fn nope(_: u32) -> Option<u32> {
        None
    }

    #[test]
    fn option_2_test() {
        assert_eq!(Some(2).and_then(sq).and_then(sq), Some(16));
        assert_eq!(Some(2).and_then(sq).and_then(nope), None);
        assert_eq!(Some(2).and_then(nope).and_then(sq), None);
        assert_eq!(None.and_then(sq).and_then(sq), None);
    }

    //* 定义错误类型 *
    //定义自己的错误类型在传递错误信息时是必要的，我们来看一个例子，将一个字符串数组中的第一个元素转换为数字并且乘以 2。下面的代码中我们也定义了自己的 Result 类型，
    //定义自己的错误类型需要实现 Error。
    #[derive(Debug, Clone)]
    struct DoubleFirstError;

    type Results<T> = std::result::Result<T, DoubleFirstError>;

    impl Display for DoubleFirstError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "double first int error")
        }
    }

    impl std::error::Error for DoubleFirstError {}

    fn double_first(vec: &Vec<&str>) -> Results<i32> {
        vec.first().ok_or(DoubleFirstError).and_then(|s| {
            s.parse::<i32>()
                .map_err(|_| DoubleFirstError)
                .map(|i| i * 2)
        })
    }

    fn print(result: Results<i32>) {
        match result {
            Ok(r) => println!("the result is {}", r),
            Err(e) => println!("error is {}", e),
        }
    }

    #[test]
    fn error_test() {
        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        print(double_first(&numbers));
        print(double_first(&empty));
        print(double_first(&strings));
    }

    //* Box<error::Error> */
    //当我们只关注错误信息，而不关注错误类型的时候，我们可以将错误装进 Box，我们对上面的例子稍加修改：
    //type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

    //* 类型转换 */
    //Rust 使用 trait 解决类型之间的转换问题。最一般的转换会用到 From 和 Into 两个 trait。
    //From 定义怎么根据另一种类型生成自己，而在定义 From 之后，我们就自然的获得了 Into，因为它就是 From 倒过来，但是在使用 Into 的时候，我们得指明要转换的类型。
    #[derive(Debug, Clone, Copy)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(value: i32) -> Self {
            Number { value }
        }
    }

    #[test]
    fn from_test() {
        let number = Number::from(128);
        println!("{:?}", number);
        let number: Number = 166i32.into();
        println!("{:?}", number);
    }

    //* TryFrom、TryInto */
    //类似于 From 和 Into，不过 TryFrom 和 TryInto 用于易出错的转换，他们的返回值类型是 Result 类型。
    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();
        fn try_from(item: i32) -> Result<Self, Self::Error> {
            if item % 2 == 0 {
                Ok(EvenNumber(item))
            } else {
                Err(())
            }
        }
    }

    #[test]
    fn try_from_test() {
        // try from
        assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
        assert_eq!(EvenNumber::try_from(5), Err(()));

        let result: Result<EvenNumber, ()> = 8i32.try_into();
        assert_eq!(Ok(EvenNumber(8)), result);
        let result: Result<EvenNumber, ()> = 5i32.try_into();
        assert_eq!(Err(()), result);
    }

    //* ToString、FromStr */
    //在我们需要将类型转换成字符串类型时，我们只需实现 ToString，但是最好的是实现 fmt::Display，它会自动提供 to_string() 方法。
    //另外，我们也经常需要将字符串转换成我们需要的目标类型，只要目标类型实现了 FromStr，我们就可以使用字符串的 parse 方法解析，不过我们得提供要转换到的目标类型，或者使用涡轮鱼（turbo fish）语法。
    struct Circle {
        radius: i32,
    }

    impl Display for Circle {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Circle({})", self.radius)
        }
    }

    #[test]
    fn to_string_test() {
        let circle = Circle { radius: 23 };
        println!("{}", circle);
        let num: i32 = "45".parse().unwrap();
        let num_2 = "25".parse::<i32>().unwrap();
        println!("num= {}, num_2= {}", num, num_2);
    }

    //* 泛型 */
    //泛型可以极大地降低代码重复度，我们可以定义泛型结构体，泛型函数，泛型方法，泛型枚举等。但是我们不用担心泛型的性能，Rust 通过在编译时进行泛型代码的单态化 (monomorphization) 来保证效率。
    //单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。

    //*泛型枚举我们最常见的应该是：Option 和 Result。*/
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    //* 结构体和方法 */
    struct Pointss<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Pointss<T, U> {
        fn mixup<V, W>(self, other: Pointss<V, W>) -> Pointss<T, W> {
            Pointss {
                x: self.x,
                y: other.y,
            }
        }
    }

    #[test]
    fn generic_1_test() {
        let p1 = Pointss { x: 12, y: 23 };
        let p2 = Pointss { x: "Hello", y: 'x' };
        let p3 = p1.mixup(p2);

        println!("p3.x= {}, p3.y= {}", p3.x, p3.y);
    }

    //* 函数 */
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    #[test]
    fn generic_2_test() {
        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];
        let result = largest(&char_list);
        println!("The largest char is {}", result);
    }

    //* trait */
    trait Area<T> {
        fn area(&self) -> T;
    }

    struct Rectangles<T> {
        width: T,
        height: T,
    }

    impl<T: Copy + Mul<Output = T>> Area<T> for Rectangles<T> {
        fn area(&self) -> T {
            self.width * self.height
        }
    }

    #[test]
    fn generic_3_test() {
        let rec = Rectangles {
            width: 4.5,
            height: 7.9,
        };
        println!("area: {}", rec.area());
    }

    //* 智能指针 */
    //指针是一个包含内存地址变量的通用概念，rust 中使用 & 或者 ref 引用一个变量。智能指针是一类数据结构，他们的表现类似指针，但是也拥有额外的元数据和功能。
    //在 Rust 中，普通引用和智能指针的一个额外的区别是引用是一类只借用数据的指针；相反，在大部分情况下，智能指针拥有他们指向的数据。

    //* Box 指向堆上的数据 */
    //Box<T> 将数据存储在堆上，留在栈上的仅仅是数据的指针，除此之外，box 没有性能损失。它们多用于如下场景：
    // *当在编译时不确定类型大小，又想在需要确切大小的上下文中使用时，例如，使用 Box<dyn error:Error> 动态分发；
    // *当有大量数据并希望在转移所有权的时候，不发生数据拷贝；
    // *当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候；

    //如下示例，定义了变量 b，其值是一个指向被分配在堆上的值 5 的 Box。我们可以像数据是储存在栈上的那样访问 box 中的数据，正如任何拥有数据所有权的值那样，当像 b 这样的 box 在 main 的末尾离开作用域时，它将被释放。
    //let b = Box::new(5);

    //Rust 需要在编译时知道类型占用多少空间。一种无法在编译时知道大小的类型是 递归类型（recursive type），其值的一部分可以是相同类型的另一个值。我们探索一下 cons list，
    //一个函数式编程语言中的常见类型，来展示这个（递归类型）概念。

    //cons list 的每一项都包含两个元素：当前项的值和下一项。其最后一项值包含一个叫做 Nil 的值且没有下一项。cons list 通过递归调用 cons 函数产生。代表递归的终止条件（base case）的规范名称是 Nil，它宣布列表的终止。
    //另外编译器还提醒我们，不能直接存储一个值，而是应该存储一个指向这个值的指针，还提示我们应该用 Box<List>：
    //help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to make `List` representable

    //因为 Box<T> 是一个指针，我们总是知道它需要多少空间：指针的大小并不会根据其指向的数据量而改变，我们对上面的程序做出修改：
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    #[test]
    fn pointer_test() {
        use List::{Cons, Nil};
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        println!("{:?}", list);
    }

    //* Rc 引用计数  */
    //大部分情况下所有权是非常明确的：可以准确地知道哪个变量拥有某个值。然而，有些情况单个值可能会有多个所有者。例如，在图数据结构中，多个边可能指向相同的节点，
    //而这个节点从概念上讲为所有指向它的边所拥有。节点直到没有任何边指向它之前都不应该被清理。

    //Rc<T> 用于当我们希望在堆上分配一些内存供程序的多个部分读取，而且无法在编译时确定程序的哪一部分会最后结束使用它的时候。如果确实知道哪部分是最后一个结束使用的话，
    //就可以令其成为数据的所有者，正常的所有权规则就可以在编译时生效。 Rc<T> 只能用于单线程场景.

    //* 使用 Rc<T> 共享数据 */
    //我们修改 List 的定义为使用 Rc<T> 代替 Box<T>，现在每一个 Cons 变量都包含一个值和一个指向 List 的 Rc<T>。当创建 b 时，不同于获取 a 的所有权，这里会克隆 a 所包含的 Rc<List>，
    //这会将引用计数从 1 增加到 2 并允许 a 和 b 共享 Rc<List> 中数据的所有权。创建 c 时也会克隆 a，这会将引用计数从 2 增加为 3。每次调用 Rc::clone，Rc<List> 中数据的引用计数都会增加，
    //直到有零个引用之前其数据都不会被清理。
    #[derive(Debug)]
    enum Lists {
        Cons(i32, Rc<Lists>),
        Nil,
    }

    #[test]
    fn pointer_rc_list_test() {
        let a = Rc::new(Lists::Cons(
            5,
            Rc::new(Lists::Cons(10, Rc::new(Lists::Nil))),
        ));
        let b = Rc::new(Lists::Cons(3, Rc::clone(&a)));
        let c = Rc::new(Lists::Cons(4, Rc::clone(&a)));

        println!("a: {:?}", a);
        println!("b: {:?}", b);
        println!("c: {:?}", c);
    }

    //* Rc::strong_count */
    //可以使用 Rc::strong_count 查看 Rc<T> 的引用计数值。
    #[test]
    fn pointer_rc_strong_count_test() {
        let a = Rc::new(Lists::Cons(5,Rc::new(Lists::Cons(10, Rc::new(Lists::Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let b = Lists::Cons(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let c = Lists::Cons(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }

    //* RefCell */
    //内部可变性（Interior mutability）是 Rust 中的一个设计模式，它允许你即使在有不可变引用时也可以改变数据，这通常是借用规则所不允许的。不同于 Rc<T>，RefCell<T> 代表其数据的唯一的所有权。
    //我们之前学习的借用规则是这样的：
    //*在任意给定时刻，只能拥有一个可变引用或任意数量的不可变引用之一（而不是两者）。
    //*引用必须总是有效的。

    //对于引用和 Box<T>，借用规则的不可变性作用于编译时。对于 RefCell<T>，这些不可变性作用于运行时。对于引用，如果违反这些规则，会得到一个编译错误。而对于 RefCell<T>，如果违反这些规则程序会 panic 并退出。

    //在编译时检查借用规则的优势是这些错误将在开发过程的早期被捕获，同时对运行时没有性能影响，因为所有的分析都提前完成了。为此，在编译时检查借用规则是大部分情况的最佳选择，这也正是其为何是 Rust 的默认行为。
    //相反在运行时检查借用规则的好处则是允许出现特定内存安全的场景，而它们在编译时检查中是不允许的。静态分析，正如 Rust 编译器，是天生保守的。

    //因为一些分析是不可能的，如果 Rust 编译器不能通过所有权规则编译，它可能会拒绝一个正确的程序；从这种角度考虑它是保守的。如果 Rust 接受不正确的程序，那么用户也就不会相信 Rust 所做的保证了。
    //然而，如果 Rust 拒绝正确的程序，虽然会给程序员带来不便，但不会带来灾难。RefCell<T> 正是用于当你确信代码遵守借用规则，而编译器不能理解和确定的时候。

    //如下为选择 Box<T>，Rc<T> 或 RefCell<T> 的理由：
    //* Rc<T> 允许相同数据有多个所有者；Box<T> 和 RefCell<T> 有单一所有者。
    //* Box<T> 允许在编译时执行不可变或可变借用检查；Rc<T> 仅允许在编译时执行不可变借用检查；RefCell<T> 允许在运行时执行不可变或可变借用检查。
    //* 因为 RefCell<T> 允许在运行时执行可变借用检查，所以我们可以在即便 RefCell<T> 自身是不可变的情况下修改其内部的值。

    //RefCell<T> 只能用于单线程场景.

    //来看一个例子，我们定义了 Messenger 用于发送消息，真实场景可能是发送短信或者发送邮件，注意它的 receiver 是 &ref；然后我们定义结构体 LimitTracker，
    //它用来实现我们的业务功能，当调用它的 set_value 方法时，根据业务逻辑发送不同的消息。
    trait Messager {
        fn send(&self, msg: &str);
    }

    struct LimitTracker<'a, T: Messager> {
        messager: &'a T,
        value: usize,
        max: usize,
    }

    impl <'a, T> LimitTracker<'a, T> where T: Messager{

        pub fn new(messager: &T, max: usize) -> LimitTracker<T>{
            LimitTracker { messager, value: 0,  max }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max > 1.0 {
                self.messager.send("Error: you are over your quota!");
            }else if percentage_of_max >= 0.9 {
                self.messager.send("Urgent warning: You've used up over 90% of your quota!");
            }else if  percentage_of_max >= 0.75 {
                self.messager.send("Warning: You've used up over 75% of your quota!");
            }
        }
    }

    //现在我们对 LimitTracker 的功能进行测试，但是肯定不能真正实现 Messenger，所以需要对其打桩，计划是对其进行 Mock，记录发送的消息
    //我们使用 borrow_mut 和 borrow 分别在运行时进行可变借用和不可变借用：
    struct MockMessager {
        send_message: RefCell<Vec<String>>,
    }

    impl MockMessager {
        fn new() -> MockMessager {
            MockMessager { send_message: RefCell::new(vec![]) }
        }
    }

    impl Messager for MockMessager {

        fn send(&self, msg: &str) {
            //可变借用 borrow_mut
            self.send_message.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messager = MockMessager::new();
        let mut limie_tracker = LimitTracker::new(&mock_messager, 100);

        limie_tracker.set_value(80);

        //borrow, 不可变借用
        assert_eq!(mock_messager.send_message.borrow().len(), 1);
    }

    //* 结合 Rc<T> 和 RcCell<T> */
    //Rc<T> 通过引用计数的方式可以让一个值有多个所有者，RcCell<T> 可以在运行时获取值的可变引用对其修改。下面的例子中，通过对 value 的修改，a，b，c 都改了。
    #[derive(Debug)]
    enum RCList {
        Cons(Rc<RefCell<i32>>, Rc<RCList>),
        Nil,
    }

    #[test]
    fn rc_refcell_test() {
        use RCList::{Cons, Nil};
        let value = Rc::new(RefCell::new(5));
        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
        let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a is {:?}", a);
        println!("b is {:?}", b);
        println!("c is {:?}", c);
    }

    //rust生命周期
}
