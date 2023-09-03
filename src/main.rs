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
