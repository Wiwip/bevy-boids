use bevy::math::vec2;
use bevy::{prelude::*};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_flock::flock::{BoidsRules, GameArea};
use bevy_flock::{flock, perception, FlockingPlugin};

#[derive(Default, Resource)]
struct OccupiedScreenSpace {
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}

#[derive(Resource, Deref, DerefMut)]
struct OriginalCameraTransform(Transform);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(FlockingPlugin)
        .init_resource::<OccupiedScreenSpace>()
        .init_resource::<UiState>()
        .insert_resource(BoidsRules {
            desired_speed: 75.0,
            max_force: 100.0,
            max_velocity: 125.0,
        })
        .insert_resource(GameArea {
            area: Rect::from_center_half_size(Vec2::ZERO, vec2(1000.0, 800.0)),
        })
        .add_startup_system(setup)
        .add_system(ui_example_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let area = Rect::from_center_half_size(Vec2::ZERO, vec2(1200.0, 1000.0));
    let perception = 32.;
    let list = flock::new(1000, area, perception);
    commands.spawn_batch(list);
}

#[derive(Default)]
struct CreateFlockUiState {
    boid_count: u32,
    color: Color,
}

#[derive(Default, Resource)]
struct UiState {
    label: String,
    value: f32,
    inverted: bool,
    egui_texture_handle: Option<egui::TextureHandle>,
    is_window_open: bool,
}

fn ui_example_system(
    mut ui_state: ResMut<UiState>,
    mut contexts: EguiContexts,
    mut state: Local<CreateFlockUiState>,
) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(ctx, |ui| {
            ui.heading("Tools Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut ui_state.label);
            });

            ui.add(egui::Slider::new(&mut ui_state.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                ui_state.value += 1.0;
            }

            ui.allocate_space(egui::Vec2::new(1.0, 100.0));
            ui.horizontal(|ui| {
                ui.button("Load").clicked();
                ui.button("Invert").clicked();
                ui.button("Remove").clicked();
            });

            ui.allocate_space(egui::Vec2::new(1.0, 10.0));
            ui.checkbox(&mut ui_state.is_window_open, "Pause");

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(egui::Hyperlink::from_label_and_url(
                    "powered by egui",
                    "https://github.com/emilk/egui/",
                ));
            });
        });
}
