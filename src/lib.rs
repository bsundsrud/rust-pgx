extern crate postgres;

use postgres::stmt::Statement;
use postgres::types::ToSql;
use postgres::rows::{Row, IntoIter};
use postgres::error::Error;

use std::marker::PhantomData;
use std::iter::Iterator;

pub trait FromRow {
	fn from_row<'a>(row: Row<'a>) -> Self;
}

pub struct RowIterator<'a, T> {
    _marker: PhantomData<T>,
    iter: IntoIter<'a>
}

impl<'a, T> Iterator for RowIterator<'a, T>
where T: FromRow {
    type Item = T;
    fn next(&mut self) -> Option<T> {
		match self.iter.next() {
            Some(a) => Some(T::from_row(a)),
            _ => None,
        }
    }
}

pub fn queryx<'a, T>(stmt: &'a Statement, args: &[&ToSql]) -> Result<RowIterator<'a, T>, Error>
		where T: FromRow {
	let rows = try!(stmt.query(args));
	let iter = rows.into_iter();
	Ok(RowIterator {
	    iter: iter,
	    _marker: PhantomData
	})
}

#[macro_export]
macro_rules! pgx_row {
	(
		$type_name: ident,
		$( $field: ident : $idx: expr),*
	) => {
		use postgres::rows::Row;
		impl FromRow for $type_name {
			fn from_row<'a>(row: Row<'a>) -> $type_name {
		        $type_name {
		            $(
		            	$field: row.get($idx),
		            )*
		        }
		    }
		}
	}
}