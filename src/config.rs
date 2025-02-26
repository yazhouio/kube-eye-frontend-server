pub struct Config {
    pub server: ServerConfig,
    pub typst: TypstConfig,
}

pub struct ServerConfig {
    pub port: u16,
    pub host: String,
}

pub struct TypstConfig {

}
