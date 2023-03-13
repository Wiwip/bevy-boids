
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::{EguiContexts, EguiPlugin};
use bevy_inspector_egui::egui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)

        .add_system(top_bar_system)
        .add_system(create_flock_window)

        .run();

}



fn top_bar_system(
    mut context: EguiContexts
) {
    egui::TopBottomPanel::top("top_panel").show(context.ctx_mut(), |ui| {
        // The top panel is often a good place for a menu bar:
        egui::menu::bar(ui, |ui| {
            egui::menu::menu_button(ui, "File", |ui| {
                let quit = ui.button("Quit");
                if quit.clicked() {
                    std::process::exit(0);
                }
            });
        });
    });
}

fn create_flock_window(mut context: EguiContexts) {
    let mut name = "test";
    let mut age = 5;
    egui::Window::new("My new flock")
        .show(context.ctx_mut(), |ui|{
            ui.heading("M");
            ui.vertical(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut name);
                ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
                if ui.button("Click each year").clicked() {
                    age += 1;
                }
                ui.label(format!("Hello '{}', age {}", name, age));
            });
        });

    egui::Window::new("Window")
        .vscroll(true)
        .show(context.ctx_mut(), |ui| {
            ui.label("Windows can be moved by dragging them.");
            ui.label("They are automatically sized based on contents.");
            ui.label("You can turn on resizing and scrolling if you like.");
            ui.label("You would normally chose either panels OR windows.");
        });

}