#[macro_use] extern crate rocket;

use std::collections::HashMap;
use std::fs::File;
use std::io::{ BufRead, BufReader, Read };
use std::net::{ IpAddr, Ipv4Addr, Ipv6Addr };
use std::sync::Mutex;
use std::time::{ Duration, Instant, SystemTime };

use rocket::{ Build, Rocket, State };
use rocket::http::{ Cookie, CookieJar };
use rocket::fs::{ FileServer, NamedFile, relative };
use rocket::fairing::{ AdHoc, self };
use rocket::response::{ Redirect, content };
use rocket_db_pools::{ Database, Connection };
use rocket_db_pools::diesel::{ prelude::*, MysqlPool, QueryResult };

use hex;
use openssl::sha::sha256 as OSSLsha256;
use rand::{ Rng, thread_rng };
use rsa::{ RsaPrivateKey, RsaPublicKey, Oaep, pkcs1::DecodeRsaPrivateKey, sha2::Sha256 };

use yaba::schema::*;
use yaba::models::*;


#[derive(Database)]
#[database("yaba")]
struct Db(MysqlPool);


// Helpers
fn read_page_file(filepath: &str) -> String {
	let mut page_content = String::new();
	let _ = BufReader::new(File::open(filepath).unwrap()).read_to_string(&mut page_content);
	page_content
}


// Login routes
#[get("/")]
async fn login(client_ip: IpAddr, challenge_map_state: &State<Mutex<HashMap<IpAddr, (String, Instant)>>>) -> content::RawHtml<String> {
	// Embeds a challenge in the login page JS script, then replies to the client with the login page as HTML
	content::RawHtml(read_page_file("webpages/templates/login.html").replace("%|%|CHALLENGE|%|%", &get_client_challenge(&client_ip, challenge_map_state)))
}


#[post("/", format = "application/octet-stream", data = "<data>")]
async fn log_in(mut db: Connection<Db>, client_ip: IpAddr, challenge_map_state: &State<Mutex<HashMap<IpAddr, (String, Instant)>>>, data: Vec<u8>) -> String {
	// Get client response
	//	Decrypt response
	let private_key = get_private_key();
	let decrypted_bytes = &private_key.decrypt(Oaep::new::<Sha256>(), &data).expect("Failed to decrypt client response");
	let client_response: String = std::str::from_utf8(&decrypted_bytes).unwrap().into();

	//	Split into challenge and username
	// TODO: client will send ciphertext of the concatenation `challenge + passkey + username`
	let username: String = client_response[64..].into();
	let client_challenge: String = client_response[..64].into();
	println!("Received client login: IP={:?}, user='{}', response='{}'", client_ip, username, client_challenge);//FIXME:DEL

	// Validate username
	let login_successful = true;//FIXME:TEMP
	let _ = validate_user(&mut db, "bob").await.expect("UHOH");
	// TODO

	// Validate challenge against compare challenge
	//	Get compare challenge
	//let compare_challenge_bytes = f
	//let compare_challenge = hex::encode(OSSLsha256(
	// TODO

	//	Compare to client challenge
	// TODO

	//	Clear client entry in challenge map
	challenge_map_state.lock().unwrap().remove(&client_ip);

	// Redirect based on challenge-response success
	if login_successful {
		"/home".into()
	}
	else {
		"/login".into()
	}
}


// User authentication functions
//	Challenge-response protocol
fn generate_challenge(client_ip: &IpAddr) -> String {
	// Get the timestamp and client_IP as byte arrays
	let timestamp_bytes = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos().to_le_bytes();

	let client_ip_string = client_ip.to_string();
	let client_ip_bytes = client_ip_string.as_bytes();

	// Concatenate the timestamp and client IP byte arrays
	let mut challenge_bytes = Vec::<u8>::new();
	challenge_bytes.extend_from_slice(&timestamp_bytes);
	challenge_bytes.extend_from_slice(client_ip_bytes);
	
	// Generate the challenge
	//	Hash the challenge bytes
	let challenge_hash = hex::encode(OSSLsha256(&challenge_bytes));

	//	Generate random substring endpoints
	let mut rng = thread_rng();
	let mut challenge_start = rng.gen_range(0..31);
	let mut challenge_end = rng.gen_range(0..31);

	//	Swap endpoints if necessary
	if challenge_end < challenge_start {
		(challenge_start, challenge_end) = (challenge_end, challenge_start);
	}

	// Return challenge
	challenge_hash[challenge_start..challenge_end].into()
}


fn get_client_challenge(client_ip: &IpAddr, challenge_map_state: &State<Mutex<HashMap<IpAddr, (String, Instant)>>>) -> String {
	let mut challenge_map = challenge_map_state.lock().unwrap();
	
	// Create the client challenge if necessary
	if !challenge_map.contains_key(&client_ip) {
		challenge_map.insert(client_ip.clone(), (generate_challenge(client_ip), Instant::now()));
	}

	// Return client challenge
	challenge_map.get(&client_ip).unwrap().0.clone()
}


fn get_compare_challenge(client_ip: &IpAddr, challenge_map_state: &State<Mutex<HashMap<IpAddr, (String, Instant)>>>) -> String {
	let challenge_map = challenge_map_state.lock().unwrap();

	if !challenge_map.contains_key(&client_ip) || challenge_map.get(&client_ip).unwrap().1.elapsed() > Duration::from_secs(20) {
		generate_challenge(client_ip)
	}
	else {
		challenge_map.get(&client_ip).unwrap().0.clone()
	}
}


fn get_private_key() -> RsaPrivateKey {
	let pem_filepath = "private.pem";
	let mut pem_contents = String::new();
	let _ = BufReader::new(File::open(pem_filepath).unwrap()).read_to_string(&mut pem_contents);
	RsaPrivateKey::from_pkcs1_pem(&pem_contents).expect("Failed to get RSA private key")
}


//	User validation
fn increment_bad_attempts(username: &str) {
	println!("INCR BAD ON {username} TODO!");	// TODO
}


fn reset_bad_attempts(username: &str) {
	println!("RESET BAD ON {username} TODO!");	// TODO
}


async fn validate_user(db: &mut Connection<Db>, username: &str) -> Option<String> {
	let result = Users::table
		.filter(Users::Name.eq(username))
		.select(UsersStruct::as_select())
		//.load(db)
		.first(db)
		.await;

	println!("VALUSR: {:?}", result);
	if result.is_err() || result.is_none() || (result.ok().is_some() && result.ok().some().BadAttempts >= 3) {
		None
	}
	else {
		let validated_username = result.ok().some().Name.
		reset_bad_attempts(validated_username);
		Some(validated_username)
	}
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
// NOTE: yaba could be vulnerable to browser switching on the same client IP / IP spoofing, as the only key into the challenge-response map is the client IP
// NOTE: yaba login resists inference attacks by giving no warning or error details upon failed login, instead simply reloading the login page


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
// TODO: extract interface code to backend
// TODO: revamp REST APIs for cleaner and more controlled access
	// Send ints and other rigid types as much as possible, heavily validate and sanitize string input (desc, etc.)
// TODO: obfuscate client JS
// TODO: set up E & J clients to trust yaba TLS snakeoil


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
		.mount("/login", routes![login, log_in])
		.mount("/", FileServer::from(relative!("webpages")))
		.mount("/category", routes![get_cats])
		.mount("/account", routes![get_accs])
		.mount("/transaction", routes![get_trans_list, log_trans])
		.manage(Mutex::new(HashMap::<IpAddr, (String, Instant)>::new()))
}

