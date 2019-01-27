/// Returns the line number and column number tuple given a byte offset into a string
pub fn get_col_line_number(s: &str, byte_offset: usize) -> (i32, i32) {
    let mut line_count = 1;
    let mut col_count = 1;
    let mut counter = 0;
    if s == "" {
        return (-1, -1);
    }
    for (i, c) in s.chars().enumerate() {
        counter = i;
        if i == byte_offset || i > byte_offset {
            break;
        } else if c == '\n' {
            line_count += 1;
            col_count = 0;
        }
        col_count += 1
    }
    println!("ret: {:?}", (byte_offset, counter));
    if byte_offset == counter {
        (col_count, line_count)
    } else {
        (-1, -1)
    }
}

/// Gets the line number given a byte offset into a string
pub fn get_line_number(s: &str, byte_offset: usize) -> i32 {
    get_col_line_number(s, byte_offset).1
}

/// Gets the column number given a byte offset into a string
pub fn get_column_number(s: &str, byte_offset: usize) -> i32 {
    get_col_line_number(s, byte_offset).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_line_number() {
        assert_eq!(get_line_number(&"foo\nbar".to_string(), 0), 1, "first line");
        assert_eq!(
            get_line_number(&"foo\nbar".to_string(), 4),
            2,
            "second line"
        );
        assert_eq!(
            get_line_number(&"foo\nbar".to_string(), 3),
            1,
            "line number of newline"
        );
        assert_eq!(get_line_number(&"foo".to_string(), 3), -1, "out of bounds");
        assert_eq!(get_line_number(&"".to_string(), 0), -1, "empty string");
    }

    #[test]
    fn test_get_column_number() {
        assert_eq!(
            get_column_number(&"foo\nbar".to_string(), 0),
            1,
            "first line"
        );
        assert_eq!(
            get_column_number(&"foo\nbar".to_string(), 4),
            1,
            "second line"
        );
        assert_eq!(
            get_column_number(&"foo\nbar".to_string(), 5),
            2,
            "second column on second line"
        );
        assert_eq!(
            get_column_number(&"foo\nbar".to_string(), 3),
            4,
            "col number of newline"
        );
        assert_eq!(get_column_number(&"".to_string(), 0), -1, "empty string");
        assert_eq!(
            get_column_number(&"foo".to_string(), 3),
            -1,
            "out of bounds"
        );
    }
}
