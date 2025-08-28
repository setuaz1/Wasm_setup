use eframe::wasm_bindgen::{self, prelude::*};

/// Это точка входа для Wasm
#[wasm_bindgen]
pub fn start(canvas_id: String) -> Result<(), eframe::wasm_bindgen::JsValue> { // ← Изменили &str на String
    // Консольные логи для отладки будут видны в браузере
    console_log::init_with_level(log::Level::Debug).ok();
    console_error_panic_hook::set_once();

    let web_options = eframe::WebOptions::default();
    
    wasm_bindgen_futures::spawn_local(async move { // ← Добавили move здесь
        eframe::WebRunner::new()
            .start(
                &canvas_id, // ← Теперь используем ссылку на String
                web_options,
                Box::new(|cc| Box::new(MyApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
    
    Ok(())
}

// Ваше приложение
#[derive(Default)]
struct MyApp {}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello, world!");
            if ui.button("Click me!").clicked() {
                web_sys::console::log_1(&"Button clicked!".into());
            }
        });
    }
}