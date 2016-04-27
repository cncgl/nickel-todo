extern crate rustc_serialize;
extern crate hyper;

//use rustc_serialize::json::{Json, Parser};
use rustc_serialize::{json};
#[macro_use] extern crate nickel;

use nickel::status::StatusCode;
use nickel::{Nickel, HttpRouter, MediaType};

#[derive(RustcDecodable, RustcEncodable)]
struct Todo {
  status: i32,
  title: String,
}

fn main() {
  // println!("Hello, world!");
  let mut server = Nickel::new();
  let mut router = Nickel::router();

  //server.utilize(router! {
  //  get "**" => |_req, _res| {
  //    "Hello World!"
  //  }
  //});
  router.get("/api/todos", middleware! { |_, mut _res|
    let mut v: Vec<Todo> = vec![];
    let todo1 = Todo{
      status: 0,
      title: "Shopping".to_string(),
    };
    v.push(todo1);
    let todo2 = Todo{
      status: 1,
      title: "Movie".to_string(),
    };
    v.push(todo2);

    let json_obj = json::encode(&v).unwrap();
    _res.set(MediaType::Json);
    _res.set(StatusCode::Ok);
    return _res.send(json_obj);
  });

  server.utilize(router);
  server.listen("127.0.0.1:6767");
}
