use bevy::math::vec2;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_flock::flock::{
    BoidsAlignment, BoidsCoherence, BoidsRules, BoidsSeparation, DesiredVelocity, GameArea,
    WorldBoundForce,
};
use bevy_flock::interface::{adjust_from_ui_system, UiState};
use bevy_flock::perception::Perception;
use bevy_flock::physics::ObstacleAvoidance;
use bevy_flock::{flock, perception, SteeringPlugin};

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
        .add_plugin(SteeringPlugin)
        .init_resource::<OccupiedScreenSpace>()
        .insert_resource(BoidsRules {
            desired_speed: 75.0,
            max_force: 100.0,
            max_velocity: 125.0,
        })
        .insert_resource(UiState {
            coherence: 4.0,
            separation: 8.0,
            alignment: 2.0,
            ..default()
        })
        .insert_resource(GameArea {
            area: Rect::from_center_half_size(Vec2::ZERO, vec2(1200.0, 800.0)),
        })
        .add_startup_system(setup)
        .add_system(ui_example_system)
        .add_system(adjust_from_ui_system)
        .run();
}

fn setup(mut commands: Commands, rules: Res<GameArea>) {
    commands.spawn(Camera2dBundle::default());
    let perception = 32.;

    flock::new(&mut commands, 2000, rules.area, |ec| {
        ec.insert(Perception {
            range: perception,
            ..default()
        })
        .insert(BoidsCoherence { factor: 4.0 })
        .insert(BoidsSeparation {
            factor: 8.0,
            distance: 10.0,
        })
        .insert(BoidsAlignment { factor: 2.0 })
        .insert(WorldBoundForce { factor: 4.0 })
        .insert(ObstacleAvoidance { factor: 50.0 })
        .insert(DesiredVelocity { factor: 1.0 });
    });
}

#[derive(Default)]
struct CreateFlockUiState {
    boid_count: u32,
    color: Color,
}

fn ui_example_system(mut ui_state: ResMut<UiState>, mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::left("side_panel")
        .default_width(250.0)
        .max_width(300.0)
        .show(ctx, |ui| {
            ui.heading("Bevy Flocking");

            ui.allocate_space(egui::Vec2::new(2.0, 22.0));
            ui.add(egui::Slider::new(&mut ui_state.coherence, 0.0..=10.0).text("Cohesion"));
            ui.add(egui::Slider::new(&mut ui_state.separation, 0.0..=10.0).text("Separation"));
            ui.add(egui::Slider::new(&mut ui_state.alignment, 0.0..=10.0).text("Alignment"));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(egui::Hyperlink::from_label_and_url(
                    "powered by egui",
                    "https://github.com/emilk/egui/",
                ));
            });
        });
}
