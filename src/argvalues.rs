use clap::App;

pub struct ArgValues {
    pub width: f32,
    pub height: f32,
    pub shaderpath: String,
    pub not_from_shadertoy: bool,
}

impl ArgValues {
    pub fn from_cli() -> Result<ArgValues, String> {
        // Load CLI matches
        let yaml = load_yaml!("cli.yml");
        let matches = App::from_yaml(yaml).get_matches();

        // Width
        let width: f32;
        match matches.value_of("width").unwrap().parse() {
            Ok(w)  => width = w,
            Err(e) => return Err(format!("Invalid width: {}", e)),
        }

        // Height
        let height: f32;
        match matches.value_of("height").unwrap().parse() {
            Ok(h)  => height = h,
            Err(e) => return Err(format!("Invalid height: {}", e)),
        }

        // Shader filepath
        let shaderpath = matches.value_of("shader").unwrap().to_string();

        // From Shadertoy?
        let not_from_shadertoy = matches.is_present("not_from_shadertoy");

        Ok(ArgValues {
            width: width,
            height: height,
            shaderpath: shaderpath,
            not_from_shadertoy: not_from_shadertoy,
        })
    }

    pub fn from_values(width: f32, height: f32, shaderpath: &str, not_from_shadertoy: bool) -> ArgValues {
        ArgValues {
            width: width,
            height: height,
            shaderpath: shaderpath.to_string(),
            not_from_shadertoy: not_from_shadertoy,
        }
    }
}
