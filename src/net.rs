use error::{ShadertoyError, InvalidShaderIdError};

use hyper::{self, Client};
use hyper::header::{Referer, ContentType};

use url::form_urlencoded;

use serde_json::{self, Value};

use std::io::Read;

pub fn get_shader_info_and_code(mut id: &str) -> Result<(String, String), ShadertoyError> {
    let https_url = "https://www.shadertoy.com/view/";
    let http_url  = "http://www.shadertoy.com/view/";
    let url       = "www.shadertoy.com/view/";

    if id.starts_with(https_url) || id.starts_with(http_url) || id.starts_with(url) {
        id = id.split_at(id.rfind("view/").unwrap() + 5).1;
    }

    let json = serde_json::from_str::<Value>(&get_json_string(id)?)?;

    extract_from_json(&json)
}

fn get_json_string(id: &str) -> Result<String, ShadertoyError> {
    let client = Client::new();

    let body = form_urlencoded::Serializer::new(String::new())
        .extend_pairs(vec![("s", format!("{{\"shaders\": [\"{}\"]}}", id))])
        .finish();

    let mut res = client.post("https://www.shadertoy.com/shadertoy/")
        .header(Referer("https://www.shadertoy.com/".to_string()))
        .header(ContentType("application/x-www-form-urlencoded".parse().unwrap()))
        .body(&body)
        .send()?;

    let mut buf = String::new();

    match res.read_to_string(&mut buf) {
        Ok(_) => {
            if buf == "[]" {
                let err = InvalidShaderIdError::new(format!("Shader '{}' not found", id));
                return Err(ShadertoyError::InvalidShaderId(err));
            } else {
                Ok(buf)
            }
        },
        Err(err) => {
            Err(ShadertoyError::DownloadShader(hyper::error::Error::from(err)))
        }
    }
}

fn extract_from_json(json: &Value) -> Result<(String, String), ShadertoyError> {
    let (info, mut code) = (String::from("UNIMPLEMENTED"), String::new());

    let shaders = json[0]["renderpass"].as_array().unwrap();
    if shaders.len() > 1 {
        for shader in shaders {
            if shader["name"] == "Image" {
                code = String::from(shader["code"].as_str().expect("CODE SEGMENT"));
            }
        }
    } else {
        code = String::from(shaders[0]["code"].as_str().expect("CODE SEGMENT"));
    }

    Ok((info, code))
}
