extern crate parser;

#[cfg(test)]
mod test {
    use parser::CharBuffer;
    #[test]
    fn test_charbuf_new() {
        let cb = "r9st".chars().collect();
        let mut tar = CharBuffer::new(cb);
        let ori = CharBuffer::from_str("r9st");
        assert_eq!(ori.to_string(), tar.to_string());
        assert_eq!(ori.to_string(), "r9st");
        assert_eq!(tar.to_string(), "r9st");
        assert_eq!(ori.position(), tar.position());

        assert_eq!(tar.next_with_progress(), Some('r'));
        assert_eq!(tar.position(), 1);
        assert_eq!(tar.next(), Some('9'));
        assert_eq!(tar.position(), 1);
        assert_eq!(tar.length(), 4);
    }

    #[test]
    fn test_next_is_tar() {
        let mut cb = CharBuffer::from_str("[C@H]");
        assert!(!cb.next_is_tar('C'));
        assert!(!cb.next_is_tar('@'));
        assert!(!cb.next_is_tar('H'));
        assert!(!cb.next_is_tar(']'));
        assert!(cb.next_is_tar('['));
        assert_eq!(cb.next_with_progress().unwrap(), '[');

        assert!(!cb.next_is_tar('['));
        assert!(cb.next_is_tar('C'));
        assert!(!cb.next_is_tar('@'));
        assert!(!cb.next_is_tar('H'));
        assert!(!cb.next_is_tar(']'));
        assert_eq!(cb.next_with_progress().unwrap(), 'C');

        assert!(!cb.next_is_tar('['));
        assert!(!cb.next_is_tar('C'));
        assert!(cb.next_is_tar('@'));
        assert!(!cb.next_is_tar('H'));
        assert!(!cb.next_is_tar(']'));
        assert_eq!(cb.next_with_progress().unwrap(), '@');

        assert!(!cb.next_is_tar('['));
        assert!(!cb.next_is_tar('C'));
        assert!(!cb.next_is_tar('@'));
        assert!(cb.next_is_tar('H'));
        assert!(!cb.next_is_tar(']'));
        assert_eq!(cb.next_with_progress().unwrap(), 'H');

        assert!(!cb.next_is_tar('['));
        assert!(!cb.next_is_tar('C'));
        assert!(!cb.next_is_tar('@'));
        assert!(!cb.next_is_tar('H'));
        assert!(cb.next_is_tar(']'));
        assert_eq!(cb.next_with_progress().unwrap(), ']');

        assert_eq!(cb.next_with_progress(), None);
    }

    #[test]
    fn test_length() {
        assert_eq!(CharBuffer::new(vec![]).length(), 0);
        assert_eq!(CharBuffer::new(vec!['a']).length(), 1);
        assert_eq!(CharBuffer::new(vec!['a', 'b']).length(), 2);
        assert_eq!(CharBuffer::from_str("a").length(), 1);
        assert_eq!(CharBuffer::from_str("ab").length(), 2);
        assert_eq!(CharBuffer::from_str("abc").length(), 3);
        assert_eq!(CharBuffer::from_str("").length(), 0);
    }

    #[test]
    fn test_next_with_progress() {
        let mut cb = CharBuffer::from_str("abc");
        assert_eq!(cb.next_with_progress(), Some('a'));
        assert_eq!(cb.position(), 1);
        assert_eq!(cb.next_with_progress(), Some('b'));
        assert_eq!(cb.position(), 2);
        assert_eq!(cb.next(), Some('c'));
        assert_eq!(cb.position(), 2);
        assert_eq!(cb.next_with_progress(), Some('c'));
        assert_eq!(cb.position(), 3);
        assert_eq!(cb.next_with_progress(), None);
        assert_eq!(cb.position(), 3);
    }

    #[test]
    fn test_next() {
        let mut cb = CharBuffer::from_str("abc");
        assert_eq!(cb.position(), 0);
        assert_eq!(cb.next(), Some('a'));
        assert_eq!(cb.position(), 0);
        assert_eq!(cb.next(), Some('a'));
        assert_eq!(cb.position(), 0);
        assert_eq!(cb.next(), Some('a'));
        assert_eq!(cb.position(), 0);

        assert_eq!(cb.next_with_progress(), Some('a'));
        assert_eq!(cb.position(), 1);
        assert_eq!(cb.next_with_progress(), Some('b'));
        assert_eq!(cb.position(), 2);
        assert_eq!(cb.next_with_progress(), Some('c'));
        assert_eq!(cb.position(), 3);
        assert_eq!(cb.next(), None);
        assert_eq!(cb.position(), 3);
        assert_eq!(cb.next_with_progress(), None);
        assert_eq!(cb.position(), 3);
    }

