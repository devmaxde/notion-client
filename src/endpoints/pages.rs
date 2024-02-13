use std::rc::Rc;

use reqwest::Client;

pub mod create;
pub mod retrieve;

pub struct PagesEndpoint {
    pub(super) client: Rc<Client>,
}