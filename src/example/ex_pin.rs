use std::pin::Pin;
use pin_project::pin_project;

#[pin_project]
pub(crate) struct Demo<T, U> {
    #[pin]
    pub(crate) all: T,
    pub(crate) ins: U,
}

impl<T: std::fmt::Display, U: std::fmt::Display> Demo<T, U> {
    pub(crate) fn method(self: Pin<&mut Self>) {
        let this = self.project();
        let mut a: Pin<&mut T> = this.all; // Pinned reference to the field
        let b: &mut U = this.ins; // Normal reference to the field

        //因为String 实现了Unpin,所以
        //a.as_mut() 通过as_mut 拿到被pin包裹的值
        println!("{} {}",a.as_mut(),b);
        //通过&mut 的方式拿到被pin住的值。
        println!("{} {}",&mut a,b);
    }
}

//https://folyd.com/blog/rust-pin-unpin/
use std::marker::PhantomPinned;
//Safe Rust下拿到可变借用&mut T ，实例被固定实现了!Unpin
//TAsk这个struct 整体被pin。
#[derive(Debug)]
struct Task {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

//T: AsyncWrite + Unpin,
