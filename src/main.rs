#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::Status;

#[get("/")]
fn index() -> Status {
    use git2::Repository;

    let url = "https://github.com/npex42/website";
    match Repository::open("repos/website") {
        Ok(repo) => {
            match repo.find_remote("origin") {
                Ok(mut remote) => {
                    if let Ok(_) = remote.fetch(&["prod"], None, None) {
                        println!("Fetched From {}", remote.name().unwrap());
                        return Status::Ok
                    } else {
                        return Status::Forbidden
                    }
                },
                Err(_) => {return Status::NotFound}
            }
            return Status::Ok
        },
        Err(e) => return Status::NotFound,
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}