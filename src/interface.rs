use crate::behaviours::{Alignment, Coherence, Separation};
use bevy::prelude::{DetectChanges, Query, Res, Resource};

#[derive(Default, Resource)]
pub struct UiState {
    pub coherence: f32,
    pub separation: f32,
    pub alignment: f32,
    pub dirty: bool,
}

pub fn adjust_from_ui_system(
    mut query: Query<(&mut Coherence, &mut Separation, &mut Alignment)>,
    res: Res<UiState>,
) {
    if !res.is_changed() {
        return;
    }

    for (mut coh, mut sep, mut ali) in &mut query {
        coh.factor = res.coherence;
        sep.factor = res.separation;
        ali.factor = res.alignment;
    }
}
