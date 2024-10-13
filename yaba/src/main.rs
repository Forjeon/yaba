#[macro_use] extern crate rocket;


use rocket::{ Rocket, Build, fs::NamedFile, response::Redirect };
use rocket::fairing::{ self, AdHoc };
use rocket_db_pools::{ Database, Connection };
use rocket_db_pools::diesel::{ prelude::*, MysqlPool, QueryResult };

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

#[get("/index.css")]
async fn home_css() -> Option<NamedFile> {
	NamedFile::open("webpages/index.css").await.ok()
}

#[get("/transaction")]
fn get_trans() -> &'static str {
	"TODO: get transactions"	// TODO
}

// GET requests
#[get("/category")]
async fn get_cat(mut db: Connection<Db>) -> String {//&'static str {
	use yaba::schema::TransactionCategory::dsl::*;

	let results = TransactionCategory
		.select(TransCat::as_select())
		.load(&mut db).await.expect("UhOH!");
	println!("TESTING: {:?}", results);
		//.expect("Error loading categories");

	format!("{:?}", results)
	//"TODO: get categories"	// TODO
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
		.mount("/", routes![index, home, get_trans, get_cat, get_acc])
}

