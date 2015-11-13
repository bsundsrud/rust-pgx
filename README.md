# rust-pgx

Attempt to both learn Rust and also adapt something similar to the 
great [sqlx](https://github.com/jmoiron/sqlx) library for Go, taking advantage of Rust's generics.

## Example

This is adapted from sfackler's example for the [rust-postgres crate](https://github.com/sfackler/rust-postgres).

	extern crate postgres;
	#[macro_use]
	extern crate pgx;

	use postgres::{Connection, SslMode};
	use pgx::{FromRow, queryx};

	#[derive(Debug)]
	struct Person {
	    id :i32,
	    name: String,
	    data: Option<Vec<u8>>,
	}

	pgx_row!{Person, id: 0, name: 1, data: 2}

	// The above generates the following:
	// use postgres::rows::Row;
	// impl FromRow for Person {
	//     fn from_row<'a>(row: Row<'a>) -> Person {
	//         Person {
	//             id: row.get(0),
	//             name: row.get(1),
	//             data: row.get(2),
	//         }
	//     }
	// }

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



	fn main() {
	    let conn = Connection::connect("postgres://postgres@localhost", &SslMode::None).unwrap();
	    
	    insert_data(&conn);

	    let stmt = conn.prepare("SELECT id, name, data FROM person").unwrap();
	    for person in queryx::<Person>(&stmt, &[]).unwrap() {
	        println!("{:?}", person);
	    }

	    // Output is:
	    // Person { id: 1, name: "Benn", data: None }
		// Person { id: 2, name: "Steven", data: None }

	    drop_data(&conn);
	}
