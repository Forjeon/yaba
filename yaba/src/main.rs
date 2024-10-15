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


// GET requests
#[get("/names")]
async fn get_cat_names(mut db:Connection<Db>) -> String {
	let results = TransactionCategory::table
		.select(TransCat::as_select())
		.load(&mut db)
		.await
		.expect("Error selecting category names");

	serde_json::to_string(&results).expect("Error serializing category names")
}

#[get("/names")]
async fn get_acc_names(mut db:Connection<Db>) -> String {
	let results = PaymentAccount::table
		.select(PayAcc::as_select())
		.load(&mut db)
		.await
		.expect("Error selecting account names");

	serde_json::to_string(&results).expect("Error serializing account names")
}

#[get("/list")]
async fn get_trans_list(mut db:Connection<Db>) -> String {
		//date, desc, cat, acc, amt
	let results = Transaction::table
		.left_join(TransactionInstanceCategory::table.left_join(TransactionCategory::table))
		.left_join(TransactionAccount::table.left_join(PaymentAccount::table))
		.select((Trans::as_select(), Option::<TransInstCat>::as_select(), Option::<TransCat>::as_select(), Option::<TransAcc>::as_select(), Option::<PayAcc>::as_select()))
		.load::<(Trans, Option::<TransInstCat>, Option::<TransCat>, Option::<TransAcc>, Option::<PayAcc>)>(&mut db)
		.await
		.expect("Error selecting join of categories to expense details");

	serde_json::to_string(&results).expect("Error serializing transaction list")
}


// Transaction logging
#[post("/")]
async fn log_trans(mut db: Connection<Db>) -> QueryResult<String> {
	Ok("WORKING".into())
}


// Transaction deletion
// TODO: UI design + programming, future milestone



// Backend setup functions
async fn fetch_db(rocket: Rocket<Build>) -> fairing::Result {
	if let Some(db) = Db::fetch(&rocket) { Ok(rocket) } else { Err(rocket) }
}

#[launch]
fn rocket() -> _ {
	rocket::build()
		.attach(Db::init())
		.attach(AdHoc::try_on_ignite("DB Connection", fetch_db))
		.mount("/", routes![index, home])
		.mount("/category", routes![get_cat_names])
		.mount("/account", routes![get_acc_names])
		.mount("/transaction", routes![get_trans_list])
}

