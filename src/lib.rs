extern crate postgres;

use postgres::stmt::Statement;
use postgres::types::ToSql;
use postgres::rows::{Row, Rows};
use postgres::error::Error;

use std::marker::PhantomData;
use std::iter::Iterator;

pub trait FromRow {
    fn from_row<'a>(row: &Row<'a>) -> Self;
}

pub struct RowIterator<'a, T>
    where T: FromRow
{
    _marker: PhantomData<T>,
    rows: Rows<'a>,
    index: usize,
}


impl<'a, T> Iterator for RowIterator<'a, T> where
    T: FromRow
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.index < self.rows.len() {
            let row = self.rows.get(self.index);
            let result = T::from_row(&row);
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

pub fn queryx<'a, T>(stmt: &'a Statement, args: &[&ToSql]) -> Result<RowIterator<'a, T>, Error>
    where T: FromRow
{
    Ok(RowIterator {
        rows: try!(stmt.query(args)),
        _marker: PhantomData,
        index: 0,
    })
}

#[macro_export]
macro_rules! pgx_row {
	(
		$type_name: ident,
		$( $field: ident : $idx: expr),*
	) => {
		impl FromRow for $type_name {
			fn from_row<'a>(row: &Row<'a>) -> $type_name {
		        $type_name {
		            $(
		            	$field: row.get($idx),
		            )*
		        }
		    }
		}
	}
}
