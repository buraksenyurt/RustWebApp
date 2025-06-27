// json_store modülü eğer bu feature ile build edilirse kullanılabilir
#[cfg(feature = "json-store")]
pub mod json_store;

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
