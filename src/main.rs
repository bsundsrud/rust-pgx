extern crate postgres;

use postgres::{Connection, Statement, SslMode};
use postgres::rows::{Row, IntoIter};
use postgres::types::ToSql;

use std::marker::PhantomData;
use std::iter::Iterator;

#[derive(Debug)]
struct Person {
    id :i32,
    name: String,
    data: Option<Vec<u8>>,
}

impl<'a> From<Row<'a>> for Person {
    fn from(row: Row<'a>) -> Person {
        Person {
            id: row.get(0),
            name: row.get(1),
            data: row.get(2)
        }
    }
}

struct RowIterator<'a, T> {
    _marker: PhantomData<T>,
    iter: IntoIter<'a>
}

impl<'a, T> Iterator for RowIterator<'a, T>
where T: From<Row<'a>> {
    type Item = T;
    fn next(&mut self) -> Option<T> {

        match self.iter.next() {
            Some(a) => Some(T::from(a)),
            _ => None,
        }
    }
}

fn insert_data(conn: &Connection) {
    conn.execute("CREATE TABLE person (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR NOT NULL,
                    data            BYTEA
                  )", &[]).unwrap();
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute("INSERT INTO person (name, data) VALUES ($1, $2)",
                 &[&me.name, &me.data]).unwrap();
}

fn queryx<'a, T>(stmt: &'a Statement, args: &[&ToSql])
  -> RowIterator<'a, T>
  where T: From<Row<'a>> {
    let rows = stmt.query(args).unwrap();
    let iter = rows.into_iter();
    RowIterator {
        iter: iter,
        _marker: PhantomData
    }
}

fn main() {
    let conn = Connection::connect("postgres://bsundsrud@localhost/pgx", &SslMode::None).unwrap();
    
    //insert_data(&conn);

    let stmt = conn.prepare("SELECT id, name FROM person").unwrap();
    for person in queryx::<Person>(&stmt, &[]) {
        println!("{:?}", person);
    }
}
