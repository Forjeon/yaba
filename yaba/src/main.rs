#[macro_use] extern crate rocket;


//use bigdecimal::BigDecimal;
//use time::Date;
use diesel::sql_types::{ Date, Decimal };

use rocket::{ Rocket, Build };
use rocket::fairing::{ self, AdHoc };
use rocket_db_pools::{ Database, Connection };
use rocket_db_pools::diesel::{ prelude::*, MysqlPool, QueryResult };


#[derive(Database)]
#[database("yaba")]
struct Db(MysqlPool);


#[derive(Queryable, Insertable)]
#[diesel(table_name = Transaction)]
struct Trans {
	TransactionID: u32,
	TransactionDate: Date,
	Description: String,
	Amount: Numeric,
}

diesel::table! {
	Transaction (TransactionID) {
		TransactionID -> Unsigned<Integer>,
		TransactionDate -> Date,
		#[max_length = 200]
		Description -> VarChar,
		Amount -> Decimal,
	}
}


// GET requests
#[get("/transaction")]
fn get_trans() -> &'static str {
	"TODO: get transactions"	// TODO
}

#[get("/category")]
fn get_cat() -> &'static str {
	"TODO: get categories"	// TODO
}

#[get("/account")]
fn get_acc() -> &'static str {
	"TODO: get accounts"	// TODO
}


// POST requests


async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
	if let Some(db) = Db::fetch(&rocket) { Ok(rocket) } else { Err(rocket) }
}

#[launch]
fn rocket() -> _ {
	rocket::build()
		.attach(Db::init())
		.attach(AdHoc::try_on_ignite("DB Migrations", run_migrations))
		.mount("/", routes![get_trans, get_cat, get_acc])
}

