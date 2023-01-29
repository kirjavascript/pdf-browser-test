mod tabs;
mod tab;
mod page;

use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "pdfbrowser",
        options,
        Box::new(|_cc| {

            Box::new(PDFWeb::default())
        }),
    );
}


struct PDFWeb {
    tabs: tabs::Tabs,
}

// TODO: password protection
// TODO: static compiling
// TODO: http

impl Default for PDFWeb {
    fn default() -> Self {
        Self {
            tabs: tabs::Tabs::new(),
        }
    }
}


impl eframe::App for PDFWeb {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.tabs.ui(ui);
        });
    }

}
