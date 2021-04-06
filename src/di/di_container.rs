use waiter_di::*;

pub fn get<T>() -> Container<T> {
    let cont = Container::<T>::new();
    cont
}
