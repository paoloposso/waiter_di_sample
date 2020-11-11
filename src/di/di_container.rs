use crate::abst::*;
use waiter_di::*;

pub fn get<T>() -> Container<T> {
    let mut cont = Container::<T>::new();
    cont
}

fn get_cont() {
    
}