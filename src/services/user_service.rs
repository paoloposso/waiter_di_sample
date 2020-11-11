use crate::abst::*;
use waiter_di::*;
use crate::di::*;

#[module]
pub struct UserService {
    repo: Box<dyn TUserRepo>,
}

impl UserService {
    pub fn new() -> Self {
        let mut container = di_container::get::<profiles::Default>();
        Provider::<UserService>::create(&mut container)
    }

    pub fn save(&self) {
        self.repo.save();
    }
}