use std::collections::HashMap;
type Table = HashMap<String, Vec<String>>;

fn show(table: Table) {
    for (artist, works) in table {
        println!("works by {}", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

fn shows(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

fn mut_shows(table: &mut Table) {
    for (_artis, works) in table {
        works.sort();
    }
}

#[cfg(test)]
mod tests {
    //在 Rust 中，指针按是否有所有权属性可以分为两类，例如 Box<T>，String，或者 Vec 具有所有权属性的指针（owning pointers），可以说它们拥有指向的内存，当它们被删除时，指向的内存也会被被释放掉。
    //但是，也有一种非所有权指针，叫做引用 (references)，它们的存在不会影响指向值的生命周期，在 Rust 中创建引用的行为称之为对值的借用。

    //要注意的是，引用决不能超过其引用的值的生命周期。必须在代码中明确指出，任何引用都不可能超过它所指向的值的寿命。为了强调这一点，Rust 将创建对某个值的引用称为借用：你所借的东西，最终必须归还给它的所有者。

    //* 引用值 */
    //在《【Rust】所有权》章节中，我们说到函数传值会转移值得所有权，for 循环也会，例如，对下面的代码，我们在将 table 传递给 show 函数之后，table 就处于未初始化状态：

    use std::{
        borrow::Cow,
        collections::BTreeSet,
        fmt::{Debug, Display},
        iter::from_fn,
        marker::PhantomData,
        ops::{Add, AddAssign},
        sync::{
            atomic::{self, AtomicU64},
            Arc, Mutex,
        },
        thread,
        time::Duration,
        vec,
    };

    use lazy_static::__Deref;
    use rand::random;

    use super::*;

    #[test]
    fn references_value_test() {
        let mut table = Table::new();
        table.insert(
            "Gesoldo".to_string(),
            vec![
                "many madrigals".to_string(),
                "Tenebrae Responsoria".to_string(),
            ],
        );
        table.insert(
            "Caravaggio".to_string(),
            vec![
                "The Musicians".to_string(),
                "The Calling of St. Matthew".to_string(),
            ],
        );
        table.insert(
            "Cellini".to_string(),
            vec![
                "Perseus with the head of Medusa".to_string(),
                "a salt cellar".to_string(),
            ],
        );
        show(table);
        //assert_eq!(table["Gesualdo"][0], "many madrigals"); Rust 编译器提示变量 table 已经不可用，show 函数的调用已经转移 table 的所有权：
    }

    //正确处理这个问题的方法是使用引用，使用引用不会改变值得所有者，引用有两种类型：
    //.shared reference：可以读引用的值，但不能改变它。而且同时可以有多个 shared reference。表达式 &e 会生成 e 的 shared reference。如果 e 的类型是 T，那么 &e 的类型是 &T，
    //读作 ref T，shared reference 是可以复制的；共享引用借用的值是只读的，在共享引用的整个生命周期中，它的引用对象或从该引用对象可到达的任何东西都不能被改变，就像加了读锁，被冻结了；
    //.mutable reference：可读可写所引用的值，不能拥有其他任何 shared reference 或者 mutable reference。表达式 &mut e 生成 e 的 mutable reference。如果 e 的类型是 T，
    //那么 &mut e 的类型是 &mut T，读作 ref mute T。 mutable reference 是不可以复制的。可变引用借用的值只能通过该引用访问，在可变引用的整个生命周期中，没有其他可用路径可以到达其引用对象；

    //因此，我们可以对上面的 show 函数作如下修改，就可以使得代码编译通过。在 show 函数中，table 的类型是 &Table，那么 artist 和 works 的类型就是 &String 和 &Vec<String>，内部的 for 循环中 work 的类型也就变成了 &String。
    #[test]
    fn references_value_2_test() {
        let mut table = Table::new();
        table.insert(
            "Gesualdo".to_string(),
            vec![
                "many madrigals".to_string(),
                "Tenebrae Responsoria".to_string(),
            ],
        );
        table.insert(
            "Caravaggio".to_string(),
            vec![
                "The Musicians".to_string(),
                "The Calling of St. Matthew".to_string(),
            ],
        );
        table.insert(
            "Cellini".to_string(),
            vec![
                "Perseus with the head of Medusa".to_string(),
                "a salt cellar".to_string(),
            ],
        );
        shows(&table);
        assert_eq!(table["Gesualdo"][0], "many madrigals");
    }

    //现在，如果我们 table 中的值进行排序，shared reference 肯定不能满足要求，因为它不允许改变值，所以我们需要一个 mutable reference。可变借用使得 sort_works 有能力读和修改 works。
    #[test]
    fn references_value_3_test() {
        let mut table = Table::new();
        table.insert(
            "Gesualdo".to_string(),
            vec![
                "many madrigals".to_string(),
                "Tenebrae Responsoria".to_string(),
            ],
        );
        table.insert(
            "Caravaggio".to_string(),
            vec![
                "The Musicians".to_string(),
                "The Calling of St. Matthew".to_string(),
            ],
        );
        table.insert(
            "Cellini".to_string(),
            vec![
                "Perseus with the head of Medusa".to_string(),
                "a salt cellar".to_string(),
            ],
        );
        mut_shows(&mut table);
        assert_eq!(table["Gesualdo"][0], "Tenebrae Responsoria");
    }
    //当我们将一个值传递给函数时，可以说是将值的所有权转移给了函数，称之为按值传参。但是，如果我们将引用传给函数，我们可以称之为按引用传参，它没有改变值的所有权，只是借用了值。

    //* 解引用 */
    //在 Rust 中，我们可以通过 & 或者 &mut 创建 shared reference 或者 mutable reference，在机器级别，它们就是个地址。解引用可以通过 * 操作符。
    #[test]
    fn dereference_test() {
        // Back to Rust code from this point onward
        let x = 10;
        let r = &x; // x is a shared reference to x
        assert!(*r == 10); // explictly dereference r

        let mut y = 32;
        let m = &mut y; // &mut y is mutable reference to y
        *m += 32; // explicitly deference m to set y's value
        assert!(*m == 64); // and to see y's new value
    }

    //如果每次访问引用指向的值，都需要 * 操作符，在访问结构体字段的时候，不难想象，体验有点糟糕。所在，在 Rust 中，可以通过 . 操作符隐式地解引用它的左操作数。
    #[test]
    fn deference_2_test() {
        struct Anime {
            name: &'static str,
            bechdel_pass: bool,
        };
        let aira = Anime {
            name: "Aria: The Animation",
            bechdel_pass: true,
        };
        let anime_ref = &aira;
        assert_eq!(anime_ref.name, "Aria: The Animation");
        // Equivalent to the above, but with the dereference written out:
        assert_eq!((*anime_ref).name, "Aria: The Animation");
    }

    //除此之外，. 操作符还可以隐式地从它的左操作数创建引用，因此下面两个操作使等价的：
    #[test]
    fn deference_3_test() {
        let mut v = vec![1973, 1968];
        v.sort(); // implicitly borrows  mutable reference to v
        (&mut v).sort(); // equivalent, but more verbose
    }

    //* 引用更新 */
    //在 C++ 中，一旦一个引用被初始化，是不能更改其指向的。但是在 Rust 中是完全允许的，例如下面的代码中，一开始 r 借用了 x，后面又借用了 y：
    #[test]
    fn reference_update_test() {
        let x = 10;
        let y = 20;
        let mut r = &x;
        assert_eq!(*r, 10);
        r = &y;
        assert_eq!(*r, 20);
    }

    //* 引用的引用 */
    //在 C 语言中我们经常听到指向指针的指针，在 Rust 中也是允许的，如下所示，为了清晰，我们写出了每个变量的类型，实际上我们完全可以省略，由 Rust 来推断。
    #[test]
    fn reference_of_ref_test() {
        struct Point {
            x: i32,
            y: i32,
        }
        let point = Point { x: 10000, y: 2000 };
        let r: &Point = &point;
        let rr: &&Point = &r;
        let rrr: &&&Point = &rr;
        assert_eq!(rrr.y, 2000);
    }
    //然而，. 操作符可以一直向前寻找，直到找到最终的值。这些变量在内存中的分布如下图所示：

    //* 引用比较 */
    //同 . 操作符一样，比较运算符也有这样的效果，能连续解引用直到找到最终的值，例如：
    #[test]
    fn reference_compare_test() {
        let x = 10;
        let y = 10;
        let rx = &x;
        let ry = &y;
        let rrx = &rx;
        let rry = &ry;
        assert!(rrx <= rry);
        assert_eq!(rrx, rry);
        //这在大多数情况下应该是我们想要的效果，但是如果我们确实想知道两个引用它们指向的内存地址是否相同，我们可以使用 std::ptr::eq，仅仅比较地址而不是指向的值：
        assert_eq!(rx, ry);
        assert!(!std::ptr::eq(rx, ry));
        //但是，无论如何，比较操作符左右两侧的操作数必须要有相同的类型，例如，下面的代码编译失败：
        //assert!(rx == rrx); // error: type mismatch: `&i32` vs `&&i32`
        //assert!(rx == *rrx); // this is okay
    }

    //* 引用永不为空 */
    //Rust 中的引用永远不会为空。没有类似于 C 的 NULL 或 C++ 的 nullptr。引用没有默认初始值（因为任何变量在初始化之前，无论其类型如何，都不能使用），Rust 不会将整数转换为引用（安全代码中），因此无法将 0 转换为引用。

    //C 和 C++ 代码中使用空指针表示没有值，例如，malloc 函数要么返回一个指向内存块的指针，要么返回 null 表示内存申请失败。

    //在 Rust 中，如果你需要用一个值表示引用某个变量的内存，或者没有，可以使用 Option<&T>。在机器层面，Rust 将其表示为代表空指针的 None 或者 Some(r)，其中 r 是 &T 值，表示为非零地址，因此 Option<&T>
    //与 C 或 C++ 中的可空指针一样有效，但是它更安全：Option 类型要求在使用它之前检查它是否为 None。

    //* 从任何表达式借用引用 */
    //在 C、C++ 或者其他大多数语言中，我们都是从变量获取引用，也就是 & 运算符后面一般都是紧跟某个变量。但是在 Rust 中，我们可以从任何表达式借用引用：
    #[test]
    fn get_reference_test() {
        fn factorial(n: usize) -> usize {
            (1..n + 1).product()
        }
        let r = &factorial(6);
        // Arithmetic operators can see through one level of references.
        assert_eq!(r + &1009, 1729);
    }
    //这种情况下，Rust 会创建一个持有表达式值的匿名变量，然后再从匿名变量创建一个引用。匿名表达式的生命周期取决于我们怎么使用这个引用：

    //如果我们是将这个引用用在赋值语句 let，结构体字段或者数组中，那么这个匿名变量的生命周期和我们 let 语句初始化的变量一样，例如上面的 r；

    //否则，这个匿名变量在当前语句结束就会被释放掉，例如上面为 1009 创建的匿名变量在 assert_eq! 结束就会被丢掉；

    //* 胖指针 */
    //胖指针，即 fat pointers，指哪些不仅仅是包含了地址的指针，就像 &[T]，引用自 slice 的指针除了包含首元素的地址之外，还包括 slice 的数量；另一种胖指针是 trait 类型，详细请看 Trait 对象。

    //* 引用安全性 */
    //截止到目前为止，我们看到的指针都和 C 中差不多，但是既然这样，我们又如何保证安全性呢？为了保证引用使用的安全性，Rust 为每个应用都会分配一个生命周期，更多请看【Rust】生命周期。
    //* 引用局部变量 */
    //如果我们引用的是一个局部变量，并且我们的引用比局部变量的作用域更大，也就是局部变量释放了之后，我们的程序会如何，编译器提示：我们引用的值没有引用活得久，因为 x 在内部的括号之后就被释放了，导致 r 成了一个悬垂指针：

    //Rust 编译器是如何确保每个引用都是有效的呢？ Rust 为每个引用都赋予了一个满足其使用范围的 生命周期。生命周期是程序的一部分，可以被安全地用于语句，表达式或者变量。但是生命周期完全是 Rust 编译时虚构的。在运行时，
    //引用只不过是一个地址，其生命周期是其类型的一部分，没有运行时表示。

    //在上面的例子中，有三个生命周期，变量 x 和 r 的生命周期是从它们初始化到编译器认为它们不再使用为止。第三个生命周期是一个引用类型，我们引用自 x 并且存储在 r 中。

    //正如我们上面看到的，生命周期有一个很明显的约束，就是它不能比它引用的值活的久。因为如果这里 x 出了内部的括号，就会被释放，所有来自于它的引用都会变成一个悬垂指针，所以，Rust 规定
    //约束 1：值的生命周期必须大于它的引用的生命周期，上面的示例中，x 的生命周期就小于它的引用的生命周期：还有另外一个约束，约束 2：如果我们将引用存储在一个变量中，那么这个引用必须要覆盖这个变量的整个生命周期，
    //从它的初始化到最后一次使用为止。上面示例中，x 引用的生命周期没有覆盖到 r 的使用范围：

    //第一个约束限制了生命周期的上限，也就是它最大是多大；第二个约束限制了它的下限，也就是它最小应该是多少；Rust 的编译器必须能找到一个能满足所有约束的生命周期，也就是从上限开始到下限为止。然而遗憾的是，
    //我们的示例中，没有这样的生命周期，所以编译失败：

    //* 更新全局引用变量 */
    //当我们传递一个引用给函数时，Rust 如何保证安全使用呢？假设我们有一个函数 f，接受一个引用作为参数，并且把它存储在全局变量中，例如：
    // 不能编译
    // static mut STASH: &i32;

    // fn f(p:&i32){
    //     STASH=p;
    // }
    //Rust 的全局变量时静态创建的，贯穿应用程序的整个生命周期。像任何其他声明一样，Rust 的模块系统控制静态变量在什么地方可见，所以它们仅仅是在生命周期里是全局的，而不是可见性。上面的代码是有一些问题的，没有遵循两个规则：
    //所有的静态变量必须被初始化；可变的静态变量不是线程安全的，因为任何线程任何时候都可以访问静态变量，即使单线程也会引发某些未知的异常；出于这些原因，我们需要放在 unsafe 块中才能访问全局可变静态变量；
    //根据这两个规则，我们将上面的代码改成下面这个样子：
    // static mut STASH: &i32 = &128;

    // fn f(p: &i32) {
    //     unsafe {
    //         STASH = p;
    //     }
    // }
    //为了让代码更加完善，我们需要手动函数参数的生命周期，这里 'a 读作 tick A，我们将 <'a> 读作 for any lifetime 'a。所以下面的代码定义了一个接受具有任意生命周期 'a 参数 p 的函数 f：
    // fn f<'a>(p: &'a i32) { ... }
    //由于 STASH 的生命周期和应用程序一样，所以我们必须赋予它一个具有相同生命周期的引用，Rust 将这种生命周期称之为 'static lifetime，静态生命周期，所以如果参数的 p 的声明是 'a，是不允许的。编译器直接拒绝编译我们的代码：

    //编译器的提示很明显，f 需要一个具有静态生命周期的参数 p，因此我们现在可以将代码修改成如下的样子：

    //从一开始的 f(p: &i32) 到结束时的 f(p: &'static i32)，如果不在函数的签名中反映该意图，我们就无法编写一个将引用固定在全局变量中的函数，我们必须指出引用的生命周期，
    //满足约束 2：如果我们将引用存储在一个变量中，那么这个引用必须要覆盖这个变量的整个生命周期，从它的初始化到最后一次使用为止。

    struct TreeNode<T> {
        element: T,
        left: BinaryTree<T>,
        right: BinaryTree<T>,
    }

    enum BinaryTree<T> {
        Empty,
        NoneEmpty(Box<TreeNode<T>>),
    }
    ///这几行代码定义了一个可以存储任意数量的 T 类型值的 BinaryTree，每个 BinaryTree 要么为空要么不为空。如果是空的，那么什么数据都不包，如果不为空，那么它有一个 Box，包含一个指向堆数据的指针。
    ///每个 TreeNode 值包含一个实际元素，以及另外两个 BinaryTree 值。这意味着树可以包含子树，因此 NonEmpty 树可以有任意数量的后代。BinaryTree<&str> 类型值的示意图如下图所示。
    ///与 Option<Box<T>> 一样，Rust 消除了 tag 字段，因此 BinaryTree 值只是一个机器字。
    impl<T: Ord> BinaryTree<T> {
        fn add(&mut self, value: T) {
            match *self {
                BinaryTree::Empty => {
                    *self = BinaryTree::NoneEmpty(Box::new(TreeNode {
                        element: value,
                        left: BinaryTree::Empty,
                        right: BinaryTree::Empty,
                    }))
                }
                BinaryTree::NoneEmpty(ref mut node) => {
                    if value <= node.element {
                        node.left.add(value);
                    } else {
                        node.right.add(value);
                    }
                }
            }
        }
    }

    #[derive(Debug, Eq, Clone, Copy)]
    struct Complex<T> {
        re: T,
        im: T,
    }

    impl<T> Add for Complex<T>
    where
        T: Add<Output = T>,
    {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            Complex {
                re: self.re + rhs.re,
                im: self.im + rhs.im,
            }
        }
    }

    impl<T> std::ops::Neg for Complex<T>
    where
        T: std::ops::Neg<Output = T>,
    {
        type Output = Complex<T>;
        fn neg(self) -> Complex<T> {
            Complex {
                re: -self.re,
                im: -self.im,
            }
        }
    }

    impl<T> AddAssign for Complex<T>
    where
        T: AddAssign<T>,
    {
        fn add_assign(&mut self, rhs: Complex<T>) {
            self.im += rhs.im;
            self.re += rhs.re;
        }
    }

    impl<T> PartialEq for Complex<T>
    where
        T: PartialEq,
    {
        fn eq(&self, other: &Complex<T>) -> bool {
            self.re == other.re && self.im == other.im
        }
    }

    struct Img<P> {
        width: usize,
        pixel: Vec<P>,
    }

    impl<P: Default + Copy> Img<P> {
        /// Create a new image of the given size.
        fn new(width: usize, height: usize) -> Img<P> {
            Img {
                width,
                pixel: vec![P::default(); width * height],
            }
        }
    }

    impl<P> std::ops::Index<usize> for Img<P> {
        type Output = [P];

        fn index(&self, row: usize) -> &[P] {
            let start = row * self.width;
            &self.pixel[start..start + self.width]
        }
    }

    impl<P> std::ops::IndexMut<usize> for Img<P> {
        fn index_mut(&mut self, row: usize) -> &mut [P] {
            let start: usize = row * self.width;
            &mut self.pixel[start..start + self.width]
        }
    }

    #[test]
    fn operators_overloading_index_test() {
        let image = Img::<u8>::new(10, 10);
        println!("{:?}", &image[5]);
    }

    ///Cow（Clone-on-Write）是 Rust 中一个很有意思且很重要的数据结构。它就像 Option 一样，在返回数据的时候，提供了一种可能：要么返回一个借用的数据（只读），要么返回一个拥有所有权的数据（可写）。
    /// Cow 的合理使用能减少不必要的堆内存分配，例如，我们写一个替换 : 的程序，如果原文字符串中没有包含 :，就返回原来的字符串；如果包含，就替换为空格，返回一个 String：
    fn show_cow(cow: Cow<str>) -> String {
        match cow {
            Cow::Borrowed(v) => format!("Borrowed, {}", v),
            Cow::Owned(v) => format!("Owned, {}", v),
        }
    }

    fn replace_colon(input: &str) -> Cow<str> {
        match input.find(':') {
            None => Cow::Borrowed(input),
            Some(_) => {
                let mut input = input.to_string();
                input = input.replace(':', " ");
                Cow::Owned(input)
            }
        }
    }

    #[test]
    fn cow_test() {
        println!("{}", show_cow(replace_colon("hello world")));
        println!("{}", show_cow(replace_colon("hello:world")));
    }

    #[test]
    fn iterator_test() {
        let v = vec![4, 20, 12, 8, 6];
        let mut iterator = v.iter();
        assert_eq!(iterator.next(), Some(&4));
        assert_eq!(iterator.next(), Some(&20));
        assert_eq!(iterator.next(), Some(&12));
        assert_eq!(iterator.next(), Some(&8));
        assert_eq!(iterator.next(), Some(&6));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn into_iterator_test() {
        let mut favorites = BTreeSet::new();
        favorites.insert("Lucy in the Sky With Diamonds".to_string());
        favorites.insert("Liebesträume No. 3".to_string());
        let mut it = favorites.into_iter();
        assert_eq!(it.next(), Some("Liebesträume No. 3".to_string()));
        assert_eq!(it.next(), Some("Lucy in the Sky With Diamonds".to_string()));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn from_fn_test() {
        let lengths: Vec<f64> = from_fn(|| Some((random::<f64>() - random::<f64>()).abs()))
            .take(1000)
            .collect();
        println!("{:?}", lengths);
    }
    //由于这个迭代器永远返回 Some(f64)，所以它永远不会结束，但是我们通过 take(1000) 只取了前 1000 个值。

    fn fibonacci() -> impl Iterator<Item = usize> {
        let mut state = (0, 1);
        std::iter::from_fn(move || {
            state = (state.1, state.0 + state.1);
            Some(state.0)
        })
    }

    #[test]
    fn fibonacci_test() {
        println!("{:?}", fibonacci().take(10).collect::<Vec<usize>>());
    }

    ///许多集合类型提供了一个 drain 方法，该方法需要获取集合的可变引用，将对应区间的值从原来的集合中删掉，并且将删除的值以一个新的迭代器返回：
    #[test]
    fn drain_test() {
        let mut outer = "Earth".to_string();
        let inner = String::from_iter(outer.drain(1..4));
        assert_eq!(outer, "Eh");
        assert_eq!(inner, "art");
    }

    #[test]
    fn iterator_adapter_test() {
        let text = " ponies \n giraffes\niguanas \nsquid".to_string();
        let v: Vec<&str> = text
            .lines()
            .map(str::trim)
            .filter(|s| *s != "iguanas")
            .collect();
        assert_eq!(v, ["ponies", "giraffes", "squid"]);
    }

    /// std::marker::PhantomData 是一个零大小的类型，用于标记一些类型，这些类型看起来拥有类型 T，但实际上并没有：
    /// Rust 并不希望在定义类型时，出现目前还没使用，但未来会被使用的泛型参数，例如未使用的生命周期参数以及未使用的类型。

    #[derive(Debug, PartialEq, Eq, Default)]
    struct Identifier<T> {
        inner: u64,
        phantom: PhantomData<T>,
    }

    #[derive(Debug, PartialEq, Eq, Default)]
    struct User {
        id: Identifier<Self>,
    }

    #[derive(Debug, PartialEq, Eq, Default)]
    struct Product {
        id: Identifier<Self>,
    }

    /// Identifier 中 phantom 字段的引入让 Identifier 在使用时具有了不同的静态类型，但 Identifier 中又实际没有使用类型 T。
    #[test]
    fn id_should_not_be_the_same() {
        let user = User::default();
        let product = Product::default();
        // assert_ne!(user.id, product.id)
        assert_eq!(user.id.inner, product.id.inner);
    }

    /// 我们可以使用泛型结构体来实现对同一种类对象不同子类对象的区分，例如，我们的系统中要设计这样一个功能，将用户分为免费用户和付费用户，而且免费用户在体验免费功能之后，
    /// 如果想升级成付费用户也是可以的。按照我们常规的思维，可能是定义两个结构体 FreeCustomer 以及 PaidCustomer，但是我们可以通过泛型结构体来实现，例如：
    ///
    /// struct Customer<T> {
    ///   id: u64,
    ///   name: String,
    /// }
    ///
    /// 不过，我们这里的 T 又无处安放，所以又不得不使用 PhantomData，它就像一个占位符，但是又没有大小，可以为我们持有在声明时使用不到的数据：

    static NEXT_ID: AtomicU64 = AtomicU64::new(0);

    struct Customer<T> {
        id: u64,
        name: String,
        phantom: PhantomData<T>,
    }

    struct FreeFeature;
    struct PaidFeature;

    trait Free {
        fn feature1(&self);
        fn feature2(&self);
    }

    trait Paid: Free {
        fn paid_feature(&self);
    }

    /// 为 Customer<T> 实现需要的方法
    impl<T> Customer<T> {
        fn new(name: String) -> Self {
            Self {
                id: NEXT_ID.fetch_add(1, atomic::Ordering::Relaxed),
                name,
                phantom: PhantomData,
            }
        }
    }

    /// 免费用户可以升级到付费用户
    impl Customer<FreeFeature> {
        fn andvance(self, payment: f64) -> Customer<PaidFeature> {
            println!(
                "{} ({})  将花费 {:.2} 元，升级到付费用户",
                self.name, self.id, payment
            );
            self.into()
        }
    }

    /// 所有客户都有权使用免费功能
    impl<T> Free for Customer<T> {
        fn feature1(&self) {
            println!("{} 正在使用免费功能一", self.name);
        }
        fn feature2(&self) {
            println!("{} 正在使用免费功能二", self.name);
        }
    }

    /// 付费用户才能使用的功能
    impl Paid for Customer<PaidFeature> {
        fn paid_feature(&self) {
            println!("{} 正在使用付费功能", self.name);
        }
    }

    ///允许免费用户转换成付费用户
    impl From<Customer<FreeFeature>> for Customer<PaidFeature> {
        fn from(c: Customer<FreeFeature>) -> Self {
            Self::new(c.name)
        }
    }

    #[test]
    fn test_customer() {
        // 一开始是免费用户
        let customer = Customer::<FreeFeature>::new("MichaelFu".to_owned());
        customer.feature1();
        customer.feature2();

        // 升级成付费用户，可能使用付费功能和普通功能
        let customer = customer.andvance(99.99);
        customer.feature1();
        customer.feature2();
        customer.paid_feature();
    }
    //使用 PhantomData<T> 表示我们的结构体拥有 T 类型的数据，当我们的结构体删除的时候，可能会删除一个或者多个 T 类型的实例。但是，如果我们的结构体实际上并不拥有类型 T 的数据，
    //那么我们最好使用 PhantomData<&'a T> 或者 PhantomData<*const T> 。

    /// 很多时候，我们需要实现一些自动优化的数据结构，在某些情况下是一种优化的数据结构和相应的算法，在其他情况下使用通用的结构和通用的算法。比如当一个 HashSet 的内容比较少的时候，
    /// 可以用数组实现，但内容逐渐增多，再转换成用哈希表实现。如果我们想让使用者不用关心这些实现的细节，使用同样的接口就能享受到更好的性能，那么，就可以考虑用智能指针来统一它的行为。
    ///
    /// 我们来实现一个智能 String，Rust 下 String 在栈上占了 24 个字节，然后在堆上存放字符串实际的内容，对于一些比较短的字符串，这很浪费内存。
    ///
    /// 参考 Cow，我们可以用一个 enum 来处理：当字符串小于 N 字节时，我们直接用栈上的数组，否则使用 String。但是这个 N 不宜太大，否则当使用 String 时，会比目前的版本浪费内存。
    ///
    /// 当使用 enum 时，额外的 tag + 为了对齐而使用的 padding 会占用一些内存。因为 String 结构是 8 字节对齐的，我们的 enum 最小 8 + 24 = 32 个字节。
    ///
    /// 所以，可以设计一个数据结构，内部用 1 个字节表示字符串的长度，用 30 个字节表示字符串内容，再加上 1 个字节的 tag，正好也是 32 字节，可以和 String 放在一个 enum 里使用，
    /// 我们暂且称这个 enum 叫 SmartString，它的结构如下图所示：

    /// INLINE_STRING_MAX_LEN represent the maximum length. that can be stored in the stack.
    const INLINE_STRING_MAX_LEN: usize = 30;

    /// InlineString 会被存储在栈上，最多占用 32 字节
    struct InlineString {
        len: u8,
        data: [u8; INLINE_STRING_MAX_LEN],
    }

    impl InlineString {
        /// 这里的 new 接口不能暴露出去，我们需要在调用的时候保证传入的字节长度小于 INLINE_STRING_MAX_LEN
        fn new(input: impl AsRef<str>) -> Self {
            let bytes = input.as_ref().as_bytes();
            let len = bytes.len();
            let mut data = [0u8; INLINE_STRING_MAX_LEN];
            data[..len].copy_from_slice(bytes);
            Self {
                len: len as u8,
                data,
            }
        }
    }

    impl __Deref for InlineString {
        type Target = str;

        fn deref(&self) -> &Self::Target {
            // 由于生成 InlineString 的接口是隐藏的，它只能来自字符串，所以下面这行是安全的
            std::str::from_utf8(&self.data[..self.len as usize]).unwrap()
        }
    }

    impl Debug for InlineString {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.deref())
        }
    }

    #[derive(Debug)]
    enum SmartString {
        Inline(InlineString),
        Standard(String),
    }

    impl __Deref for SmartString {
        type Target = str;

        fn deref(&self) -> &Self::Target {
            match *self {
                SmartString::Inline(ref v) => v.deref(),
                SmartString::Standard(ref v) => v.deref(),
            }
        }
    }

    impl From<&str> for SmartString {
        fn from(s: &str) -> Self {
            match s.len() > INLINE_STRING_MAX_LEN {
                true => SmartString::Standard(s.to_owned()),
                _ => SmartString::Inline(InlineString::new(s)),
            }
        }
    }

    impl Display for SmartString {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.deref())
        }
    }

    #[test]
    fn smart_string_test() {
        let len1 = std::mem::size_of::<SmartString>();
        let len2 = std::mem::size_of::<InlineString>();
        println!("Len: SmartString {}, Len: InlineString {}", len1, len2);
        let s1: SmartString = "hello world".into();
        let s2: SmartString = SmartString::from("这是一个超过了三十个字节的很长很长的字符串");
        println!("s1: {}, s2: {}", s1, s2);

        //display 输出
        println!(
            "s1: {}({} bytes, {} chars), s2: {}({} bytes, {} chars)",
            s1,
            s1.len(),
            s1.chars().count(),
            s2,
            s2.len(),
            s2.chars().count()
        );

        assert!(s1.ends_with("world"));
        assert!(s2.starts_with("这是"))
    }

    /// 使用 std::sync::Mutex 可以多线程共享可变数据，Mutex、RwLock 和原子类型，即使声明为 non-mut，这些类型也可以修改：
    #[test]
    fn mutex_test() {
        // 用 Arc 来提供并发环境下的共享所有权（使用引用计数）
        let metrics: Arc<Mutex<HashMap<Cow<'static, str>, usize>>> =
            Arc::new(Mutex::new(HashMap::new()));
        for _ in 1..32 {
            let m = metrics.clone();
            thread::spawn(move || {
                let mut g = m.lock().unwrap();
                // 此时只有拿到 MutexGuard 的线程可以访问 HashMap
                let data = &mut *g;

                // Cow 实现了很多数据结构的 From trait，所以我们可以用 "hello".into() 生成 Cow
                let value = data.entry("hello".into()).or_insert(0);
                *value += 1;

                // MutexGuard 被 Drop，锁被释放
            });
        }

        thread::sleep(Duration::from_millis(200));
        println!("metrics: {:?}", metrics.lock().unwrap());
    }

    /// 使用 unsafe 特性构造指向同一块内存的两个变量，导致 Double Free：
    #[test]
    fn double_free_test() {
        let mut d = String::from("cccc");
        let d_len = d.len();

        let mut c = String::with_capacity(d_len);
        unsafe {
            std::ptr::copy(&d, &mut c, 1);
        };
        println!("{:?}", c.as_ptr());

        println!("{:?}", d.as_ptr());
        d.push('c');
        println!("{}", d);
    }

    #[test]
    fn arc_deref_move_test() {
        let s = Arc::new(Box::new("hello".to_string()));
        println!("{:p}", &s);
        println!("{:p}", s.as_ptr());
        // DerefMove Error : cannot move out of an `Arc`
        //但如果换成 Box 是可以的。
        let s2 = s;
        // println!("{:p}", s.as_ptr()); // Moved s
        println!("{:p}", s2.as_ptr());
    }

    /// 泛型参数可以是早期绑定或晚期绑定，当前（以及在可预见的将来）类型参数总是早期绑定，但生命周期参数可以是早期绑定或后绑定。早期绑定参数由编译器在单态化期间确定，由于类型参数始终是早期绑定的，因此不能拥具有未解析类型参数的值。
    fn m<T>() {}
    fn s<'a>(_: &'a ()) {}
    #[test]
    fn par_bind_test() {
        let m1 = m::<u8>;
        //let m2 = m; // error: cannot infer type for `T`
        let m3 = s; // ok even though 'a isn't provided
        //出于这个原因，我们不能指定生命周期直到它被调用，也不能让借用检查器去推断它：
        // error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
        //let m4 = s::<'static>;

        // error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
        //let m5 = s::<'_>;
    }

    //晚期绑定参数的想法与 Rust 的一个称为 “高级 Trait 边界”（HRTB）的特性有很大的重叠，这是一种机制，用于表示 trait 参数的界限是后期界限。目前这仅限于生命周期参数，可以使用 for 关键字表达生命周期的 HRTB，
    //例如，对于上面的 m1：

    //let m1: impl for<'r> Fn(&'r ()) = m;

    //可以把它理解为这里有一个生命周期，但是我们目前还不需要知道它。

    //后期绑定生命周期总是无限的；没有语法来表示必须比其他生命周期更长的后期绑定生命周期：

    //除非开发人员明确使用 HRTB 作为语法，否则数据类型的生命周期总是提前绑定的。在函数上，生命周期默认为后期绑定，但在以下情况下可以提前绑定：
    //生命周期在函数签名之外声明，例如在结构体的关联方法中，它可以来自结构体本身；
    //生命周期参数以它必须超过的其他生命周期为界；

    //下面这段代码编译失败，原因很很直接，我们对 buf 存在两次可变借用，但是我们的第一次可变借用在获取 b1 之后就应该失效，只要 buf 存在，b1 和 b2 就应该保持有效。但是从 read_bytes 的实现中我们可以看出，
    //它有一个后期绑定生命周期参数，返回值还和每次调用的可变借用必须具有相同生命周期，所以可变借用得保留到返回值最后一次使用位置。

    //但是我们将我们的 Buffer 改改，让它拥有一个具有 'a 的 buf，而且让 read_bytes 的返回值生命周期跟 buf 相同，这样就和它的调用者没关系了，生成 b1 和 b2 的可变借用在它们使用完就结束了，
    //这里 read_bytes 的参数生命周期是早期绑定的，在编译期间就能但太态化。
    struct Buffer<'a> {
        buf: &'a [u8],
        pos: usize,
    }

    impl <'b , 'a: 'b> Buffer<'a> {
        fn new(b: &'a [u8]) -> Buffer {
            Buffer { buf: b, pos: 0, }
        }

        fn read_bytes(&'b mut self) -> &'a [u8] {
            self.pos += 3;
            &self.buf[self.pos - 3..self.pos]
        }
    }

    fn print(b1: &[u8], b2: &[u8]) {
        println!("{:#?} {:#?}", b1, b2);
    }

    #[test]
    fn buffer_test() {
        let v = vec![1, 2, 3, 4, 5, 6];
        let mut buf = Buffer::new(&v);
        let b1 = buf.read_bytes();  // 第一次可变借用，相当于 (&mut buf).read_bytes()
        let b2 = buf.read_bytes();  // 第二次可变借用
        print(b1, b2);                     // b1 和 b2 引用至 v，和 v 有相同的生命周期

    }
}
