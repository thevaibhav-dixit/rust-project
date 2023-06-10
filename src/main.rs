fn add_number(a: i32, b: i32) -> i32 {
    a + b
}
fn main() {
    let ans = add_number(12, 12);
    println!("The answer is {}", ans);
}
fn multiplY(a : i32 , b : i32) -> i32 {
    a*b
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_number() {
        assert_eq!(add_number(12, 12), 24);
    }
    fn test(){
        assert_eq!(12, 12);
    }
