use clap::App;

pub struct ArgValues {
    pub width: f32,
    pub height: f32,

    // None if using default fragment shader
    pub shaderpath: Option<String>,

    // None if using default textures
    pub texture0path: Option<String>,
    pub texture1path: Option<String>,
    pub texture2path: Option<String>,
    pub texture3path: Option<String>,

    // Some(name) if running an example
    pub examplename: Option<String>,
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

        // Closure for converting &str to String
        let str_to_string = |s: &str| s.to_string();

        // Check to see if they want an example run
        let examplename = matches.value_of("example").map(&str_to_string);

        // Fragment shader path
        let shaderpath = matches.value_of("shader").map(&str_to_string);

        // Texture paths
        let texture0path = matches.value_of("texture0").map(&str_to_string);
        let texture1path = matches.value_of("texture1").map(&str_to_string);
        let texture2path = matches.value_of("texture2").map(&str_to_string);
        let texture3path = matches.value_of("texture3").map(&str_to_string);

        Ok(ArgValues {
            width: width,
            height: height,
            shaderpath: shaderpath,
            texture0path: texture0path,
            texture1path: texture1path,
            texture2path: texture2path,
            texture3path: texture3path,
            examplename: examplename,
        })
    }
}
