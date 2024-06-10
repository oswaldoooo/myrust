use std::collections::HashMap;

use self::loader::Loader;

pub trait Human {
    fn is_human(&self) -> bool {
        return false;
    }
}
#[derive(Copy, Clone)]
pub struct Stock {
    // _name: String,
    pub _name: &'static str,
    pub _price: f64,
}
static mut STOCK_LIST: Vec<Stock> = vec![];
impl Stock {
    #[allow(dead_code)]
    pub fn add_stock<'a>(_stock: &'a Stock) {
        unsafe {
            STOCK_LIST.push(*_stock);
        }
    }
    #[allow(dead_code)]
    pub fn get_stock_info<'a>(_stock_name: &'a str) {}
    #[allow(dead_code)]
    pub fn load_from_disk(&self, rpath: &'static str) -> Loader {
        return loader::load_stock_from_disk(rpath);
    }
    #[allow(dead_code)]
    pub fn check(&self) -> Option<String> {
        match stock_checker::check(self) {
            Some(things) => {}
            None => return Some("load error".to_string()),
        }
        match stock_checker::check_err(self) {
            Err(someerr) => {
                return Some(format!("{}", someerr).to_string());
            }
            Ok(ans) => {
                return None;
            }
        }
    }
}
impl Human for Stock {}
pub struct VariableStock {
    pub _name: String,
    pub _price: f64,
    _parent_len: usize,
}
pub struct StockClass {
    pub _name: String,
    _hash_map: HashMap<String, Stock>,
}
impl StockClass {
    pub fn new(_name: &'static str) -> Self {
        return Self {
            _name: String::from(_name),
            _hash_map: HashMap::new(),
        };
    }
    pub fn add(&mut self, stock_name: &'static str, stock_price: f64) {
        self._hash_map.insert(
            String::from(stock_name),
            Stock {
                _name: stock_name,
                _price: stock_price,
            },
        );
    }
    pub fn get(&self, stock_name: &'static str) -> Option<VariableStock> {
        let ans = self._hash_map.get(stock_name);
        match ans {
            None => return None,
            Some(data) => {
                let mut fname = self._name.clone();
                fname.push('_');
                fname.push_str(data._name);
                return Some(VariableStock {
                    _name: fname,
                    _price: data._price,
                    _parent_len: self._name.len(),
                });
            }
        }
    }
}

impl VariableStock {
    pub fn get_real_name(&self) -> &str {
        let rname = self._name.as_str();
        let real_name = &rname[self._parent_len + 1..];
        return real_name;
    }
}
pub mod loader {
    use std::{fmt, fs};
    pub struct Loader {
        pub _size: usize,
        pub _msg: &'static str,
        content: Box<[u8; 1024]>,
    }
    impl fmt::Display for Loader {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let content = self.content.as_slice();
            return write!(f, "{}", String::from_utf8_lossy(content));
        }
    }
    #[allow(dead_code)]
    pub fn load_stock_from_disk(rpath: &'static str) -> Loader {
        use std::io::Read;
        let mut fio = fs::File::open(rpath).unwrap();
        let mut buf = Box::new([0u8; 1 << 10]);
        let res = fio.read(buf.as_mut_slice());
        let size = res.unwrap();
        return Loader {
            _size: size,
            _msg: "ok",
            content: buf,
        };
    }
}
mod stock_checker {
    #[allow(dead_code)]
    pub fn check(_target: &super::Stock) -> Option<bool> {
        if _target._name.len() == 0 {
            return None;
        }
        return Some(true);
    }
    #[derive(EnumString, Display, Debug, PartialEq)]
    pub enum CheckErr {
        Ok,
        NameInvalid,
        PriceInvalid,
    }
    #[allow(dead_code)]
    pub fn check_err(_target: &super::Stock) -> Result<bool, CheckErr> {
        if _target._name.len() == 0 {
            return Err(CheckErr::PriceInvalid);
        } else if _target._price == 0.0 {
            return Err(CheckErr::PriceInvalid);
        }
        return Ok(true);
    }
}
