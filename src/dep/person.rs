// use std::ptr::eq;

// use std::str::FromStr;
pub struct Person<'a> {
    pub _name: &'a str,
    pub _age: u8,
    pub _race: Race,
}
#[derive(EnumString, Display, Debug, PartialEq)]
pub enum Race {
    Asia,
    Africa,
    America,
}
impl Person<'_> {
    pub fn info<'a>(&self) -> String {
        let info = format!(
            "name:{},age:{},race:{}",
            self._name.to_string(),
            self._age,
            self._race,
        );
        return info;
    }
}
impl crate::dep::stock::Human for Person<'_> {
    fn is_human(&self) -> bool {
        return true;
    }
}
impl Drop for Person<'_> {
    fn drop(&mut self) {
        print!("drop {}", self._name);
    }
}

pub mod enhance {
    #[allow(dead_code)]
    pub fn get_age() -> u8 {
        return 0;
    }
    pub struct Student<'a> {
        pub _person: super::Person<'a>,
        pub _class_name: &'a str,
        pub _score: u16,
    }
    impl Student<'_> {
        #[allow(dead_code)]
        pub fn new(name: String, age: u8, ClassName: String) -> Self {
            return Self {
                _person: super::Person {
                    _name: "jackson",
                    _age: 23,
                    _race: super::Race::Asia,
                },
                _class_name: "one",
                _score: 560,
            };
        }
        pub fn get_level(&self) -> String {
            if self._score < 400 {
                return "C".to_string();
            } else if self._score < 550 {
                return "B".to_string();
            } else {
                return "A".to_string();
            }
        }
    }
    impl crate::dep::stock::Human for Student<'_> {
        fn is_human(&self) -> bool {
            return true;
        }
    }
}
