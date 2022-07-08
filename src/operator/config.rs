use chrono::Local;

pub struct Config {
    pub prog_name: String,
    pub dir_name: String,
    pub title: String,
    pub form: String,
    pub specify_fig: String,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let title = Local::now()
            .format(&format!("%Y%m%d-%H%M%S"))
            .to_string();

        let out_name = "workspace";
        let dir_name = format!("{}/{}", &out_name, &title);

        // Program Name
        let prog_name = format!("{}/ode", &dir_name,);

        // figure form (.png, .jpg, ... etc.)
        let form = format!(".png");
        let specify_fig = format!("{}/img", dir_name);

        Ok(Config {
            prog_name: prog_name,
            dir_name: dir_name,
            title: title,
            form: form,
            specify_fig: specify_fig,
        })
    }
}