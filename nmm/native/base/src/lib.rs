pub fn base_hello() -> String {
    String::from("Hello from base")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
