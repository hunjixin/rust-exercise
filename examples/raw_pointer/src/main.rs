use std::ops::AddAssign;

fn main() {
    let mut my_speed: Box<i32> = Box::new(88);
    let my_speed_ptr: *mut i32 = &mut *my_speed; //not take owner ship panic on drop
                                                 // let my_speed_ptr: *mut i32 = Box::into_raw(my_speed);  //take owner ship panic on drop
    unsafe {
        println!("{}, {}", my_speed, Box::from_raw(my_speed_ptr));
    }
}
