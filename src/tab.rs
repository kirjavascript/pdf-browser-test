pub struct Tab {
    // history
    url: String,
    image: Option<(Vec<egui::Color32>, u32, u32)>,
    errorText: Option<String>,
}

impl Tab {
    pub fn blank() -> Self {
        Self {
            url: "".to_string(),
            image: None,
            errorText: None,
        }
    }

    pub fn title(&self) -> String {
        "⚪new tab".to_string()
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // ui.button("<");
            // ui.button(">");
            if ui.button("go").clicked() {
                self.go();
            }
            let input = egui::TextEdit::singleline(&mut self.url).desired_width(f32::INFINITY);
            let res = ui.add(input);
            if res.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                self.go();
            }
        });

        if let Some(e) = &self.errorText {
            ui.colored_label(egui::Color32::from_rgb(255,0,0), e);
        }

        if let Some((pixels, width, height)) = &self.image {
            let texture: &egui::TextureHandle = &ui.ctx().load_texture(
                "viewport",
                egui::ColorImage {
                    size: [*width as _, *height as _],
                    pixels: pixels.clone(),
                },
                egui::TextureOptions::NEAREST
            );
            let img = egui::Image::new(texture, texture.size_vec2());
            ui.add(img);
        }

    }

    fn go(&mut self) {
        let res = reqwest::blocking::get(&self.url).unwrap().bytes().unwrap();
        let bytes: Vec<u8> = res.into();


        use pdfium_render::prelude::*;

        let pdfium = Pdfium::new(
            Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library()).expect("pdfium not found"),
        );

        // Load the document from the given path...

        let document = pdfium.load_pdf_from_byte_vec(bytes, None).expect("load file");

        // ... set rendering options that will be applied to all pages...

        let render_config = PdfRenderConfig::new()
            // .set_target_width(500)
            // .set_maximum_height(500)
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

        self.image = Some((pixels, image.width(), image.height()));
    }

}
