// use crate::error::{*};

#[derive(Debug)]
pub  struct Body {
    pub  name: String,
    pub  age: Box<u32>,

}

#[warn(unused_variables)]
pub trait Count<T>: Sized {
    fn into(self) -> T;
}

#[warn(unused_variables)]
trait Gif {
    fn left<A,C>(_name: A)where C:Count<A> + 'static {
        println!("sss");
    }
}

impl Body {
     fn get_lundor_bps(&mut self,) -> &str {
         &*self.name
     }
}


#[test]
fn  xx (){
    let mut bd = Body{name:"sss".to_string(), age: Box::new(32)};
    let name  = bd.get_lundor_bps();
    println!("{}",name);
}
