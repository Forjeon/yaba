#[macro_use] extern crate rocket;


use std::str::FromStr;

use rocket::{ Rocket, Build, fs::NamedFile, response::Redirect };
use rocket::fairing::{ self, AdHoc };
use rocket_db_pools::{ Database, Connection };
use rocket_db_pools::diesel::{ prelude::*, MysqlPool, QueryResult };

use bigdecimal::BigDecimal;
use chrono::NaiveDate;

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

//	Category
#[get("/")]
async fn get_cats(mut db:Connection<Db>) -> String {
	let results = TransactionCategory::table
		.select(TransCat::as_select())
		.load(&mut db)
		.await
		.expect("Error selecting categories");

	serde_json::to_string(&results).expect("Error serializing categories")
}

//	Accounts
#[get("/")]
async fn get_accs(mut db:Connection<Db>) -> String {
	let results = PaymentAccount::table
		.select(PayAcc::as_select())
		.load(&mut db)
		.await
		.expect("Error selecting accounts");

	serde_json::to_string(&results).expect("Error serializing accounts")
}

//	Full transaction list
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


// POST requests

//	Transaction logging
#[post("/", format = "json", data = "<data>")]
async fn log_trans(mut db: Connection<Db>, data: String) -> QueryResult<String> {
	let new_trans_data: Trans_NewData = serde_json::from_str(&data).expect("Error deserializing new transaction data");

	diesel::insert_into(Transaction::table)
		.values(Trans_Insert {
			TransactionDate: new_trans_data.date,
			Description: new_trans_data.desc.clone(),
			Amount: new_trans_data.amt.clone(),
		})
		.execute(&mut db)
		.await?;

	let new_trans = Transaction::table
		.filter(Transaction::TransactionDate.eq(new_trans_data.date))
		.filter(Transaction::Description.eq(new_trans_data.desc))
		.filter(Transaction::Amount.eq(new_trans_data.amt))
		.select(Trans::as_select())
		.first(&mut db)
		.await?;
	println!("DEBUG: {:?}", new_trans);

	diesel::insert_into(TransactionInstanceCategory::table)
		.values(TransInstCat {
			TransactionID: new_trans.TransactionID,
			CategoryID: new_trans_data.cat,
		})
		.execute(&mut db)
		.await?;

	diesel::insert_into(TransactionAccount::table)
		.values(TransAcc {
			TransactionID: new_trans.TransactionID,
			AccountID: new_trans_data.acc,
		})
		.execute(&mut db)
		.await?;

	Ok("WORKING".into())
}


// DELETE requests

//	Transaction deletion
// TODO: UI design + programming, future milestone


// Backend setup functions
async fn fetch_db(rocket: Rocket<Build>) -> fairing::Result {
	if let Some(_) = Db::fetch(&rocket) { Ok(rocket) } else { Err(rocket) }
}

#[launch]
fn rocket() -> _ {
	rocket::build()
		.attach(Db::init())
		.attach(AdHoc::try_on_ignite("DB Connection", fetch_db))
		.mount("/", routes![index, home])
		.mount("/category", routes![get_cats])
		.mount("/account", routes![get_accs])
		.mount("/transaction", routes![get_trans_list, log_trans])
}

