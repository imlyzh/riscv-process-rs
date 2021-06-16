
#[derive(Debug, Clone, Copy)]
pub struct Pointer(pub u32, pub u32);

#[derive(Debug, Clone, Copy)]
pub struct U32(pub u32, pub u32);

#[derive(Debug, Clone, Copy)]
pub struct I32(pub i32, pub i32);

#[derive(Debug, Clone, Copy)]
pub struct F32(pub f32, pub f32);


macro_rules! gen_methods {
    ($t:ident, $it:ident) => {
        impl $t {
            pub fn new(i: $it) -> Self {
                Self(i, i)
            }
            pub fn from(a: $it, b: $it) -> Self {
                Self(a, b)
            }
        }
        
        impl Default for $t {
            fn default() -> Self {
                Self($it::MIN, $it::MAX)
            }
        }
    };
}

gen_methods!(Pointer, u32);
gen_methods!(U32, u32);
gen_methods!(I32, i32);
gen_methods!(F32, f32);
