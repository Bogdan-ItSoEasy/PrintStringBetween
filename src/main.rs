use std::env;
use std::str;

//Used for simple agrs parsing
use stackvec::TryCollect;

const FIRST_SYMBOL: u8 = 'a' as u8;
const LAST_SYMBOL: u8 = 'z' as u8;

fn for_each_between<F: FnMut(&mut Vec<u8>)>(begin: String, end: String, mut f: F) {
    for i in begin.len()..=end.len() {
        let end = if i == end.len() {
            Some(end.as_bytes())
        } else {
            None
        };

        let mut begin = if i == begin.len() {
            begin.clone().into_bytes()
        } else {
            vec!['a' as u8; i]
        };

        for_each_between_same_length(&mut begin, end, 0, 0, &mut f);
    }
}

fn for_each_between_same_length<F: FnMut(&mut Vec<u8>)>(
    current: &mut Vec<u8>,
    end: Option<&[u8]>,
    index: usize,
    finished_index: usize,
    f: &mut F,
) {
    if index >= current.len() {
        f(current);
        return;
    }

    let end_index = match end {
        None => LAST_SYMBOL,
        Some(end) => {
            if index > finished_index {
                LAST_SYMBOL
            } else {
                end[index]
            }
        }
    };

    while current[index] <= end_index {
        let finished_index = if current[index] == end_index {
            finished_index + 1
        } else {
            finished_index
        };

        for_each_between_same_length(current, end, index + 1, finished_index, f);

        current[index] = current[index] + 1;
        if index < current.len() - 1 {
            current[index + 1] = FIRST_SYMBOL;
        }
    }
}

//Run example: cargo run -- ab xyz
fn main() {
    let [begin, end] = env::args()
        .skip(1)
        .try_collect::<[String; 2]>()
        .expect("Usage: cargo run -- begin_string end_string");

    for_each_between(begin, end, |x: &mut Vec<u8>| {
        println!("{}", str::from_utf8(x).unwrap())
    });
}

#[cfg(test)]
mod tests {
    use crate::{for_each_between, FIRST_SYMBOL, LAST_SYMBOL};
    use std::str;

    const DELTA: usize = (LAST_SYMBOL - FIRST_SYMBOL) as usize + 1;

    #[test]
    fn test_one_char_length() {
        let mut count = 0;
        for_each_between("a".to_string(), "z".to_string(), |_: &mut Vec<u8>| {
            count += 1
        });
        assert_eq!(DELTA, count);
    }

    #[test]
    fn test_one_char_length_from_middle() {
        let mut count = 0;
        for_each_between("n".to_string(), "z".to_string(), |_: &mut Vec<u8>| {
            count += 1
        });
        assert_eq!((LAST_SYMBOL - 'n' as u8 + 1) as usize, count);
    }

    #[test]
    fn test_two_char_length() {
        let mut count = 0;
        for_each_between("aa".to_string(), "zz".to_string(), |_: &mut Vec<u8>| {
            count += 1
        });
        assert_eq!(DELTA * DELTA, count);
    }

    #[test]
    fn test_tree_char_length() {
        let mut count = 0;
        for_each_between("abc".to_string(), "xyz".to_string(), |_: &mut Vec<u8>| {
            count += 1
        });
        assert_eq!(
            count,
            DELTA * DELTA * DELTA
                - ('b' as u8 - FIRST_SYMBOL) as usize * DELTA
                - ('c' as u8 - FIRST_SYMBOL) as usize
                - (LAST_SYMBOL - 'y' as u8) as usize * DELTA
                - (LAST_SYMBOL - 'x' as u8) as usize * DELTA * DELTA
        );
    }

    #[test]
    fn test_from_one_to_two_char_length() {
        let mut count = 0;
        for_each_between("a".to_string(), "zz".to_string(), |_: &mut Vec<u8>| {
            count += 1
        });
        assert_eq!(DELTA + DELTA * DELTA, count);
    }

    #[test]
    fn test_from_one_to_tree_char_length() {
        let mut count = 0;
        for_each_between("d".to_string(), "xyz".to_string(), |_: &mut Vec<u8>| {
            count += 1
        });
        assert_eq!(
            count,
            (LAST_SYMBOL - 'd' as u8 + 1) as usize + DELTA * DELTA + DELTA * DELTA * DELTA
                - (LAST_SYMBOL - 'y' as u8) as usize * DELTA
                - (LAST_SYMBOL - 'x' as u8) as usize * DELTA * DELTA
        );
    }

    #[test]
    fn test_empty_string() {
        let mut result = vec![];
        for_each_between("".to_string(), "d".to_string(), |x: &mut Vec<u8>| {
            result.insert(result.len(), str::from_utf8(x).unwrap().to_string())
        });
        assert_eq!(vec!["", "a", "b", "c", "d"], result);
    }

    #[test]
    fn test_switch_length() {
        let mut result = vec![];
        for_each_between("zzx".to_string(), "aaab".to_string(), |x: &mut Vec<u8>| {
            result.insert(result.len(), str::from_utf8(x).unwrap().to_string())
        });
        assert_eq!(vec!["zzx", "zzy", "zzz", "aaaa", "aaab"], result);
    }

    #[test]
    fn test_wrong_length() {
        let mut count = 0;
        for_each_between("aaaa".to_string(), "zzz".to_string(), |_: &mut Vec<u8>| {
            count += 1
        });
        assert_eq!(0, count);
    }

    #[test]
    fn test_wrong_order() {
        let mut count = 0;
        for_each_between("zzz".to_string(), "aaa".to_string(), |_: &mut Vec<u8>| {
            count += 1
        });
        assert_eq!(0, count);
    }

    #[test]
    fn test_same_string() {
        let mut result = vec![];
        for_each_between("aaaa".to_string(), "aaaa".to_string(), |x: &mut Vec<u8>| {
            result.insert(result.len(), str::from_utf8(x).unwrap().to_string())
        });
        assert_eq!(vec!["aaaa"], result);
    }
}
