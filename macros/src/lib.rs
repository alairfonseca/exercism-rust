#[macro_export]
macro_rules! hashmap {
    () => {{
         ::std::collections::HashMap::new()
    }};
    ($($i:expr => $j:expr),+ $(,)?) => {
        {
            let mut hm = ::std::collections::HashMap::new();

            $(
                hm.insert($i, $j);
            )*
        
            hm
        }
    }
}
