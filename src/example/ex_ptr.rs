


//指针类型
//pointer

#[allow(dead_code)]
fn ptr_demo(){
    let my_num: i32 = 10;

    //常量指针
    let my_num_ptr: *const i32 = &my_num;
    let mut my_speed: i32 = 88;
    //可变指针
    let my_speed_ptr: *mut i32 = &mut my_speed;


    //智能指针box
    let my_num = Box::new(20);
    let pt2 = &*my_num;

    let mut my_speed: Box<i32> = Box::new(88);
    let my_speed_ptr = &mut *my_speed;

    let s: &str = "Follow the rabbit";
    let ptr: *const u8 = s.as_ptr();
    assert!(!ptr.is_null());


    let s: &str = "123";
    let ptr1: *const u8 = s.as_ptr();

    //offset 移动指针
    unsafe {
        println!("{}", *ptr1.offset(1) as char);
        println!("{}", *ptr1.offset(2) as char);
    }

    //通过指针操作加减
    // let s: &str = "123";
    // let ptr: *const u8 = s.as_ptr();
    //
    // unsafe {
    //     println!("{}", *ptr.add(1) as char);
    //     println!("{}", *ptr.add(2) as char);
    // }
}


#[test]
fn xxx(){
    ptr_demo();
}
