//Configure handler
use actix_web::{Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use extrablatt::{Extrablatt, Article};
use futures::{StreamExt, FutureExt};

pub async fn health_check_handler() -> impl Responder {

        HttpResponse::Ok()
        .json(get_post()
        )
}

async fn get_post() -> Article {
    let article = extrablatt::Article::get("https://www.znbc.co.zm/news/shona-ferguson-has-died/")
        .unwrap();
    article


}

#[derive(Deserialize,Serialize,)]
struct Name{
    first_name:String,
    last_name: String
}

#[derive(Deserialize,Serialize,)]
struct Demographics{
    gender:String,
    race:String
}

#[derive(Deserialize,Serialize,)]
struct User{
    id:u32,
    names: Name,
    demographics:Demographics
}

fn get_person()-> User{
    let name = Name{
        first_name: String::from("John"),
        last_name: String::from("Banda")
    };
    let demographics = Demographics{
        gender:String::from("MALE"),
        race:String::from("BLACK")
    };
    return User{ id:1,names:name,demographics:demographics};
}