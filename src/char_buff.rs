#[derive(Debug)]
pub struct CharBuffer {
    char_buffer: Vec<char>,
    pos: usize,
}

impl CharBuffer {
    /// Construct a CharBuffer by a `Vec<char>`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use ruatom::CharBuffer;
    ///
    /// assert_eq!(CharBuffer::new(vec!['H', 'e', 'l', 'l', 'o']).length(), 5);
    ///
    /// let lang = "rust".chars().collect();
    /// let mut cb = CharBuffer::new(lang);
    /// assert_eq!(cb.length(), 4)
    /// ```
    pub fn new(cb: Vec<char>) -> Self {
        Self {
            char_buffer: cb,
            pos: 0,
        }
    }

    /// Construct a CharBuffer from a `&str`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use ruatom::CharBuffer;
    ///
    /// assert_eq!(CharBuffer::from_str("Hello").length(), 5);
    ///
    /// let mut cb = CharBuffer::from_str("rust");
    /// assert_eq!(cb.length(), 4)
    /// ```
    pub fn from_str(s: &str) -> Self {
        let cb = s.chars().collect();
        Self {
            char_buffer: cb,
            pos: 0,
        }
    }

    pub fn is_remain(&self) -> bool {
        return self.pos < self.char_buffer.len();
    }

    /// Convert `char_buffer` to `String`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use ruatom::CharBuffer;
    ///
    /// let cb = CharBuffer::from_str("Hello");
    /// assert_eq!("Hello".to_string(), cb.to_string());
    /// ```
    pub fn to_string(&self) -> String {
        let s: String = self.char_buffer.iter().collect();
        s
    }

    /// Return the next position in `char_buffer`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use ruatom::CharBuffer;
    ///
    /// let mut cb = CharBuffer::from_str("rust");
    /// assert_eq!(cb.position(), 0);
    /// assert_eq!(cb.next_with_progress(), Some('r'));
    /// assert_eq!(cb.position(), 1);
    /// ```
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Return the number of characters in `char_buffer`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use ruatom::CharBuffer;
    ///
    /// assert_eq!(CharBuffer::from_str("Hello").length(), 5);
    /// ```
    pub fn length(&self) -> usize {
        self.char_buffer.len()
    }

    /// Get the next character in `char_buff` and progress position.
    ///
    /// If there are no characters in `char_buffer`, return `None`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use ruatom::CharBuffer;
    ///
    /// let mut cb = CharBuffer::from_str("[C1@HC1]");
    /// assert_eq!(cb.next_with_progress().unwrap(), '[');
    /// assert_eq!(cb.position(), 1);
    /// ```
    pub fn next_with_progress(&mut self) -> Option<char> {
        if self.is_remain() {
            let c = self.char_buffer[self.pos];
            self.pos += 1;
            Some(c)
        } else {
            None
        }
    }

    /// Get the next character in `char_buff`.If there are no characters in `char_buffer`, return `None`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use ruatom::CharBuffer;
    ///
    /// assert_eq!(CharBuffer::from_str("Hello").length(), 5);
    ///
    /// let mut cb = CharBuffer::from_str("[C1@HC1]");
    /// assert_eq!(cb.next().unwrap(), '[');
    /// assert_eq!(cb.position(), 0);
    /// ```
    pub fn next(&self) -> Option<char> {
        if self.is_remain() {
            let c = self.char_buffer[self.pos];
            Some(c)
        } else {
            None
        }
    }

    /// Determine if next character is a digit.If there are no characters in `char_buffer`, return `false`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use ruatom::CharBuffer;
    ///
    /// let mut cb = CharBuffer::new(vec!['r','2','s','t']);
    /// assert_eq!(cb.next_with_progress(), Some('r'));
    /// assert!(cb.next_is_digit());
    /// ```
    pub fn next_is_digit(&self) -> bool {
        if let Some(c) = self.next() {
            return self.is_digit(c);
        };
        return false;
    }

    /// Access next character in `char_buffer` and convert it to digit.If there are no characters in `char_buffer`, return `None`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use ruatom::CharBuffer;
    ///
    /// let cb = CharBuffer::from_str("");
    /// assert_eq!(None, cb.next_with_digit());
    /// let mut cb_alpha = CharBuffer::from_str("[C1@HC1]");
    /// assert_eq!(Some(43), cb_alpha.next_with_digit());
    /// ```
    pub fn next_with_digit(&self) -> Option<usize> {
        match self.next() {
            None => None,
            Some(c) => Some(self.to_digit(c)),
        }
    }

