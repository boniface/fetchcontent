use serde::{Deserialize, Serialize};
use actix_web::web;

#[derive(Deserialize,Serialize,Debug, Clone)]
struct LinkUrl{
    pub site_url:String
}

impl From<web::Json<LinkUrl>> for LinkUrl {
    fn from(link: web::Json<LinkUrl>) -> Self {
        LinkUrl {
            site_url: link.site_url.clone(),
        }
    }
}
