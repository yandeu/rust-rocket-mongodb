use regex::Regex;
use rocket::request::FromParam;
use std::fmt;

#[derive(Clone)]
pub struct MongoId(String);

impl fmt::Display for MongoId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.clone().0)
    }
}

impl<'a> FromParam<'a> for MongoId {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<MongoId, Self::Error> {
        let re = Regex::new(r"^[a-f0-9]{24}$").unwrap();
        let m = re.is_match(param);

        if m {
            let p = MongoId(param.to_string());
            Ok(p)
        } else {
            Err(param)
        }
    }
}
