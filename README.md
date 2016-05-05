# nickel-todo

A toto application for Nickel on Rust

[![Build Status](https://travis-ci.org/cncgl/nickel-todo.svg?branch=master)](https://travis-ci.org/cncgl/nickel-todo)


## Prepare Database
Create user ``postgres`` with password ``postgres`` on PostgreSQL.

Install Diesel cli
```
$ cargo install diesel_cli
$ diesel setup
$ diesel migration run
```

## Install
```
$ cargo build --verbose
```

## Usage
debug build
```
$ cargo run --verbose
```

release build
```
$ cargo run --release --verbose
```

## Test
```
$ cargo test --verbose
```

## API
index
```
$ curl -s http://lvh.me:6767/api/todos | jq
```

show
```
$ curl -s http://lvh.me:6767/api/todos/1 | jq
```

store
```
$ curl -s http://lvh.me:6767/api/todos -H "Content-type: application/json" \
-X POST -d '{"status":1, "title":"Meeting"}'
```

update
```
$ curl -s http://lvh.me:6767/api/todos/1 -H "Content-type: application/json" \
-X PUT -d '{"status":1, "title":"Meeting"}'
```

destroy
```
$ curl -s http://lvh.me:6767/api/todos/1 -X DELETE | jq
```

## License

[MIT](LICENSE)
