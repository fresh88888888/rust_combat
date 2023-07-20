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

#[cfg(test)]
mod my_ascii_test {

    use std::{
        error::Error,
        fmt::Display,
        fs::File,
        io::{self, BufRead, BufReader, Read},
        path::PathBuf, net::TcpListener, thread::spawn, collections::HashSet,
    };


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
                io::copy(&mut stream, &mut  write_stream).expect("error: in client thread: ");
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
        assert_eq!(&byte_vec_2,  b"Misp");

        //slices.concat()：连接所有的 item 并且返回新的 vector：
        assert_eq!([[1, 2], [3, 4], [5, 6]].concat(), vec![1, 2, 3, 4, 5, 6]);

        //slices.join(&separator)：类似于前者，但是在连接的过程中可以插入一个分割 item：
        assert_eq!([[1, 2], [3, 4], [5, 6]].join(&0), vec![1, 2, 0, 3, 4, 0, 5, 6]);

        vec![1,2,3,4,5,6].windows(2).for_each(|items| {
            for item in items {
                print!("{} ", item);
            }
        });

        assert_eq!([1, 2, 3, 4].starts_with(&[1,2]), true);
        assert_eq!([1, 2, 3, 4].starts_with(&[2,3]), false);
        assert_eq!([1, 2, 3, 4].ends_with(&[3,4]), true);

        let a = HashSet::from([1,2,3]);
        let b = HashSet::from([4, 2, 3, 4]);


        for x in a.symmetric_difference(&b) {
            println!("{}", x);
        }

        //set1.symmetric_difference(&set2)：返回只存在其中一个集合中的值的新集合；
        let diff1: HashSet<_> = a.symmetric_difference(&b).collect();
        let diff2: HashSet<_> = b.symmetric_difference(&a).collect();

        assert_eq!(diff1, diff2);
        assert_eq!(diff1, [1,4].iter().collect());

    }

}
