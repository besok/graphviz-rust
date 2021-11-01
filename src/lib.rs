mod dot_lang;

#[macro_use]
extern crate pest_derive;
extern crate pest;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
