#[macro_use]                                                                                                                                                                                                      
extern crate clap;
#[macro_use]
extern crate rust_embed;
// use cinder_cli::*;
extern crate job_scheduler;

use job_scheduler::{JobScheduler, Job};
use std::time::Duration;

use postgres::{Connection, TlsMode};

use dotenv::dotenv;
use std::env;

use clap::App;

struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

#[derive(RustEmbed)]
#[folder = "dist/"]
struct Asset;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
                                .author(crate_authors!())                                                                                                                                                        
                                .version(crate_version!())                                                                                                                                                       
                                .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    dotenv().ok();
    let conn = Connection::connect("postgres://postgres:postgres@localhost:5432/rustlang", TlsMode::None).unwrap();
    // conn.execute("CREATE TABLE person (
    //                 id              SERIAL PRIMARY KEY,
    //                 name            VARCHAR NOT NULL,
    //                 data            BYTEA
    //               )", &[]).unwrap();
    // let me = Person {
    //     id: 0,
    //     name: "Steven".to_string(),
    //     data: None,
    // };
    // conn.execute("INSERT INTO person (name, data) VALUES ($1, $2)",
    //              &[&me.name, &me.data]).unwrap();
    for row in &conn.query("SELECT id, name, data FROM person", &[]).unwrap() {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
            data: row.get(2),
        };
        println!("Found person {}: {}", person.id, person.name);
    }

    let mut sched = JobScheduler::new();

    let index_html = Asset::get("index.html").unwrap();
    println!("{:?}", std::str::from_utf8(index_html.as_ref()));

    for file in Asset::iter() {
        println!("{}", file.as_ref());
    }

    sched.add(Job::new("1/10 * * * * *".parse().unwrap(), || {
        println!("I get executed every 10 seconds!");
    }));

    loop {
        sched.tick();

        std::thread::sleep(Duration::from_millis(500));
    }
}