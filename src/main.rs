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
        arr![<$t>::new( {i += 1; i - 1}); $l]
    }}
}

fn main() {
    let _ = expand!([MyStruct; 33]);

    let mut i = 0;
    let _ = arr![MyStruct::new( {i += 1; i - 1}); 33];
}
