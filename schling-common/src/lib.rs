pub mod id;
#[cfg(feature = "yew-wasm")]
pub mod invoke;
#[cfg(feature = "yew-wasm")]
pub mod key;
pub mod markup;
#[cfg(feature = "yew-wasm")]
pub mod source;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
