#![allow(dead_code)]

#[derive(Clone, Copy)]
pub struct ShortVal(pub u32);

#[derive(Clone, Copy)]
pub struct LongVal(pub u64);

impl ShortVal {
    pub fn new(value: u32) -> ShortVal {
        ShortVal(value)
    }
}

impl LongVal {
    pub fn new(value: u64) -> LongVal {
        LongVal(value)
    }
}
