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

        // Check to see if they want an example run
        let examplename = if matches.is_present("example") {
            Some(matches.value_of("example").unwrap().to_string())
        } else {
            None
        };

        // Fragment shader path
        let shaderpath = if matches.is_present("shader") {
            Some(matches.value_of("shader").unwrap().to_string())
        } else {
            None
        };

        // Texture paths
        let texture0path = if matches.is_present("texture0") {
            Some(matches.value_of("texture0").unwrap().to_string())
        } else {
            None
        };
        let texture1path = if matches.is_present("texture1") {
            Some(matches.value_of("texture1").unwrap().to_string())
        } else {
            None
        };
        let texture2path = if matches.is_present("texture2") {
            Some(matches.value_of("texture2").unwrap().to_string())
        } else {
            None
        };
        let texture3path = if matches.is_present("texture3") {
            Some(matches.value_of("texture3").unwrap().to_string())
        } else {
            None
        };

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
