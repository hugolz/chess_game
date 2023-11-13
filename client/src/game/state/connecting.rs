pub struct Connecting {
    ui: crate::ui::UiManager,
    client: crate::game::Client,
}

impl Connecting {
    pub fn new(ui: crate::ui::UiManager, client: crate::game::Client) -> Self {
        Self { ui, client }
    }
}

impl super::StateMachine for Connecting {
    fn update(mut self, _: f64) -> super::State {
        self.client.update().unwrap();
        if self.client.is_connected() {
            debug!("Client is now connected, switching State to connected");
            super::Connected::new(self.client).into()
        } else {
            warn!("Still trying to connect");
            self.into()
            // let dummy_state = std::mem::replace(&mut self.state, State::Connecting { client });
        }
    }
    fn draw(mut self, _: &mut crate::render::RenderRequest) -> super::State {
        self.into()
    }

    fn try_get_client_mut(&mut self) -> Option<&mut crate::game::Client> {
        Some(&mut self.client)
    }

    fn try_get_ui_mgr_mut(&mut self) -> Option<&mut crate::ui::UiManager> {
        Some(&mut self.ui)
    }
}
