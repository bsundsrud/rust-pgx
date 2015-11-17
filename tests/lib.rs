#![feature(test)]
#![feature(convert)]
extern crate test;
use test::Bencher;

extern crate postgres;
#[macro_use]
extern crate pgx;

use postgres::{Connection, SslMode};
use postgres::rows::Row;
use pgx::{FromRow, queryx};

#[derive(Debug)]
struct Person {
    id :i32,
    name: String,
    data: Option<Vec<u8>>,
}

pgx_row!{Person, id: 0, name: 1, data: 2}

fn insert_data(conn: &Connection, table_name: &str) {

    conn.execute(format!("CREATE TABLE {} (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR NOT NULL,
                    data            BYTEA
                  )", table_name).as_str(), &[]).unwrap();
    let him = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    let me = Person {
        id: 0,
        name: "Benn".to_string(),
        data: None,
    };
    conn.execute(format!("INSERT INTO {} (name, data) VALUES ($1, $2)", table_name).as_str(),
                 &[&me.name, &me.data]).unwrap();
    conn.execute(format!("INSERT INTO {} (name, data) VALUES ($1, $2)", table_name).as_str(),
                 &[&him.name, &him.data]).unwrap();
}


fn drop_data(conn: &Connection, table_name: &str) {
    conn.execute(format!("DROP TABLE {};", table_name).as_str(), &[]).unwrap();
}


    
#[test]
fn run_test() {
    let conn = Connection::connect("postgres://postgres:postgres@localhost/pgx", &SslMode::None).unwrap();
    let table_name = "Person_run_test";
    insert_data(&conn, table_name);

    let stmt = conn.prepare(format!("SELECT id, name, data FROM {}", table_name).as_str()).unwrap();
    let people = queryx::<Person>(&stmt, &[]).unwrap().collect::<Vec<Person>>();
    assert_eq!(people[0].name, "Benn");
    assert_eq!(people[1].name, "Steven");

    drop_data(&conn, table_name);
}

#[bench]
fn run_bench_standard(b: &mut Bencher) {
    let conn = Connection::connect("postgres://postgres:postgres@localhost/pgx", &SslMode::None).unwrap();
    let table_name = "Person_run_bench_standard";
    insert_data(&conn, table_name);
    
    let stmt = conn.prepare(format!("SELECT id, name, data FROM {}", table_name).as_str()).unwrap();

    b.iter(|| {
        let mut v = vec!();
        for row in stmt.query(&[]).unwrap() {
            let person = Person {
                id: row.get(0),
                name: row.get(1),
                data: row.get(2)
            };
            v.push(person);
        }
        v
    });
    drop_data(&conn, table_name);
}

#[bench]
fn run_bench_pgx(b: &mut Bencher) {
    let conn = Connection::connect("postgres://postgres:postgres@localhost/pgx", &SslMode::None).unwrap();
    let table_name = "Person_run_bench_pgx";
    insert_data(&conn, table_name);

    let stmt = conn.prepare(format!("SELECT id, name, data FROM {}", table_name).as_str()).unwrap();

    b.iter(|| {
        let mut v = vec!();
        for person in queryx::<Person>(&stmt, &[]).unwrap() {
            v.push(person);
        }
        v
    });

    drop_data(&conn, table_name);
}
