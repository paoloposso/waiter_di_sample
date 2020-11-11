use crate::abst::TUserRepo;
use waiter_di::*;

#[component]
pub struct UserRepo {}

#[provides]
impl TUserRepo for UserRepo {
    fn save(&self) {
        println!("User saved!");
    }
}
