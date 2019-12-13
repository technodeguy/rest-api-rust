use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use serde::{ Serialize, Deserialize };

use crate::schema::book;
use book::dsl:: { book as book_model, author as a, title as t };
use crate::dto::NewBookDto;

#[derive(Queryable, Debug, Serialize, Clone)]
pub struct Book {
  pub id: i32,
  pub title: String,
  pub author: String,
  pub is_published: bool
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "book"]
pub struct NewBook {
  pub title: String,
  pub author: String,
  pub is_published: bool
}

impl From<NewBookDto> for NewBook {
  fn from(i: NewBookDto) -> NewBook {
    NewBook {
      title: i.title,
      author: i.author,
      is_published: i.is_published
    }
  }
}

impl Book {
  pub fn find_by_id(id: &i32, db_connection: &MysqlConnection) -> Vec<Book> {
    book_model.find(*id).load::<Book>(db_connection).expect("Error retrieving by id")
  }

  pub fn find_by_author(author: String, db_connection: &MysqlConnection) -> Vec<Book> {
    book_model.filter(book::author.eq(author)).load::<Book>(db_connection).expect("Error loading books by author")
  }

  pub fn find_all(db_connection: &MysqlConnection) -> Vec<Book> {
    book_model.load::<Book>(db_connection).expect("Error loading all books")
  }

  pub fn find_all_ordered(db_connection: &MysqlConnection) -> Vec<Book> {
    book_model.order(book::id.desc()).load::<Book>(db_connection).expect("Error loading all books")
  }

  pub fn insert(newBook: &NewBook, db_connection: &MysqlConnection) -> bool {
    diesel::insert_into(book_model).values(newBook).execute(db_connection).is_ok()
  }

  pub fn update(id: i32, book: NewBook, db_connection: &MysqlConnection) -> bool {
  
    diesel::update(book_model.find(id))
      .set((a.eq(book.author), t.eq(book.title)))
      .execute(db_connection)
      .is_ok()
  }

  pub fn delete_by_id(id: i32, db_connection: &MysqlConnection) -> bool {
    // if Book::find_by_id(&id, db_connection).is_empty() {
    //   return false;
    // };

    diesel::delete(book_model.find(id)).execute(db_connection).is_ok()
  }


}