use tokio::task_local;

async fn xxx(){

    tokio::task_local! {
    static NUMBER: u32;
    }

    NUMBER.scope(1, async move {
        assert_eq!(NUMBER.get(), 1);
    }).await;

    NUMBER.scope(2, async move {
        assert_eq!(NUMBER.get(), 2);

        NUMBER.scope(3, async move {
            assert_eq!(NUMBER.get(), 3);
        }).await;
    }).await;

}




#[test]
fn  dexxx(){
    xxx();
}

//std modules


//alloc


//borrow


//boxed


//cell


//char



//collections



//convert



//error



//fs


//ffi


//fs


//mem



//marker


//net



//prelude



//sync


//task



//thread



//time


//--------------常用宏

//stringify


//thread_local


//unreachable




//write



//关键词 dyn
