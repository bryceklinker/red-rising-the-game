use bevy::asset::Assets;
use bevy::audio::{AudioPlayer, AudioSource, PlaybackSettings};
use bevy::prelude::{Commands, Component, Entity, Query, ResMut, With};

const DRILL_RUMBLE_BYTES: &[u8] = include_bytes!("../assets/audio/drill_rumble.wav");
const CALL_BEEP_BYTES: &[u8] = include_bytes!("../assets/audio/call_beep.wav");

#[derive(Component)]
pub struct DrillRumbleAudio;

#[derive(Component)]
pub struct CallBeepAudio;

pub fn spawn_drill_rumble(mut commands: Commands, mut audio_sources: ResMut<Assets<AudioSource>>) {
    let handle = audio_sources.add(AudioSource {
        bytes: DRILL_RUMBLE_BYTES.into(),
    });
    commands.spawn((
        DrillRumbleAudio,
        AudioPlayer::new(handle),
        PlaybackSettings::LOOP,
    ));
}

pub fn despawn_drill_rumble(mut commands: Commands, query: Query<Entity, With<DrillRumbleAudio>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_call_beep(mut commands: Commands, mut audio_sources: ResMut<Assets<AudioSource>>) {
    let handle = audio_sources.add(AudioSource {
        bytes: CALL_BEEP_BYTES.into(),
    });
    commands.spawn((
        CallBeepAudio,
        AudioPlayer::new(handle),
        PlaybackSettings::DESPAWN,
    ));
}
