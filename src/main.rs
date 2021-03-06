extern crate rustc_serialize;
extern crate hyper;

use rustc_serialize::json;
use rustc_serialize::json::Json;
#[macro_use] extern crate nickel;

use nickel::status::StatusCode;
use nickel::{Nickel, HttpRouter, MediaType, JsonBody};

#[derive(RustcDecodable, RustcEncodable)]
struct Todo {
  status: i32,
  title: String,
}

fn main() {
  let mut server = Nickel::new();
  let mut router = Nickel::router();

  // index
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

  // detail
  router.get("/api/todos/:id", middleware! { |_req, mut _res|
    let id = _req.param("id");
    println!("id: {:?}", id);

    let todo1 = Todo{
      status: 0,
      title: "Shopping".to_string(),
    };
    let json_obj = json::encode(&todo1).unwrap();
    _res.set(MediaType::Json);
    _res.set(StatusCode::Ok);
    return _res.send(json_obj);
  });

  // delete
  router.delete("/api/todos/:id", middleware! { |_req, mut _res|
    let id = _req.param("id");
    println!("delete id: {:?}", id);

    let json_obj = Json::from_str("{}").unwrap();
    _res.set(MediaType::Json);
    _res.set(StatusCode::Ok);
    return _res.send(json_obj);
  });

  // create
  router.post("/api/todos", middleware! { |_req, mut _res|
    let todo   = _req.json_as::<Todo>().unwrap();
    let status = todo.status;
    let title  = todo.title.to_string();
    println!("create status {:?} title {}", status, title);

    let json_obj = Json::from_str("{}").unwrap();
    _res.set(MediaType::Json);
    _res.set(StatusCode::Created);
    return _res.send(json_obj);
  });

  // update
  router.put("/api/todos/:id", middleware! { |_req, mut _res|
    let todo  = _req.json_as::<Todo>().unwrap();
    let status = todo.status;
    let title  = todo.title.to_string();
    println!("update status {:?} title {}", status, title);

    let json_obj = Json::from_str("{}").unwrap();
    _res.set(MediaType::Json);
    _res.set(StatusCode::Ok);
    return _res.send(json_obj);
  });

  server.utilize(router);
  server.listen("127.0.0.1:6767");
}
