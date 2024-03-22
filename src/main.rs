fn main() {
    println!("{result}", result = add(1,2));
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b + 1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1,2), 3);
    }
}