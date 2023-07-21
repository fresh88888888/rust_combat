/// A ASCII-encoded string
mod my_ascii {

    #[derive(Debug, PartialEq, Eq)]
    pub struct Ascii(Vec<u8>);

    // When conversion fails, we give back the vector we couldn't convert. This should implement `std::error::Error` ; omitted for brevity.
    #[derive(Debug, PartialEq, Eq)]
    pub struct NotAsciiError(Vec<u8>);

    // Safe, efficient conversion, implemented using unsafe code.
    impl From<Ascii> for String {
        fn from(ascii: Ascii) -> String {
            unsafe { String::from_utf8_unchecked(ascii.0) }
        }
    }

    impl Ascii {
        /// Create an `Ascii` from the ASCII text in `bytes` . Return a NotAsciiError` error if `bytes` contains any non-ASCII characters.
        pub fn from_bytes(bytes: Vec<u8>) -> Result<Ascii, NotAsciiError> {
            if bytes.iter().any(|&byte| !byte.is_ascii()) {
                return Err(NotAsciiError(bytes));
            }
            Ok(Ascii(bytes))
        }
    }
}

pub use std::boxed::Box;
pub use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
pub use std::string::ToString;

#[macro_export]
macro_rules! json {
        (null) => {
            Json::Null
        };

        ([$($element:tt),*]) => {
            Json::Array(vec![json!($element),*])
        };

        ({$($key: tt, $value: tt),*}) => {
            {
                let mut fields = $crate::json::Box::new($crate::json::HashMap::new());
                $(fields.insert($crate::json::ToString::to_string($key), json!($value)))*
                Json::Object(fields)
            }
        };

        ($other:tt) => {
            Json::from($other)
        };
}

#[macro_export]
macro_rules! log {
    ($format:tt, $($arg:expr),*) => {
        $crate::leetcode::unsafe_desc::write_log_entry(format_args!($format, $($arg),*))
    };
}

fn write_log_entry(entry: std::fmt::Arguments) {
    if true {
        // Keep things simple for now, and just open the file every time.
        let mut log_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("log-file-name")
            .expect("failed to open log file");
        log_file.write_fmt(entry).expect("failed to write to log");
        println!("argument: {:?}, log_file: {:?}", entry, log_file);
    }
}

#[cfg(test)]
mod my_ascii_test {

    use std::{
        collections::{hash_map::DefaultHasher, HashMap, HashSet},
        error::Error,
        fmt::{self, Display},
        fs::File,
        hash::{Hash, Hasher},
        io::{self, BufRead, BufReader, Read},
        net::{IpAddr, TcpListener},
        path::PathBuf,
        rc::Rc,
        str::FromStr,
        thread::spawn,
        vec,
    };

    use regex::Regex;
    use unicode_normalization::UnicodeNormalization;

    use super::my_ascii::Ascii;

    #[test]
    fn from_string_test() {
        let bytes: Vec<u8> = b"ASCII and ye shell receive".to_vec();
        // This call entails no allocation or text copies, just a scan.
        let ascii: Ascii = Ascii::from_bytes(bytes).unwrap();

        let string = String::from(ascii);
        assert_eq!(string, "ASCII and ye shell receive");
    }

    fn option_to_raw<T>(opt: Option<&T>) -> *const T {
        match opt {
            None => std::ptr::null(),
            Some(t) => t as *const T,
        }
    }

    #[test]
    fn option_to_raw_test() {
        assert!(!option_to_raw(Some(&("pea", "poo"))).is_null());
        assert_eq!(option_to_raw::<i32>(None), std::ptr::null());
    }

    #[test]
    fn ptr_operator_test() {
        let trucks = vec!["garbage truck", "dump truck", "moonstruck"];
        let first: *const &str = &trucks[0];
        let last: *const &str = &trucks[2];
        assert_eq!(unsafe { last.offset_from(first) }, 2);
        assert_eq!(unsafe { first.offset_from(last) }, -2);
    }

