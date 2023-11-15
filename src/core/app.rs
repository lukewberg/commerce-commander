pub struct App {
    pub secret: String,
    pub client_id: String,
    pub should_quit: bool,
    pub counter: u16,
}

impl App {
    pub fn new(secret: String, client_id: String) -> App {
        App {
            secret,
            client_id,
            should_quit: false,
            counter: 0,
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}
