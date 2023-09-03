// 需要reqwest crate
#[macro_export]
macro_rules! headermap {
    ($( $key: expr => $val: expr ),*) => {{
        let mut map = reqwest::header::HeaderMap::new();
        $( map.insert($key, reqwest::header::HeaderValue::from_static($val)); )*
        map
    }}
}

// macro_rules! hashmap {
//     ($( $key: expr => $val: expr ),*) => {{
//         let hashmap = std::collections::HashMap::new();
//         $( hashmap.insert($key, $val); )*
//         hashmap
//     }}
// }

