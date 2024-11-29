#[macro_use] extern crate rocket;

use rocket::{ Rocket, Build };
use rocket::http::{ Cookie, CookieJar };
use rocket::fs::{ FileServer, NamedFile, relative };
use rocket::fairing::{ self, AdHoc };
use rocket::response::{ Redirect, content };
use rocket_db_pools::{ Database, Connection };
use rocket_db_pools::diesel::{ prelude::*, MysqlPool, QueryResult };

//use sha256;
use openssl::{ base64, rsa, sha };

use yaba::schema::*;
use yaba::models::*;


#[derive(Database)]
#[database("yaba")]
struct Db(MysqlPool);


// Login routes
#[get("/test")]
fn test() -> content::RawHtml<String> {
	content::RawHtml(r#"<!DOCTYPE html>
<html lang="en">
	<head>
		<title>Yaba Login</title>
	</head>
	<body>
		<h2>TEST</h2>
	</body>
</html>"#.into())
}


#[get("/")]
async fn login() -> Option<NamedFile> {
	NamedFile::open("webpages/login.html").await.ok()
}


// User authentication functions
//	Challenge-response protocol
fn compare_response(challenge: &str, username: &str, response: &str) -> bool {
	todo!();	// TODO: first call validateUser(username), then use that result to get the user passkey and generate the appropriate response to compare against (decrypt response before comparison)
}

fn generate_challenge() -> String {
	todo!();	// TODO: time-based (~10s? how to generate?) also send strings of different lengths and contents to reduce predictability
}

//	User validation
fn increment_bad_attempts(username: &str) {
	todo!();
}

fn reset_bad_attempts(username: &str) {
	todo!();
}

fn validate_user(username: &str) -> Option<String> {
	todo!();	// TODO: first try to get username from db with `SELECT Name FROM Users WHERE Name = <username> LIMIT 1;` and then validate that the BadAttempts for that user are less than 3 and return Some(username) (or None if any of those steps failed)
}

//	User session
fn create_user_session(cookie_jar: &CookieJar<'_>) {
	todo!();	// TODO: id is "session", value is username; max age is 10 minutes, expires is Session
}

fn validate_user_session(cookie_jar: &CookieJar<'_>) -> bool {
	todo!();	// TODO: get the user session private cookie (return false if no such cookie) and validate it (return false if invalid)
}


// Yaba routes and functions
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
			TransactionDate: new_trans_data.date.clone(),
			Description: new_trans_data.desc.clone(),
			Amount: new_trans_data.amt.clone(),
		})
		.execute(&mut db)
		.await?;

	let new_trans_id_nullable: Option<u32> = Transaction::table
		.select(diesel::dsl::max(Transaction::TransactionID))
		.first(&mut db)
		.await?;

	let new_trans_id = new_trans_id_nullable.unwrap();

	diesel::insert_into(TransactionInstanceCategory::table)
		.values(TransInstCat {
			TransactionID: new_trans_id,
			CategoryID: new_trans_data.cat,
		})
		.execute(&mut db)
		.await?;

	diesel::insert_into(TransactionAccount::table)
		.values(TransAcc {
			TransactionID: new_trans_id,
			AccountID: new_trans_data.acc,
		})
		.execute(&mut db)
		.await?;

	Ok("WORKING".into())
}


// Security Plan TODO:
// TODO: user authentication:
	// login page is sent with embedded nonce
	// challenge response is pwd SHA256 digest concatenated with challenge and encrypted with yaba server public key
	// nonce is time-based (derive from nondecreasing datum?) and expires after some short time (e.g. 10s?)
	// submitted user (identified by username) is locked out permanently (must be reset on server by manually unlocking in user db) after three failed login attempts (failed attempts are reset after successful login)
	// user session is tracked using Rust Rocket private cookies
	// user session lasts for short period (e.g., 10min?) before API calls are ignored (and logged) and a redirect to the login page is requested
	// valid user session private cookie must exist to access any endpoint other than login.html and login.css
	// valid user session private cookie is created upon successful login and is requested by server to be destroyed after session expires


// NOTE: demo users are "Alice":"P@ssw0rd1" and "bob":"asdf;lkj"
// NOTE: RustCrypto RSA (rsa crate) was found to be vulnerable to the Marvin Attack by a third party; the crate is actively being remidiated to address this


// Security attacks to defend against TODO:
// TODO: MITM and eavesdrop
	// Solve with: hash passwords for transmitting and storing in user db
// TODO: Masquerade
	// Solve with: only give out server public key directly from server (not through API, must have direct local access to server), then use public-key encryption for challenge-response login
// TODO: Host attacks (plaintext theft, dictionary search)
	// Solve with: transmit and store only password hashes; use good password practices
// TODO: Replay
	// Solve with: login nonce is time-based (10s max?)
// TODO: specific account attack
	// Solve with: permanent lockout after 3 attempts (can be undone from within server, maybe as a bool in user db?)


// Security remidiation TODO:
// TODO: user authentication
	// login page will accept username and password simultaneously without any intermediate communication with backend before challenge response; embed challenge in login page somehow?
	// User session will only last ten minutes, after which the page is reloaded and should redirect to login page
// TODO: set up E & J clients to trust yaba TLS snakeoil
// TODO: extract interface code to backend
// TODO: revamp REST APIs for cleaner and more controlled access
// TODO: obfuscate client JS


// yaba features TODO:
// TODO: error on bad transaction (rollback on backend, warn on client)
// TODO: budget reporting
// TODO: transaction list filtering and sorting
// TODO: multi-browser functionality (minimum of Chrome, Vivaldi, and qutebrowser)
// TODO: CSS
// TODO: favicon.ico
// TODO: (maybe) lightmode/darkmode button or browser-based


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
		.mount("/", routes![test])
		.mount("/login", routes![login])
		.mount("/", FileServer::from(relative!("webpages")))
		.mount("/category", routes![get_cats])
		.mount("/account", routes![get_accs])
		.mount("/transaction", routes![get_trans_list, log_trans])
}

