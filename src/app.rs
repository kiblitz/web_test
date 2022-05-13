/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ClassNotes {
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,

    mode: Mode,
    page: Page,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(PartialEq)]
enum Mode {
    NONE,
    NOTES,
}

#[derive(serde::Deserialize, serde::Serialize)]
enum Page {
    NONE,
    KthSmallestIntro,
}

impl Default for ClassNotes {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 3.1,
            mode: Mode::NOTES,
            page: Page::NONE,
        }
    }
}

impl ClassNotes {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    pub fn update_none(
        &mut self, ctx: &egui::Context, _frame: &mut eframe::Frame
    ) {
        egui::CentralPanel::default().show(ctx, |_| {});
    }

    pub fn update_notes(
        &mut self, ctx: &egui::Context, _frame: &mut eframe::Frame
    ) {
        let Self { label, value, .. } = self;

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::CollapsingHeader::new("Kth Smallest").show(ui, |ui| {
                    if ui.link("Intro").clicked() {
                        self.page = Page::KthSmallestIntro
                    };
                });
                egui::CollapsingHeader::new("heading2").show(ui, |ui| {
                    ui.label("Contents");
                });
            });

            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });

            ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                *value += 1.0;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
                });
            });
        });

        let page_renderer = match self.page {
            Page::KthSmallestIntro => ClassNotes::page_kth_smallest_intro,
            _ => ClassNotes::page_none,
        };

        egui::CentralPanel::default().show(ctx, page_renderer);

        if true {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }

    }

    fn page_none(ui: &mut egui::Ui) {
        ui.heading("web test");
        ui.hyperlink("https://github.com/");
        ui.add(egui::github_link_file!(
            "https://github.com/kiblitz",
            "Profile."
        ));
    }

    fn page_kth_smallest_intro(_ui: &mut egui::Ui) {
    }
}

impl eframe::App for ClassNotes {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Mode", |ui| {
                    let mut selected: Option<Mode> = None;
                    if ui.button("Notes").clicked() {
                        selected = Some(Mode::NOTES);
                    } else if ui.button("None").clicked() {
                        selected = Some(Mode::NONE);
                    }
                    if let Some(mode) = selected {
                        if mode != self.mode {
                            self.mode = mode;
                            ctx.request_repaint();
                        }
                    }
                });
            });
        });

        match self.mode {
            Mode::NONE => self.update_none(ctx, frame),
            Mode::NOTES => self.update_notes(ctx, frame),
        };
    }
}
