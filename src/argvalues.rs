use clap::App;

pub struct ArgValues {
    pub width: f32,
    pub height: f32,
    pub shaderpath: String,
    pub texture0path: String,
    pub texture1path: String,
    pub texture2path: String,
    pub texture3path: String,
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
        let shaderpath = if matches.is_present("example") {
            let example = matches.value_of("example").unwrap();
            if example.contains(".frag") {
                format!("examples/{}", example)
            } else {
                format!("examples/{}.frag", example)
            }
        } else {
            matches.value_of("shader").unwrap().to_string()
        };

        // Texture paths
        let texture0path = matches.value_of("texture0").unwrap().to_string();
        let texture1path = matches.value_of("texture1").unwrap().to_string();
        let texture2path = matches.value_of("texture2").unwrap().to_string();
        let texture3path = matches.value_of("texture3").unwrap().to_string();

        Ok(ArgValues {
            width: width,
            height: height,
            shaderpath: shaderpath,
            texture0path: texture0path,
            texture1path: texture1path,
            texture2path: texture2path,
            texture3path: texture3path,
        })
    }
}
