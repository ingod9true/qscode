



//线程安全传值demo

#[allow(dead_code)]
pub(crate) struct SyncWrapper<T>(T);

#[allow(dead_code)]
impl<T> SyncWrapper<T> {
    #[allow(dead_code)]
    pub(crate) fn new(value: T) -> Self {
    Self(value)
}

    //    /// let mut wrapped = SyncWrapper::new(42);
    //     /// let value = wrapped.get_mut();
    //     /// *value = 0;
    //     /// assert_eq!(*wrapped.get_mut(), 0);
    #[allow(dead_code)]
    pub(crate) fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }
    //获取值
    /// let mut wrapped = SyncWrapper::new(42);
    /// assert_eq!(wrapped.into_inner(), 42);
    #[allow(dead_code)]
    pub(crate) fn into_inner(self) -> T {
        self.0
    }
}

//Send Sync
unsafe impl<T: Send> Sync for SyncWrapper<T> {}


//标记属性，如果实际上数据不满足要求，出问题还是程序员背锅
//实现Send 特性的数据可以跨线程传输Send 表示数据能安全地被 move 到另一个线程
//实现Sync 特性的数据可以跨线程传输 Sync 表示数据能在多个线程中被同时安全地访问
