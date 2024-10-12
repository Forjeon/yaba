#[macro_use] extern crate rocket;


use rocket::{ Rocket, Build };
use rocket::fairing::{ self, AdHoc };
use rocket_db_pools::{ Database, Connection };
use rocket_db_pools::diesel::{ prelude::*, MysqlPool, QueryResult };


#[derive(Database)]
#[database("yaba")]
struct Db(MysqlPool);


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

