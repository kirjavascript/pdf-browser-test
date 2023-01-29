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
    image: Vec<egui::Color32>,
    width: u32,
    height: u32,
    tabs: tabs::Tabs,
}

// TODO: password protection
// TODO: static compiling
// TODO: egui dock

use pdfium_render::prelude::*;

impl Default for PDFWeb {
    fn default() -> Self {

        let pdfium = Pdfium::new(
            Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library()).expect("pdfium not found"),
        );

        // Load the document from the given path...

        let document = pdfium.load_pdf_from_file("./dummy.pdf", None).expect("load file");

        // ... set rendering options that will be applied to all pages...

        let render_config = PdfRenderConfig::new()
            .set_target_width(500)
            .set_maximum_height(500)
            .rotate_if_landscape(PdfBitmapRotation::Degrees90, true);

        // ... then render each page to a bitmap image, saving each image to a JPEG file.
        let pages = document.pages();
        let page = pages.get(0).unwrap();
        let image = page
                .render_with_config(&render_config).unwrap()
                .as_image(); // Renders this page to an image::DynamicImage...


        let page1 = image
                .as_rgba8() // ... then converts it to an image::Image...
                .ok_or(PdfiumError::ImageError).unwrap();

        let pixels: Vec<egui::Color32> = page1.chunks_exact(4)
            .map(|p| egui::Color32::from_rgb(p[0], p[1], p[2]))
            // .map(|p| egui::Rgba::from_srgba_premultiplied(p[0], p[1], p[2], p[3]).into())
            .collect();

        Self {
            image: pixels,
            width: image.width(),
            height: image.height(),
            tabs: tabs::Tabs::new(),
        }
    }
}


impl eframe::App for PDFWeb {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            ui.add(egui::TextEdit::singleline(&mut "http://fake.url/dummy.pdf".to_string()));
            // let texture: &egui::TextureHandle = &ui.ctx().load_texture(
            //     "viewport",
            //     egui::ColorImage {
            //         size: [self.width as _, self.height as _],
            //         pixels: self.image.clone(),
            //     },
            //     egui::TextureOptions::NEAREST
            // );


            // let img = egui::Image::new(texture, texture.size_vec2());

            // ui.add(img);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.tabs.ui(ui);
        });
    }

}
