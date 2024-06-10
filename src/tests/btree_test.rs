use bplustree::BPlusTree;
use rust_decimal::prelude::*;
#[test]
fn test_btree() {
    use std::collections::BTreeMap;
    let mut bpm: BPlusTree<Decimal, Order> = BPlusTree::new();
    let mut kp = Decimal::from_str("132.7932").unwrap();
    let mut ord = Order {
        price: kp,
        id: "10001",
    };
    bpm.insert(kp, ord);
    kp = Decimal::from_str("140.129").unwrap();
    ord.price = kp;
    ord.id = "10002";
    bpm.insert(kp, ord);
    kp = Decimal::from_str("119.129").unwrap();
    ord.price = kp;
    ord.id = "10003";
    bpm.insert(kp, ord);
    let mut iter = bpm.raw_iter();
    iter.seek_to_first();
    let mut vnode = iter.next();
    let mut i = 0;
    while vnode.is_some() && i < bpm.len() {
        let vnd = vnode.unwrap();
        println!("price {} id {}", vnd.0, vnd.1.id);
        vnode = iter.next();
        i += 1;
    }
    println!("end");
    // let data = bpm.lookup(&kp, |value| *value);
}
#[derive(Clone)]
struct Order {
    pub price: Decimal,
    pub id: &'static str,
}
impl Copy for Order {}
