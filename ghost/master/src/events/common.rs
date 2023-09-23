use crate::events::mouse::MouseDialogs;
use crate::events::translate::on_translate;

use rand::Rng;
use shiorust::message::{parts::HeaderName, parts::*, traits::*, Request, Response};

pub fn new_response() -> Response {
    let mut headers = Headers::new();
    headers.insert(
        HeaderName::Standard(StandardHeaderName::Charset),
        String::from("UTF-8"),
    );
    Response {
        version: Version::V30,
        status: Status::OK,
        headers,
    }
}

pub fn new_response_nocontent() -> Response {
    let mut r = new_response();
    r.status = Status::NoContent;
    r
}

pub fn new_response_with_value(value: String, use_translate: bool) -> Response {
    let v;
    if use_translate {
        v = on_translate(value);
    } else {
        v = value;
    }
    let mut r = new_response();
    r.headers.insert(HeaderName::from("Value"), v);
    r
}

pub fn new_response_from_mouse_dialogs(
    dialogs: &MouseDialogs,
    info: String,
    use_translate: bool,
) -> Response {
    match dialogs.get(&info) {
        Some(dialog) => match choose_one(dialog) {
            Some(s) => new_response_with_value(s, use_translate),
            None => new_response_nocontent(),
        },
        None => new_response_nocontent(),
    }
}

pub fn choose_one(values: &Vec<String>) -> Option<String> {
    if values.len() == 0 {
        return None;
    }
    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..values.len());
    Some(values[i].to_string())
}

pub fn get_references(req: &Request) -> Vec<&str> {
    let mut references: Vec<&str> = Vec::new();
    let mut i = 0;
    loop {
        match req
            .headers
            .get(&HeaderName::from(&format!("Reference{}", i)))
        {
            Some(value) => {
                references.push(value);
                i += 1;
            }
            None => break,
        }
    }
    references
}

pub fn user_talk(dialog: &str, text: &str) -> String {
    let mut result = String::new();
    if dialog != "" {
        result.push_str(&format!("『{}』\\n", dialog));
    }
    if text != "" {
        result.push_str(&format!("{}\\n", text));
    }

    format!("\\1\\n{}\\n", result)
}