    #[test]
    fn type_size_align_test() {
        // Fat pointers to slices carry their referent's length.
        let slice: &[i32] = &[1, 3, 9, 27, 81];
        assert_eq!(std::mem::size_of_val(slice), 20);
        let text: &str = "alligator";
        assert_eq!(std::mem::size_of_val(text), 9);
        let unremarkable: &dyn Display = &193_u8;
        let remarkable = &0.0072973525664;
        // These return the size/alignment of the value the trait object points to, not those of the trait object
        // itself. This information comes from the vtable the trait object refers to.
        assert_eq!(std::mem::size_of_val(unremarkable), 1);
        assert_eq!(std::mem::size_of_val(remarkable), 8);
    }

    #[repr(C)]
    union SignExtractor {
        value: i64,
        bytes: [u8; 8],
    }

    fn sign(int: i64) -> bool {
        let se = SignExtractor { value: int };
        println!("{:b}, {:?}", unsafe { se.value }, unsafe { se.bytes });
        unsafe { se.bytes[7] >= 0b10000000 }
    }

    #[test]
    fn unin_sign_test() {
        assert_eq!(sign(-1), true);
        assert_eq!(sign(1), false);
        assert_eq!(sign(i64::MAX), false);
        assert_eq!(sign(i64::MIN), true);
    }

    fn message(addr: String, maybe_payload: Option<Vec<u8>>) -> Result<u64, io::Error> {
        // ...
        if let Some(payload) = maybe_payload {
            // of you go
        }
        // ...
        Ok(0)
    }

    fn grep<R>(target: &str, reader: R) -> io::Result<()>
    where
        R: BufRead,
    {
        for line_result in reader.lines() {
            let line = line_result?;
            if line.contains(target) {
                println!("{}", line);
            }
        }

        Ok(())
    }

    fn main_grep() -> Result<(), Box<dyn Error>> {
        // Get the command-line arguments. The first argument is the string to search for; the rest are filenames.
        let mut args = std::env::args().skip(1);
        let target = match args.next() {
            Some(s) => s,
            None => Err("usage: grep PATTERN File...")?,
        };
        let files: Vec<PathBuf> = args.map(PathBuf::from).collect();

        if files.is_empty() {
            let stdin = io::stdin();
            grep(&target, stdin.lock())?;
        } else {
            for file in files {
                let f = File::open(file)?;
                grep(&target, BufReader::new(f))?;
            }
        }

        Ok(())
    }

    #[test]
    fn buffer_reader_test() {
        let result = main_grep();
        if let Err(er) = result {
            eprintln!("{}", er);
            std::process::exit(1);
        }
    }

    ///对于底层网络代码，从 std::net 模块开始，它为 TCP 和 UDP 网络提供跨平台支持，使用 native_tls crate 来支持 SSL/TLS。
    /// 这些模块为网络上直接的、阻塞的输入和输出提供了构建块，可以用几行代码编写一个简单的服务器，使用 std::net 并为每个连接生成一个线程。 例如，这是一个 "echo" 服务器：
    /// Accept connections forever, spawning a thread for each one.
    fn echo(addr: &str) -> io::Result<()> {
        let listener = TcpListener::bind(addr)?;
        println!("listening on {}", addr);

        loop {
            // wait for a client to connect
            let (mut stream, addr) = listener.accept()?;
            println!("connection received from {}", addr);

            // spawn a thread to handle this lient.
            let mut write_stream = stream.try_clone()?;

            spawn(move || {
                // echo everything we receive from `stream` back to it.
                io::copy(&mut stream, &mut write_stream).expect("error: in client thread: ");
                println!("connection closed...");
            });
        }
    }

    #[test]
    fn echo_net_test() {
        echo("127.0.0.1:17007").expect("error: ");
    }

