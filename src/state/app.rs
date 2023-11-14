pub struct App {
    pub secret: String,
    pub client_id: String,
    pub should_quit: bool,
}

impl App {
    pub fn new(secret: String, client_id: String) -> App {
        App {
            secret,
            client_id,
            should_quit: false,
        }
    }
}
