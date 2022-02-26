pub mod id;
#[cfg(feature = "yew-wasm")]
pub mod key;
pub mod markup;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