    #[test]
    fn vec_collector_test() {
        let slice = [0, 1, 2, 3];
        assert_eq!(slice.get(1), Some(&1));
        assert_eq!(slice.get(4), None);

        let mut slice_2 = [0, 1, 2, 3];
        {
            //因为按值返回 T 意味着转移所有权，所以访问 item 的方法通常通过引用返回这些 item。
            let last = slice_2.last_mut().unwrap();
            assert_eq!(*last, 3);
            *last = 10;
        }

        assert_eq!(slice_2, [0, 1, 2, 10]);

        let v = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(v.to_vec(), [1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(v[0..6], [1, 2, 3, 4, 5, 6]);

        let mut byte_vec = b"Misssssssissippi".to_vec();
        //vec.dedup()：删除连续重复的元素，删除完之后仍然有两个 s：
        byte_vec.dedup();
        assert_eq!(&byte_vec, b"Misisipi");

        //如果要删除所有重复的 item
        let mut byte_vec_2 = b"Misssssssissippi".to_vec();
        let mut seen = HashSet::new();
        byte_vec_2.retain(|item| seen.insert(*item));
        assert_eq!(&byte_vec_2, b"Misp");

        //slices.concat()：连接所有的 item 并且返回新的 vector：
        assert_eq!([[1, 2], [3, 4], [5, 6]].concat(), vec![1, 2, 3, 4, 5, 6]);

        //slices.join(&separator)：类似于前者，但是在连接的过程中可以插入一个分割 item：
        assert_eq!(
            [[1, 2], [3, 4], [5, 6]].join(&0),
            vec![1, 2, 0, 3, 4, 0, 5, 6]
        );

        vec![1, 2, 3, 4, 5, 6].windows(2).for_each(|items| {
            for item in items {
                print!("{} ", item);
            }
        });

        assert_eq!([1, 2, 3, 4].starts_with(&[1, 2]), true);
        assert_eq!([1, 2, 3, 4].starts_with(&[2, 3]), false);
        assert_eq!([1, 2, 3, 4].ends_with(&[3, 4]), true);

        let a = HashSet::from([1, 2, 3]);
        let b = HashSet::from([4, 2, 3, 4]);

        for x in a.symmetric_difference(&b) {
            println!("{}", x);
        }

        //set1.symmetric_difference(&set2)：返回只存在其中一个集合中的值的新集合；
        let diff1: HashSet<_> = a.symmetric_difference(&b).collect();
        let diff2: HashSet<_> = b.symmetric_difference(&a).collect();

        assert_eq!(diff1, diff2);
        assert_eq!(diff1, [1, 4].iter().collect());
    }

    #[test]
    fn char_str_test() {
        // assert_eq!("カニ".chars().next(), Some('力'));
        assert!(32u8.is_ascii_whitespace());
        assert!(b'9'.is_ascii_digit());

        // 在使用这些函数来实现现有规范时要小心，因为分类可能不同。例如 is_whitespace 和 is_ascii_whitespace 在对某些字符的处理上有所不同：
        // 因为 is_ascii_whitespace 实现了 web 标准的空白字符，而 is_whitespace 实现了 Unicode 标准的字符。
        let line_tab = '\u{000b}';
        assert_eq!(line_tab.is_whitespace(), true);
        assert_eq!(line_tab.is_ascii_whitespace(), false);

        assert_eq!('F'.to_digit(16), Some(15));
        assert_eq!(std::char::from_digit(15, 16), Some('f'));
        assert!(char::is_digit('f', 16));
        assert!(char::is_digit('8', 10));

        // ch.to_lowercase()、ch.to_uppercase()：转换成小写或者大小可迭代字符序列，根据 Unicode 大小写转换算法；
        let mut u = 'U'.to_lowercase();
        assert_eq!(u.next(), Some('u'));
        assert_eq!(u.next(), None);

        // 使用 as 操作符可以将字符转换成整数，高位字节可能会被删除：
        assert_eq!('B' as u32, 66);
        assert_eq!('饂' as u8, 66); // upper bits truncated
        assert_eq!('二' as i8, -116); // same

        assert_eq!(char::from(66), 'B');
        assert_eq!(std::char::from_u32(0x9942), Some('饂'));
        assert_eq!(std::char::from_u32(0xd800), None); // reserved for UTF-16

        let spacey = "man hat tan";
        let spaceless: String = spacey.chars().filter(|c| !c.is_whitespace()).collect();
        assert_eq!(spaceless, "manhattan");

        let full = "bookkeeping";
        assert_eq!(&full[..4], "book");
        assert_eq!(&full[5..], "eeping");
        assert_eq!(&full[2..4], "ok");
        assert_eq!(full[..].len(), 11);
        assert_eq!(full[5..].contains("boo"), false);

        // 不能通过括号单个索引字符，必须要先将字符串转换成 Unicode 字符序列，然后进行迭代：
        assert_eq!(full.chars().next(), Some('b'));

        // string.extend(iter)：将迭代器产生的所有 item 追加到 String，迭代器可以产生 char，str 或者 String，这些都被 String 实现：
        let mut also_spaceless = "con".to_string();
        also_spaceless.extend("tri but ion".split_whitespace());
        assert_eq!(also_spaceless, "contribution");

        // 因为 String 实现了 Add<&str> 和 AddAssign<&str>，所以你可以使用 + 和 +=：
        let left = "partners".to_string();
        let mut right = "crime".to_string();
        assert_eq!(left + " in " + &right, "partners in crime");
        right += " doesn't pay";
        assert_eq!(right, "crime doesn't pay");
        // 但是左操作数不能是 &str，所以下面的写法是不可以的：let parenthetical = "(" + string + ")"; 而是应该这样写： let parenthetical = "(".to_string() + &string + ")";
        // 但是，不鼓励从末尾向后构建字符串。字符串的行为方式与向量相同，当它需要更多容量时，它的缓冲区大小总是至少翻倍。这使重新复制开销与最终大小成正比。即便如此，
        // 使用  String::with_capacity 创建具有正确缓冲区大小的字符串可以完全避免调整大小，并且可以减少重新的内存分配。

        // string.drain(range)：删除指定范围的资费并且返回，后面的字符会前移：
        let mut chco = "chocolate".to_string();
        assert_eq!(chco.drain(3..6).collect::<String>(), "col");
        assert_eq!(chco, "choate");

        // string.replace_range(range, replacement)：用给定的替换字符串切片替换字符串中的给定范围。切片的长度不必与被替换的范围相同，但除非被替换的范围到达字符串的末尾，
        // 否则将需要移动范围末尾之后的所有字节：
        let mut beverage = "a piña colada".to_string();
        beverage.replace_range(2..7, "kahlua");
        assert_eq!(beverage, "a kahlua colada");

        // 当标准库函数需要搜索、匹配、拆分或修剪文本时，它接受几种不同的类型来表示要查找的内容：
        let haystack = "One fine day, in the middle of the night";
        assert_eq!(haystack.find(','), Some(12));
        assert_eq!(haystack.find("night"), Some(35));
        assert_eq!(haystack.find(char::is_whitespace), Some(3));
        assert_eq!(
            "## Elephants".trim_start_matches(|ch: char| ch == '#' || ch.is_whitespace()),
            "Elephants"
        );
    }

    #[test]
    fn str_test() {
        let code = "\t function noodle() { ";
        // &[char] 匹配任何出现在 char 列表中的字符，如果使用数组字面量，需要使用 as_ref 进行类型转换：
        assert_eq!(
            code.trim_start_matches([' ', '\t'].as_ref()),
            "function noodle() { "
        );
        // Shorter equivalent: &[' ', '\t'][..]

        // slice.find(pattern), slice.rfind(pattern)：返回 Some(i) 表示 slice 包含指定的模式，i 是偏移量，find 找到最后一个匹配的，而 rfind 找到最后一个匹配的：
        let quip = "We also know there are known unknowns";
        assert_eq!(quip.find("know"), Some(8));
        assert_eq!(quip.rfind("known"), Some(31));
        assert_eq!(quip.find("ya know"), None);
        assert_eq!(quip.rfind(char::is_uppercase), Some(0));

        //slice.replace(pattern, replacement)：替换所有匹配 pattern 子串：
        assert_eq!(
            "The only thing we have to fear is fear itself".replace("fear", "spin"),
            "The only thing we have to spin is spin itself"
        );
        assert_eq!(
            "`Borrow` and `BorrowMut`".replace(|ch: char| !ch.is_alphanumeric(), ""),
            "BorrowandBorrowMut"
        );
        // .replace() 在重叠匹配上的行为可能有点怪，在这里，模式 "aba" 有四个实例，但在替换第一个和第三个后，第二个和第四个不再匹配：
        assert_eq!("cabababababbage".replace("aba", "***"), "c***b***babbage");
        // slice.replacen(pattern, replacement, n)：和前者相同，但是至多替换 n 次；

        assert_eq!(
            "élan".char_indices().collect::<Vec<_>>(),
            vec![
                (0, 'é'), // has a two-byte UTF-8 encoding
                (2, 'l'),
                (3, 'a'),
                (4, 'n')
            ]
        );

        // slice.bytes()：返回切片各个字节的迭代器：
        assert_eq!(
            "élan".bytes().collect::<Vec<_>>(),
            vec![195, 169, b'l', b'a', b'n']
        );

        // The ':' characters are separators here. Note the final "".
        assert_eq!(
            "jimb:1000:Jim Blandy:".split(':').collect::<Vec<_>>(),
            vec!["jimb", "1000", "Jim Blandy", ""]
        );

        // The '\n' characters are terminators here.
        assert_eq!(
            "127.0.0.1 localhost\n127.0.0.1 www.reddit.com\n"
                .split_terminator('\n')
                .collect::<Vec<_>>(),
            vec!["127.0.0.1 localhost", "127.0.0.1 www.reddit.com"]
        );
        // Note, no final ""!

        let poem = "This is just to say\n I have eaten\n the plums\n again\n";
        // slice.split_whitespace(), slice.split_ascii_whitespace()：通过 Unicode 定义的空格和 ASCII 空格来分割字符串：
        assert_eq!(
            poem.split_whitespace().collect::<Vec<_>>(),
            vec!["This", "is", "just", "to", "say", "I", "have", "eaten", "the", "plums", "again"]
        );

        // slice.trim()：返回删除了前后空格的子串，slice.trim_start() 和 slice.trim_end() 仅删除前或后空格：
        assert_eq!("\t*.rs ".trim(), "*.rs");
        assert_eq!("\t*.rs ".trim_start(), "*.rs ");
        assert_eq!("\t*.rs ".trim_end(), "\t*.rs");
        // slice.trim_matches(pattern)：删除 slice 前后匹配 pattern 的子串，trim_start_matches 和 trim_end_matches 仅作用于前面或者后面：
        assert_eq!("001990".trim_start_matches('0'), "1990");

        // 如果一个类型实现了 std::str::FromStr，那么它就提供了一个标准的方式可以从字符串生成它的值：
        use std::str::FromStr;
        assert_eq!(usize::from_str("3628800"), Ok(3628800));
        assert_eq!(f64::from_str("128.5625"), Ok(128.5625));
        assert_eq!(bool::from_str("true"), Ok(true));
        assert!(f64::from_str("not a float at all").is_err());
        assert!(bool::from_str("TRUE").is_err());

        // char 也实现了 FromStr，但是只针对那些只包含一个字符的：
        assert_eq!(char::from_str("é"), Ok('é'));
        assert!(char::from_str("abcdefg").is_err());

        // std::net::IpAddr 也实现了 FromStr：
        let address = IpAddr::from_str("fe80::0000:3ea9:f4ff:fe34:7a50").unwrap();
        assert_eq!(
            address,
            IpAddr::from([0xfe80, 0, 0, 0, 0x3ea9, 0xf4ff, 0xfe34, 0x7a50])
        );

        // 字符串切片有一个 parse 方法，可以将切片解析为想要的任何类型，只要它实现了 FromStr，但是需要拼出所需的类型：
        let address = "fe80::0000:3ea9:f4ff:fe34:7a50".parse::<IpAddr>().unwrap();
        println!("{:?}", address);
    }

    #[test]
    fn str_2_test() {
        let address = "fe80::0000:3ea9:f4ff:fe34:7a50".parse::<IpAddr>().unwrap();
        assert_eq!(format!("{}, wow", "doge"), "doge, wow");
        assert_eq!(format!("{}", true), "true");
        assert_eq!(
            format!("({:.3}, {:.3})", 0.5, f64::sqrt(3.0) / 2.0),
            "(0.500, 0.866)"
        );

        // Using `address` from above.
        let formatted_addr: String = format!("{}", address);
        assert_eq!(formatted_addr, "fe80::3ea9:f4ff:fe34:7a50");
        // 所有 Rust 的数字类型，字符以及字符串都实现了 Display，智能指针 Box<T>, Rc<T>, Arc<T> 在 T 实现 Display 时也会实现 Display，Vec 和 HashMap 没有实现 Display。

        // 如果一个类型实现了 Display，那么他就会自动实现 std::str::ToString，可以通过调用 .to_string() 达到目的：
        assert_eq!(address.to_string(), "fe80::3ea9:f4ff:fe34:7a50");

        // 标准库里面的导出类型都实现了 std::fmt::Debug，可以通过 {:?} 格式声明生成字符串：
        let address = "fe80::0000:3ea9:f4ff:fe34:7a50".parse::<IpAddr>().unwrap();

        // Continued from above.
        let addresses = vec![address, IpAddr::from_str("192.168.0.1").unwrap()];
        assert_eq!(
            format!("{:?}", addresses),
            "[fe80::3ea9:f4ff:fe34:7a50, 192.168.0.1]"
        );

        // 对于任何实现了 Debug 的 T，Vec<T> 也实现了 Debug，所有 Rust 集合类型都有这样的实现。可以通过派生为自己的类型实现 Debug：
        #[derive(Copy, Clone, Debug)]
        struct Complex {
            re: f64,
            im: f64,
        }

        // String::from_utf8(vec)：尝试去构建字符串从 Vec<u8>，如果转化成功，返回 Ok(String)，并且将 Vec 中缓冲区的所有权转移至 String，以至于没有额外的内存申请。如果转换失败，
        // 返回 Err(e)，e 的类型是 FromUtf8Error，可以调用 e.into_bytes() 获得原 vec 的所有权：
        let good_utf8: Vec<u8> = vec![0xe9, 0x8c, 0x86];
        assert_eq!(String::from_utf8(good_utf8).ok(), Some("錆".to_string()));

        let bad_utf8: Vec<u8> = vec![0x9f, 0xf0, 0xa6, 0x80];
        let result = String::from_utf8(bad_utf8);
        assert!(result.is_err());

        // Since String::from_utf8 failed, it didn't consume the original. vector, and the error value hands it back to us unharmed.
        assert_eq!(
            result.unwrap_err().into_bytes(),
            vec![0x9f, 0xf0, 0xa6, 0x80]
        );

        assert_eq!(format!("{:4}", "th\u{e9}"), "th\u{e9} ");
        assert_eq!(format!("{:4}", "the\u{301}"), "the\u{301}");

        // 格式化
        let mut map = HashMap::new();
        map.insert("Portland", (45.5237606, -122.6819273));
        map.insert("Taipei", (25.0375167, 121.5637));
        println!("{:#?}", map);

        // 调试格式通常以十进制打印数字，但可以在问号前放置一个 x 或 X 来请求十六进制。前导 0 和字段宽度语法也可以接受。
        println!("ordinary: {:02?}", [9, 15, 240]);
        println!("hex: {:02x?}", [9, 15, 240]);

        // 通常，如果将任何类型的指针传递给格式化宏 —— 引用、Box、Rc—— 宏只会格式化引用的对象，指针本身并不重要。但是在调试时，
        // 有时查看指针会很有帮助：地址可以作为单个值的粗略 “名称”，这在检查具有循环或共享的结构时会很有启发性。
        // {:p} 将引用和智能指针格式化为地址：
        let original = Rc::new("mazurka".to_string());
        let cloned = original.clone();
        let impostor = Rc::new("mazurka".to_string());

        println!("text: {}, {}, {}", original, cloned, impostor);
        println!("pointers: {:p} {:p}, {:p}", original, cloned, impostor);

        // 可以简单的通过索引来指定格式化参数使用哪个值，也就是指定开始所说的 which：
        assert_eq!(
            format!("{1}, {0}, {2}", "zeroth", "first", "second"),
            "first, zeroth, second"
        );
        assert_eq!(
            format!("{2:#06x}, {1:b}, {0:=>10}", "first", 10, 100),
            "0x0064, 1010, =====first"
        );

        assert_eq!(
            format!(
                "{description:.<25}{quantity:2} @ {price:4.2}",
                price = 3.25,
                quantity = 3,
                description = "Maple Turmeric Latte"
            ),
            "Maple Turmeric Latte..... 3 @ 3.25"
        );

        // 可以将命名参数，位置参数，索引参数混合起来使用，只是命名参数必须出现在最后。位置参数与参数从左到右配对，就好像索引和命名参数不存在一样：
        assert_eq!(
            format!(
                "{mode} {2} {} {}",
                "people",
                "eater",
                "purple",
                mode = "flying"
            ),
            "flying purple people eater"
        );

        fn get_width() -> usize {
            10
        }

        fn get_limit() -> usize {
            10
        }

        let content = "hello world";
        println!(
            "{:>width$.limit$}",
            content,
            width = get_width(),
            limit = get_limit()
        );
        println!("{:.*}", get_limit(), content);
    }

    // fmt 方法的工作是生成有效的 self 表示并将其字符写入 dest。 除了作为输出流之外，dest 参数还携带从格式参数解析的详细信息，例如对齐方式和最小字段宽度。
    #[derive(Debug)]
    struct Complex {
        re: f64,
        im: f64,
    }

    impl fmt::Display for Complex {
        fn fmt(&self, dest: &mut fmt::Formatter<'_>) -> fmt::Result {
            let (re, im) = (self.re, self.im);
            if dest.alternate() {
                let abs = f64::sqrt(re * re + im * im);
                let angle = f64::atan2(im, re) / std::f64::consts::PI * 180.0;
                write!(dest, "{} ∠ {}°", abs, angle)
            } else {
                let im_sign = if im < 0.0 { '-' } else { '+' };
                write!(dest, "{} {} {}i", re, im_sign, f64::abs(im))
            }
        }
    }

    // 如果格式化参数中携带 #，我们以极坐标的形式显示负数，否则我们按照常规的方式展示。虽然 fmt 返回 Result，但是我们通常不用处理错误，只需向上传递，Formatter 还有很多其他有用的方法，alternate 只是其中一个。
    #[test]
    fn customer_format_test() {
        let ninety = Complex { re: 0.0, im: 2.0 };
        assert_eq!(format!("{}", ninety), "0 + 2i");
        assert_eq!(format!("{:#}", ninety), "2 ∠ 90°");

        let mysterious_value = Complex { re: 0.2, im: 1.2 };
        log!(
            "O day and night, but this is wondrous strange! {:?}\n",
            mysterious_value
        );
    }

    #[test]
    fn regex_test() {
        let semver = Regex::new(r"(\d+)\.(\d+)\.(\d+)(-[-.[:alnum:]]*)?").unwrap();
        // Simple search, with a Boolean result.
        let haystack = r#"regex = "0.2.5""#;
        assert!(semver.is_match(haystack));

        // Regex::captures 方法在字符串中搜索第一个匹配项，并返回一个 regex::Captures 值，其中包含表达式中每个组的匹配信息：
        let captures = semver
            .captures(haystack)
            .ok_or("semver regex should have matched")
            .unwrap();
        assert_eq!(&captures[0], "0.2.5");
        assert_eq!(&captures[1], "0");
        assert_eq!(&captures[2], "2");
        assert_eq!(&captures[3], "5");

        // 使用索引可能会发生 panic，可以使用 Captures::get，它返回一个 Option<regex::Match>，Match 包含了的那个的组匹配：
        assert_eq!(captures.get(4), None);
        assert_eq!(captures.get(3).unwrap().start(), 13);
        assert_eq!(captures.get(3).unwrap().end(), 14);
        assert_eq!(captures.get(3).unwrap().as_str(), "5");

        // find_iter 为文本中每个连续的非重叠匹配返回一个迭代器，返回相对于文本的开始和结束字节索引。例如：
        let haystack = "In the beginning, there was 1.0.0. For a while, we used 1.0.1-beta, but in the end, we settled on 1.2.4.";
        let matches: Vec<&str> = semver
            .find_iter(haystack)
            .map(|match_| match_.as_str())
            .collect();
        assert_eq!(matches, vec!["1.0.0", "1.0.1-beta", "1.2.4"]);

        // captures_iter 产生的 Captures 包含所有匹配组：
        for caps in semver.captures_iter(haystack) {
            for m in caps.iter() {
                if let Some(m) = m {
                    println!("{} -> {}, {}", m.start(), m.end(), &haystack[m.range()]);
                    break;
                }
            }
        }
    }

    #[test]
    fn regex_2_test() {
        // Regex::new 构造函数可能很昂贵：为 1,200 个字符的正则表达式构造 Regex 即使在快速机器上也需要一毫秒，即使是很小的表达式也需要数微秒，
        // 因此，最好将 Regex 构造排除在繁重的计算循环之外，而是应该构建一次正则表达式，然后重用同一个。

        // lazy_static 提供了一种比较好的方式用于延迟初始化静态值，这些值只有在第一次使用时才会被初始化，在 Cargo.toml 添加如下依赖：
        lazy_static! {
            static ref SEMVER: Regex =
                Regex::new(r"(\d+)\.(\d+)\.(\d+)(-[-.[:alnum:]]*)?").expect("error parsing regex");
        }

        // Simple search, with a Boolean result.
        let haystack = r#"regex = "0.2.5""#;
        assert!(SEMVER.is_match(haystack));

        // 该宏为一个名为 SEMVER 的静态变量的声明，但它的类型不完全是 Regex。 相反，它是实现 Deref<Target=Regex> 的宏生成类型，因此公开了与 Regex 相同的所有方法。
        // 第一次解引用 SEMVER 时会进行初始化，并保存该值以供以后使用。由于 SEMVER 是一个静态变量，而不是一个局部变量，因此初始化程序在每次程序执行时最多运行一次。
    }

    fn hash<T: ?Sized + Hash>(t: &T) -> u64 {
        let mut s = std::collections::hash_map::DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }


    #[test]
    fn normalization_test() {
        assert!("th\u{e9}" != "the\u{301}");
        assert!("th\u{e9}" > "the\u{301}");
        
        // 然而，考虑到 Rust 中 &str 或 String 值，“th\u{e9}” 和 “the\u{301}” 是完全不同的。它们有不同的长度，比较不相等，有不同的哈希值，并且相对于其他字符串有不同的顺序：
        // 显然，如果打算比较用户提供的文本或将其用作哈希表或 Btree 中的键，则需要首先将每个字符串放在某种规范形式中。
        assert_eq!(hash("th\u{e9}"), 0x53e2d0734eb1dff3);
        assert_eq!(hash("the\u{301}"), 0x90d837f0a0928144);


        // 首先，更喜欢字符是尽可能组合还是尽可能分开？例如，越南语单词 Phở 的 composed 形式是三个字符串 "Ph\u{1edf}"，其中声调符号̉和元音符号̛都应用于单个字符的基本字符 "o"，
        // Unicode 负责将拉丁小写字母 o 命名为上面带有角和钩的字符。decomposed 形式将基本字母及其两个标记拆分为三个单独的 Unicode 字符：o、\u{31b} 和 \u{309}，从而产生 Pho\u{31b}\u{309}。
        // 组合形式通常兼容性问题较少，因为它与大多数语言在 Unicode 建立之前用于其文本的表示形式更加匹配。它还可以很好地与 Rust 的 format! 工作。另一方面，decomposed 形式可能更适合显示文本或搜索，
        // 因为它使文本的详细结构更加明确。

        // 第二个问题是：如果两个字符序列表示相同的基本文本，但文本格式的方式不同，你想将它们视为等价还是保持不同？
        // Unicode 对普通数字 5、上标数字 ⁵（或 \u{2075}）和带圆圈的数字 ⑤（或 \u{2464}）有单独的字符，但声明所有这三个是兼容性等价的。类似地，Unicode 有一个用于连字的单个字符 \u{fb03}，
        // 但声明它与三个字符序列 ffi 等效。

        // 兼容性等价对搜索有意义：仅使用 ASCII 字符搜索 "difficult"，应该匹配字符串 "di\u{fb03}cult"。对后一个字符串应用兼容性分解会将连字替换为三个纯字母 “ffi”，从而使搜索更容易。
        // 但是将文本规范化为兼容性等效形式可能会丢失基本信息，因此不应粗心地应用它。例如，在大多数情况下将 2⁵存储为 25 是不正确的。

        // 万维网联盟的建议对所有内容使用 NFC。 Unicode 标识符和模式语法建议在编程语言中使用 NFKC 作为标识符，并提供必要时调整格式的原则。


    }

    #[test]
    fn unicode_normalization_test() {
        // Rust 的 unicode-normalization 提供了一个 trait，它向 &str 添加方法以将文本置于四种规范化形式中的任何一种。要使用它，将以下行添加到 Cargo.toml 文件的 [dependencies] 部分：
        // unicode-normalization = "0.1.17"

        // No matter what representation the left-hand string uses (you shouldn't be able to tell just by looking), these assertions will hold.
        assert_eq!("Phở".nfd().collect::<String>(), "Pho\u{31b}\u{309}");
        assert_eq!("Phở".nfc().collect::<String>(), "Ph\u{1edf}");

        // The left-hand side here uses the "ffi" ligature character.
        assert_eq!("① Di\u{fb03}culty".nfkc().collect::<String>(), "1 Difficulty");
        // Taking a normalized string and normalizing it again in the same form is guaranteed to return identical text.
    }
}
