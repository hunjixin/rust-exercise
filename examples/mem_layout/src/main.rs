#![feature(ptr_alignment_type)]
use std::mem::size_of;
struct Example {
    a: u8,  //1
    b: u32, //4
    c: u16, //2
}

#[repr(C)]
struct Example2 {
    a: u8,  //1
    b: u32, //4
    c: u16, //2
}

fn main() {
    {
        let v = std::ptr::Alignment::of::<Example>();
        println!("algn {:?} size {:?}", v.as_usize(), size_of::<Example>()); //algn 4 size 8         // a(1byte)|c)(2bytes)|pad(1byte)|b(4byte)
    }
    {
        let v = std::ptr::Alignment::of::<Example2>();
        println!("algn {:?} size {:?}", v.as_usize(), size_of::<Example2>()); //algn 4 size 8         // a(4byte)|b(2bytes)||c(4byte)
    }
}
