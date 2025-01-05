use serde_json;
use std::time::SystemTime;

type T = (String, u32);

pub struct Osdb {
    pub file: String,
}

impl Osdb {
    pub fn read(&self) -> Vec<T> {
        serde_json::from_str(&std::fs::read_to_string(&self.file).unwrap()).unwrap()
    }

    pub fn query(&self, m: (Option<String>, Option<u32>)) -> T {
        for i in self.read() {
            let mut is_match = true;
            if let Some(a) = m.0.clone() {
                is_match = a == i.0;
            }
            if is_match {
                if let Some(b) = m.1.clone() {
                    is_match = b == i.1;
                }
            }
            if is_match {
                return i;
            }
        }
        panic!("Not found")
    }
}

fn main() {
    let db = Osdb { file: "db.json".to_string() };

    let start = SystemTime::now();
    let q = db.query((Some("Eva".to_string()), None));
    let taken = SystemTime::now().duration_since(start).unwrap();

    println!("Fount {:?} and took {:?}", q, taken);
}
