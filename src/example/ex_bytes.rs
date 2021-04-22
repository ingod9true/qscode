//traits
//buf bufmut
use bytes::{buf, BufMut};
//struct
use bytes::{BytesMut, Bytes};


pub(crate) fn tt_buf(){
    let mut buf = BytesMut::with_capacity(1024);
    buf.put(&b"hello world"[..]);
    buf.put(&b"zzz"[..]);
    let a = buf.split();
    assert_eq!(a, b"hello worldzzz"[..]);


    //通过from特性初始化

    let mut by = Bytes::from("ssssssssssss");

    println!("{:?}",by);

   // Bytes 本身方法
    by.clear();//remove all data;
    let mut by1  = Bytes::copy_from_slice(&b"hello world"[..3]);
    by1.clear();
    by1.is_empty();//判断空

    let mut by4 = Bytes::from("ssssssssssss");

    //切割
    by4.split_to(3);

    //new 产生空集合
    let b = Bytes::new();
    assert_eq!(&b[..], b"");

    let a = Bytes::from(&b"hello world"[..]);
    //slice 取切片
    let b = a.slice(2..5);

    //Bytes 类似数组

    //取切片，
    let bytes = Bytes::from(&b"012345678"[..]);
    let as_slice = bytes.as_ref();
    let subset = &as_slice[2..6];
    let subslice = bytes.slice_ref(&subset);
    assert_eq!(&subslice[..], b"2345");

    //truncate 删除末尾几个item


    //来自deref的方法

    unsafe {
        let bytes: [u8; 7] = [1, 2, 3, 4, 5, 6, 7];
        let (prefix, shorts, suffix) = bytes.align_to::<u16>();
        // less_efficient_algorithm_for_bytes(prefix);
        // more_efficient_algorithm_for_aligned_shorts(shorts);
        // less_efficient_algorithm_for_bytes(suffix);
        println!("{:?}",shorts);
    }

   // Returns a raw pointer to the slice's buffer.
    let x = &[1, 2, 4];
    let x_ptr = x.as_ptr();//返回指向第一个数据的指针If you need to mutate the contents of the slice, use as_mut_ptr.

    //windows 滑动的窗口
    let slice = ['r', 'u', 's', 't'];
    let mut iter = slice.windows(2);
    assert_eq!(iter.next().unwrap(), &['r', 'u']);
    assert_eq!(iter.next().unwrap(), &['u', 's']);
    assert_eq!(iter.next().unwrap(), &['s', 't']);
    assert!(iter.next().is_none());



    //通用特性
    //as_ref
    let s = "hello";
    is_hello(s);


    let s = "hello".to_string();
    is_hello(s);


    let mut boxed_num = Box::new(0);
    add_one(&mut boxed_num);
    assert_eq!(*boxed_num, 1);

    //buf 特性

    //remaining 还有多少
    //chunk 从头开始，获取第一个字符块
    //advance 从头开始，吃掉多少


    use bytes::Buf;

    let mut buf = &b"hello world"[..];
    let mut dst = [0; 5];

    //复制进slice切片,然后原来的位置删除。首指针位置移动了。。
    //hyper的buf里面可以恢复。。
    buf.copy_to_slice(&mut dst);
    assert_eq!(&b"hello"[..], &dst);
    assert_eq!(6, buf.remaining());


// hyper common io rewind (支持回滚功能)

}
//T 实现了调用as_ref方法的时候获得是他的引用
fn is_hello<T: AsRef<str>>(s: T) {
    assert_eq!("hello", s.as_ref());
}

//box 实现的* 直接拿到值
fn add_one<T: AsMut<u64>>(num: &mut T) {
//as_mut 优先级高于*
    *(num.as_mut()) += 1;
}
#[test]
fn fxe(){
    tt_buf();
}



// use std::borrow::Borrow;
// use std::hash::Hash;
//
// pub struct HashMap<K, V> {
//     // fields omitted
// }
//
// impl<K, V> HashMap<K, V> {
//
// //Borrow 意思等于 &
//     pub fn get<Q>(&self, k: &Q)
//         where
//             K: Borrow<Q>,
//             Q: Hash + Eq + ?Sized
//     {
//         // ...
//     }
// }
