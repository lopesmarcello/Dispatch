use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl Method {
    pub fn as_str(&self) -> &'static str {
        match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::PATCH => "PATCH",
            Method::DELETE => "DELETE",
        }
    }

    pub fn from_index(index: u32) -> Self {
        match index {
            0 => Method::GET,
            1 => Method::POST,
            2 => Method::PUT,
            3 => Method::PATCH,
            4 => Method::DELETE,
            _ => Method::GET,
        }
    }

    pub fn to_index(&self) -> u32 {
        match self {
            Method::GET => 0,
            Method::POST => 1,
            Method::PUT => 2,
            Method::PATCH => 3,
            Method::DELETE => 4,
        }
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for Method {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "PATCH" => Ok(Method::PATCH),
            "DELETE" => Ok(Method::DELETE),
            _ => Err(()),
        }
    }
}
