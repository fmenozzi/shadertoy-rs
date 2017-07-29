use error::{self, ShadertoyError, SaveShaderError};

use hyper::Client;
use hyper::header::{Referer, ContentType};

use url::form_urlencoded;

use serde_json::{self, Value};

use std::io::{self, Read, Write};
use std::fs::File;

pub fn download(id: &str) -> error::Result<(String, String)> {
    let (name, code) = get_shader_name_and_code(id)?;

    File::create(&name).or_else(|err| {
        return_save_shader_error(&name, err)
    })?.write_all(&code.as_bytes()).or_else(|err| {
        return_save_shader_error(&name, err)
    })?;

    Ok((name, code))
}

fn return_save_shader_error<E>(name: &str, err: io::Error) -> error::Result<E> {
    Err(ShadertoyError::SaveShader(SaveShaderError::new(name, err)))
}

fn get_shader_name_and_code(mut id: &str) -> error::Result<(String, String)> {
    let https_url = "https://www.shadertoy.com/view/";
    let http_url  = "http://www.shadertoy.com/view/";
    let url       = "www.shadertoy.com/view/";

    if id.starts_with(https_url) || id.starts_with(http_url) || id.starts_with(url) {
        id = id.split_at(id.rfind("view/").unwrap() + 5).1;
    }

    let json = serde_json::from_str::<Value>(&get_json_string(id)?)?;

    extract_from_json(&json)
}

fn get_json_string(id: &str) -> error::Result<String> {
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
                return Err(ShadertoyError::InvalidShaderId(id.into()))
            } else {
                Ok(buf)
            }
        },
        Err(err) => {
            Err(ShadertoyError::DownloadShader(err.into()))
        }
    }
}

fn extract_from_json(json: &Value) -> error::Result<(String, String)> {
    let name = format!("{}.frag", json[0]["info"]["name"].as_str().unwrap().replace(" ", "_")).to_lowercase();
    let mut code = String::new();

    let shaders = json[0]["renderpass"].as_array().unwrap();

    if shaders.len() > 1 {
        for shader in shaders {
            if shader["name"] == "Image" {
                code = String::from(shader["code"].as_str().unwrap());
            }
        }
    } else {
        code = String::from(shaders[0]["code"].as_str().unwrap());
    }

    Ok((name, code))
}
