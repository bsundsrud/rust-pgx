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

fn insert_data(conn: &Connection) {
    conn.execute("CREATE TABLE person (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR NOT NULL,
                    data            BYTEA
                  )", &[]).unwrap();
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
    conn.execute("INSERT INTO person (name, data) VALUES ($1, $2)",
                 &[&me.name, &me.data]).unwrap();
    conn.execute("INSERT INTO person (name, data) VALUES ($1, $2)",
                 &[&him.name, &him.data]).unwrap();
}

fn drop_data(conn: &Connection) {
    conn.execute("DROP TABLE person;", &[]).unwrap();
}
#[test]
fn run_test() {
    let conn = Connection::connect("postgres://postgres:postgres@localhost/pgx", &SslMode::None).unwrap();
    
    insert_data(&conn);

    let stmt = conn.prepare("SELECT id, name, data FROM person").unwrap();
    let people = queryx::<Person>(&stmt, &[]).unwrap().collect::<Vec<Person>>();
    assert_eq!(people[0].name, "Benn");
    assert_eq!(people[1].name, "Steven");

    drop_data(&conn);
}
