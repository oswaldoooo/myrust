#[path = "dep/mod.rs"]
mod dep;
#[path = "tests/one_test.rs"]
mod tests;
#[macro_use]
extern crate strum;

use std::env::consts::OS;

use dep::stock::Human;

use crate::dep::stock::{self, Stock};
fn main() {
    tperson();
    tpersonmod();
    tstudent();
    tstock();
    tos();
}
fn tperson() {
    let name = "jack";
    let p = crate::dep::person::Person {
        _name: name,
        _age: 23,
        _race: crate::dep::person::Race::Asia,
    };
    println!("is human {} info:\n{}", p.is_human(), p.info());
}
fn tpersonmod() {
    let name = "jack";
    let p = crate::dep::person::Person {
        _name: name,
        _age: 23,
        _race: crate::dep::person::Race::Asia,
    };
}
fn tstudent() {
    let adamwin =
        crate::dep::person::enhance::Student::new(String::from("adamwin"), 23, String::from("one"));
    println!(
        "adamwin level is {} is human {}",
        adamwin.get_level(),
        adamwin.is_human()
    );
}
fn tstock() {
    Stock::add_stock(&Stock {
        _name: "myname",
        _price: 23.0,
    });
    let st = Stock {
        _name: "brotherhood",
        _price: 0.0,
    };
    println!("stock is human {}", st.is_human());
    let box2: Box<stock::loader::Loader> = Box::new(st.load_from_disk("test.st"));
    println!("size is {}\n{}", box2._size, box2);
    let status = st.check();
    match status {
        None => {
            println!("ok")
        }
        Some(err) => {
            eprintln!("err:{err}")
        }
    }
    // stock class
    let mut sc = stock::StockClass::new("jacket");
    sc.add("queue", 2.32);
    sc.add("king", 1.33);
    let ans = sc.get("queue");
    match ans {
        None => eprintln!("not found queue"),
        Some(data) => println!(
            "found {} {} realname {}",
            data._name,
            data._price,
            data.get_real_name()
        ),
    }
}
fn tos() {
    let x = OS;
    println!("os is {x}");
    crate::dep::stockadv::StockAdv1::listen("127.0.0.1".to_string(), 1999);
}