    /// Access next character in `char_buffer` and convert it to digit and progress position.
    ///
    /// If there are no characters in `char_buffer`, return `None`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use ruatom::CharBuffer;
    ///
    /// let mut cb = CharBuffer::new(vec!['z','2']);
    /// assert_eq!(74, cb.next_with_digit_and_progress().unwrap());
    /// assert_eq!(cb.position(), 1);
    /// ```
    pub fn next_with_digit_and_progress(&mut self) -> Option<usize> {
        match self.next_with_progress() {
            None => None,
            Some(c) => Some(self.to_digit(c)),
        }
    }

    fn is_digit(&self, c: char) -> bool {
        c.is_numeric()
    }

    fn to_digit(&self, c: char) -> usize {
        c as usize - '0' as usize
    }

    /// Get a part of `char_buffer` from `beg` to `end` and convert it to `String`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use ruatom::CharBuffer;
    ///
    /// let cb = CharBuffer::from_str("Hello World!");
    /// assert_eq!(cb.substr(1, 4), "ell".to_string());
    /// ```
    pub fn substr(&self, beg: usize, end: usize) -> String {
        if beg > end || end > self.length() || beg >= self.length() {
            return "".to_string();
        }
        self.char_buffer[beg..end].iter().collect()
    }

    /// Determine if next character is `c`, if `true` then progress position.
    ///
    /// If there are no characters in `char_buffer`, return `false`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use ruatom::CharBuffer;
    ///
    /// let mut cb = CharBuffer::from_str("rust");
    /// assert!(cb.next_is_tar('r'));
    /// assert_eq!(cb.next_with_progress().unwrap(), 'r');
    /// assert!(cb.next_is_tar('u'));
    /// ```
    pub fn next_is_tar(&self, c: char) -> bool {
        self.is_remain() && self.char_buffer[self.pos] == c
    }

    /// Determine if next character is `c`, if `true` then progress position.
    ///
    /// If there are no characters in `char_buffer`, return `false`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use ruatom::CharBuffer;
    ///
    /// let mut cb = CharBuffer::from_str("rust");
    /// assert!(cb.is_tar_with_progress('r'));
    /// assert_eq!(cb.position(), 1);
    /// assert!(cb.is_tar_with_progress('u'));
    /// ```
    pub fn is_tar_with_progress(&mut self, c: char) -> bool {
        if self.next_is_tar(c) {
            self.pos += 1;
            return true;
        }
        false
    }

    /// If next character is digit, return a sequence of digits as a positive integer and
    ///
    /// the `char_buffer` is progressed until the end of number, else return -1.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use ruatom::CharBuffer;
    ///
    /// assert_eq!(CharBuffer::from_str("1").to_number().unwrap(), 1);
    /// let mut cb = CharBuffer::from_str("2C");
    /// assert_eq!(cb.to_number().unwrap(), 2);
    /// assert_eq!(cb.next().unwrap(), 'C');
    /// ```
    pub fn to_number(&mut self) -> Option<usize> {
        if !self.next_is_digit() {
            return None;
        }
        let mut num = self.next_with_digit_and_progress().unwrap();
        while self.next_is_digit() {
            num = num * 10 + self.next_with_digit_and_progress().unwrap();
        }
        Some(num)
    }

    /// If next character is digit, return a sequence of specified digits from `char_buffer` as a positive integer and
    ///
    /// the `char_buffer` is progressed until the end of number, else return -1.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use ruatom::CharBuffer;
    ///
    /// let mut cb_beta = CharBuffer::from_str("002C");
    /// assert_eq!(0, cb_beta.to_sub_number(&mut 0).unwrap());
    /// assert_eq!(002, cb_beta.to_sub_number(&mut 3).unwrap());
    /// assert!(cb_beta.next_is_tar('C'));
    /// ```
    pub fn to_sub_number(&mut self, end: &mut u32) -> Option<usize> {
        if !self.next_is_digit() {
            return None;
        }
        let mut num = 0;
        while self.next_is_digit() && *end > 0 {
            num = num * 10 + self.next_with_digit_and_progress().unwrap();
            *end -= 1;
        }
        Some(num)
    }
}