    #[test]
    fn test_is_digit() {
        let mut cb = CharBuffer::new(vec!['r', '2', 's', 't']);
        assert_eq!(cb.next_with_progress(), Some('r'));
        assert!(cb.next_is_digit());
    }

    #[test]
    fn test_next_with_digit() {
        let cb = CharBuffer::from_str("");
        assert_eq!(None, cb.next_with_digit());

        let mut cb_alpha = CharBuffer::from_str("[C1@HC1]");
        assert_eq!(Some(43), cb_alpha.next_with_digit());
        assert_eq!(cb_alpha.next_with_progress().unwrap(), '[');
        assert_eq!(cb_alpha.next_with_progress(), Some('C'));
        assert_eq!(Some(1), cb_alpha.next_with_digit());
        assert_eq!(cb_alpha.next_with_progress(), Some('1'));
        assert_eq!(cb_alpha.next_with_progress(), Some('@'));
        assert_eq!(cb_alpha.next_with_progress(), Some('H'));
        assert_eq!(cb_alpha.next_with_progress(), Some('C'));
        assert_eq!(Some(1), cb_alpha.next_with_digit());
        assert_eq!(cb_alpha.next_with_progress().unwrap(), '1');
        assert_eq!(cb_alpha.next_with_progress(), Some(']'));

        assert_eq!(None, cb_alpha.next_with_digit());
        assert_eq!(cb_alpha.next_with_progress(), None);
    }

    #[test]
    fn test_next_with_digit_and_progress() {
        let mut cb = CharBuffer::new(vec!['z', '2']);
        assert!(!cb.next_is_digit());
        assert_eq!(74, cb.next_with_digit_and_progress().unwrap());
        assert_eq!(2, cb.next_with_digit_and_progress().unwrap());
        assert_eq!(None, cb.next_with_digit());
    }

    #[test]
    fn test_to_number() {
        assert_eq!(CharBuffer::from_str("1").to_number(), Some(1));

        let mut cb_one = CharBuffer::from_str("2C");
        assert_eq!(cb_one.to_number(), Some(2));
        assert_eq!(cb_one.next().unwrap(), 'C');

        assert_eq!(CharBuffer::from_str("12").to_number(), Some(12));
        let mut cb_two = CharBuffer::new(vec!['0', '2']);
        assert_eq!(Some(02), cb_two.to_number());

        let mut cb_two_alph = CharBuffer::from_str("C12");
        assert_eq!(cb_two_alph.to_number(), None);
        assert_eq!(cb_two_alph.next_with_progress().unwrap(), 'C');
        assert_eq!(cb_two_alph.to_number(), Some(12));

        assert_eq!(CharBuffer::from_str("12C").to_number(), Some(12));

        assert_eq!(CharBuffer::from_str("123").to_number(), Some(123));

        let mut cb_three = CharBuffer::from_str("123C");
        assert_eq!(cb_three.to_number(), Some(123));
        assert_eq!(cb_three.next_with_progress().unwrap(), 'C');
    }

    #[test]
    fn test_to_sub_number() {
        let mut cb_beta = CharBuffer::from_str("002C");
        assert_eq!(Some(0), cb_beta.to_sub_number(&mut 0));
        assert_eq!(Some(002), cb_beta.to_sub_number(&mut 3));
        assert!(cb_beta.next_is_tar('C'));
    }

    #[test]
    fn test_to_string() {
        let cb = CharBuffer::from_str("Hello");
        assert_eq!("Hello".to_string(), cb.to_string());
    }

    #[test]
    fn test_is_tar_with_progress() {
        let mut cb = CharBuffer::from_str("rust");
        assert!(cb.is_tar_with_progress('r'));
        assert_eq!(cb.position(), 1);
        assert!(cb.is_tar_with_progress('u'));
        assert_eq!(cb.position(), 2);
        assert!(cb.is_tar_with_progress('s'));
        assert_eq!(cb.position(), 3);
        assert!(cb.is_tar_with_progress('t'));
        assert_eq!(cb.position(), 4);
        assert!(!cb.is_tar_with_progress('t'));
        assert_eq!(cb.position(), 4);
    }

    #[test]
    fn test_substr() {
        let cb = CharBuffer::from_str("");
        assert_eq!(cb.substr(0, 10), String::from(""));

        let cb_alpha = CharBuffer::from_str("Hello World!");
        assert_eq!(cb_alpha.substr(1, 4), "ell".to_string());
        assert_eq!(cb.substr(3, 1), String::from(""));
        assert_eq!(cb.substr(12, 15), String::from(""));
    }
}
