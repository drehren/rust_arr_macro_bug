use arr_macro::arr;

struct MyStruct {
    data: u32,
}

impl MyStruct {
    fn new(data: u32) -> Self {
        Self { data: data }
    }
}

macro_rules! expand{
    ([$t:ty; $l:literal]) => {{
        let mut i = 0;
        arr![<$t>::new({i += 1; i - 1}); $l]
    }}
}

macro_rules! expand2 {
    ([$t:ty; $l:literal]) => {
        fn build_arr2() -> [$t; $l] {
            let mut i = 0;
            arr![<$t>::new({ i += 1; i - 1}); $l]
        }
    };
}

expand_derive::create_build_array!([MyStruct; 34]);

expand2!([MyStruct; 34]);

fn main() {
    let _ = expand!([MyStruct; 33]);

    let _ = build_array();

    let _ = build_arr2();

    let mut i = 0;
    let _ = arr![MyStruct::new({i += 1; i - 1}); 35];
}
