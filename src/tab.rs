pub struct Tab {

    // history

}

impl Tab {
    pub fn blank() -> Self {
        Self {}
    }

    pub fn title(&self) -> String {
        "new tab".to_string()
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label(format!("Content of X"));
    }
}
