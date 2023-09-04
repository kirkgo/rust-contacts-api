use postgres::{ Client, NoTls};
use postgres::Error as PostgresError;
use std::net::{ TcpListener, TcpStream};
use std::io::{ Read, Write};
use std::env;

#[macro_use]
extern crate serde_derive;

// Model: Contact struct with id, name, email and phone
#[derive(Serialize, Deserialize)]
struct Contact {
    id: Option<i32>,
    name: String,
    email: String,
    phone: String,
}

// database url
const DB_URL: &str = env!("DATABASE_URL");

// response constants
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 400 NOT FOUND\r\n\r\n";
const INTERNAL_ERROR: &str = "HTTP/1.1 500 INTERNAL ERROR\r\n\r\n";

// main function - entry point
fn main() {
    // set database
    if let Err(_) = set_database() {
        println!("Error setting database");
        return;
    }

    // start server and print port
    let listener = TcpListener::bind(format!("0.0.0.0:8080")).unwrap();
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}

// db setup
fn set_database() -> Result<(), PostgresError> {
    let mut client = Client::connect(DB_URL, NoTls)?;
    client.batch_execute(
        "
            CREATE TABLE IF NOT EXISTS contacts (
                id SERIAL PRIMARY KEY,
                name VARCHAR NOT NULL,
                email VARCHAR NOT NULL,
                phone VARCHAR NOT NULL
            )
        "
    )?;
    Ok(())
}

// get id from request URL
fn get_id(request: &str) -> &str {
    request.split("/")
        .nth(2)
        .unwrap_or_default()
        .split_whitespace()
        .next()
        .unwrap_or_default()
}

// deserialize contact from request body without id
fn get_contact_request_body(request: &str) -> Result<Contact, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}
