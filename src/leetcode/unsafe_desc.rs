
/// A ASCII-encoded string
mod my_ascii {

    #[derive(Debug, PartialEq, Eq)]
    pub struct Ascii (Vec<u8>);

    // When conversion fails, we give back the vector we couldn't convert. This should implement `std::error::Error` ; omitted for brevity.
    #[derive(Debug, PartialEq, Eq)]
    pub struct NotAsciiError(Vec<u8>);

    // Safe, efficient conversion, implemented using unsafe code.
    impl From<Ascii> for String{
        fn from(ascii: Ascii) -> String {
            unsafe {String::from_utf8_unchecked(ascii.0)}
        }
    }

    impl Ascii {
        /// Create an `Ascii` from the ASCII text in `bytes` . Return a NotAsciiError` error if `bytes` contains any non-ASCII characters.
        pub fn from_bytes(bytes: Vec<u8>) -> Result<Ascii, NotAsciiError>{
            if bytes.iter().any(|&byte| !byte.is_ascii()) {
                return Err(NotAsciiError(bytes));
            }
            Ok(Ascii(bytes))
        }
    }
}


#[cfg(test)]
mod my_ascii_test {
    use std::fmt::Display;

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
        assert_eq!(unsafe {
            last.offset_from(first)
        }, 2);
        assert_eq!(unsafe {
            first.offset_from(last)
        }, -2);

     }

     #[test]
     fn type_size_align_test() {
        // Fat pointers to slices carry their referent's length.
        let slice: &[i32] = &[1, 3, 9, 27, 81];
        assert_eq!(std::mem::size_of_val(slice), 20);
        let text: &str = "alligator";
        assert_eq!(std::mem::size_of_val(text), 9);
        let unremarkable: &dyn Display  = &193_u8;
        let remarkable: &dyn Display = &0.0072973525664;
        // These return the size/alignment of the value the trait object points to, not those of the trait object
        // itself. This information comes from the vtable the trait object refers to.
        assert_eq!(std::mem::size_of_val(unremarkable), 1);
        assert_eq!(std::mem::size_of_val(remarkable), 8);
        
     }
}