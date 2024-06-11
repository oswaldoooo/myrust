// // use crate::dep::person::Person;

// macro_rules! iname {
//     (struct $name:ident{ $($fname:ident : $ftype:ty),* }) => {
//         struct $name {
//           $($fname:$ftype),*
//         }
//         impl $name{
//           pub fn set_$fname(&mut self,$fname:$ftype){
//             self.$fname=$fname;
//           }
//         }
//     };
// }
// // struct Person {
// //     Name: String,
// //     Age: u8,
// // }
// iname! {
// struct Person {
//     Name: String,
//     Age: u8
// }
// }

// #[test]
// fn testmacro() {
//     let p = Person {
//         Name: "jackson".to_string(),
//         Age: 23,
//     };
//     p.set_Name("jack".to_string());
//     // let p = Person {
//     //     Name: "jack".to_string(),
//     //     Age: 23,
//     // };
// }
use paste::paste;
macro_rules! struct_getters_setters {
  ($struct_name:ident { $($field:ident : $field_type:ty),* }) => {
      paste!{
        impl $struct_name {
          $(

              pub fn [<get_$field>](&self) -> &$field_type {
                  &self.$field
              }
                pub fn [<set_$field>](&mut self, value: $field_type) {
                    self.$field = value;
                }
                // pub fn set_$field($mut self){

              // }
              // pub fn set_$field($mut self,value: $field_type){}
              // pub fn set_field(&mut self, value: $field_type) {
              //     self.$field = value;
              // }
          )*
      }
    }
  }
}

struct MyStruct {
    field1: i32,
    field2: String,
}

struct_getters_setters!(MyStruct {
    field1: i32,
    field2: String
});
#[test]
fn testone() {
    let mut my_struct = MyStruct {
        field1: 42,
        field2: String::from("Hello"),
    };
    // my_struct.set_field1(33);
    println!("field1 {},field2 {}", my_struct.get_field1(), my_struct.get_field2());
    // my_struct.
    // my_struct.set_field1(100);
    // my_struct.set_field2(String::from("World"));

    // println!("field1: {}", my_struct.field1);
    // println!("field2: {}", my_struct.field2);
}
use serde::*;
#[derive(Serialize, Deserialize,Debug)]
struct Person{
    name:String,
    age:u8,
}
#[test]
fn testjson(){
    let data="{\"name\":\"jackson\",\"age\":23}".to_string();
    let mut obj=json::from("{\"name\":\"jackson\",\"age\":23}");
    json::parse("{\"name\":\"jackson\",\"age\":23}");
    let p:Person=serde_json::from_str(data.as_str()).unwrap();
    println!("person is {:?}",p);

}