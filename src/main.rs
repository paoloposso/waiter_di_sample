use services::UserService;  

mod abst;
mod services;
mod implementation;
mod di;

pub fn main() {

    let serv = UserService::new();

    serv.save();
}
