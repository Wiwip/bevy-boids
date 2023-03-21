
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::{EguiContexts, EguiPlugin};
use bevy_inspector_egui::egui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)

        .add_system(top_bar_system)
        .add_system(create_flock_window)
        .add_system(reset)

        // Events
        .add_event::<ResetGameState>()

        .run();

}

#[derive(Default)]
struct CreateFlockUiState {
    boid_count: u32,
    color: Color,
}


#[derive(Default)]
struct ResetGameState;

fn reset(
    mut event: EventReader<ResetGameState>
) {
    for i in &mut event {
        println!("reset?");
    }

}

fn top_bar_system(
    mut context: EguiContexts,
    mut event: EventWriter<ResetGameState>,
) {
    egui::TopBottomPanel::top("top_panel").show(context.ctx_mut(), |ui| {
        // The top panel is often a good place for a menu bar:
        egui::menu::bar(ui, |ui| {

            // File
            egui::menu::menu_button(ui, "File", |ui| {
                // Reset
                let reset = ui.button("Reset");
                if reset.clicked() {
                    event.send_default();
                }

                // Quit
                let quit = ui.button("Quit");
                if quit.clicked() {
                    std::process::exit(0);
                }
            });
        });
    });
}

fn create_flock_window(
    mut context: EguiContexts,
    mut state: Local<CreateFlockUiState>
) {
    let mut name = "test";
    egui::Window::new("My new flock")
        .show(context.ctx_mut(), |ui|{
            ui.heading("M");
            ui.vertical(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut name);
                ui.add(egui::Slider::new(&mut state.boid_count, 0..=120).text("Boid Count"));
                if ui.button("Click each year").clicked() {
                    state.boid_count += 1;
                }
                ui.label(format!("Hello '{}', age {}", name, state.boid_count));
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