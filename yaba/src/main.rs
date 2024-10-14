#[macro_use] extern crate rocket;


use rocket::{ Rocket, Build, fs::NamedFile, response::Redirect };
use rocket::fairing::{ self, AdHoc };
use rocket_db_pools::{ Database, Connection };
use rocket_db_pools::diesel::{ prelude::*, MysqlPool, QueryResult };

use yaba::schema::*;
use yaba::models::*;


#[derive(Database)]
#[database("yaba")]
struct Db(MysqlPool);


// Yaba pages
#[get("/")]
fn index() -> Redirect {
	Redirect::to(uri!("/home"))
}

#[get("/home")]
async fn home() -> Option<NamedFile> {
	NamedFile::open("webpages/index.html").await.ok()
}

#[get("/transaction")]
fn get_trans() -> &'static str {
	"TODO: get transactions"	// TODO
}

// GET requests
#[get("/category")]
async fn get_cat(mut db: Connection<Db>) -> String {
	use yaba::schema::TransactionCategory::dsl::*;

	let results = TransactionCategory
		.select(TransCat::as_select())
		.load(&mut db)
		.await
		.expect("Error selecting categories!");

	serde_json::to_string(&results).expect("Error serializing categories")
}

#[get("/category/full")]
async fn get_cat_full(mut db: Connection<Db>) -> String {
	let results = TransactionCategory::table
		.left_join(ExpenseCategory::table)
		.select(TransactionCategory::CategoryName, TransactionCategory::CategoryType, ExpenseCategory::MonthlyBudget)
		.load::<(TransCat, ExpCat)>(&mut db)
		.await
		.expect("Error selecting join of categories to expense details");

	serde_json::to_string(&results).expect("Error serializing full categories")	
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
		.mount("/", routes![index, home, get_trans, get_cat, get_acc, get_cat_full])
}

