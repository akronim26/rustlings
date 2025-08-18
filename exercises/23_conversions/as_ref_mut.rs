fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().len()
}

fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    let x: &mut u32 = arg.as_mut();
    *x = (*x) * (*x);
}

fn main() {
    let s = "Café au lait";
    println!("bytes: {}, chars: {}", byte_counter(s), char_counter(s));

    let mut n: Box<u32> = Box::new(5);
    num_sq(&mut n);
    println!("squared: {}", n);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
