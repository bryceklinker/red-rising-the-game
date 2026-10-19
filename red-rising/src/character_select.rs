use crate::game_state::GameState;
use bevy::asset::{Assets, Handle, RenderAssetUsages};
use bevy::image::{CompressedImageFormats, Image, ImageSampler, ImageType};
use bevy::prelude::{
    AlignItems, BackgroundColor, Button, Changed, Color, Commands, Component, Entity, ImageNode,
    Interaction, JustifyContent, NextState, Node, Query, ResMut, Text, TextColor, TextShadow,
    UiRect, Val, With,
};

#[derive(Component)]
pub struct CharacterSelectUi;

#[derive(Component)]
pub struct CharacterSelectButton;

const MENU_BACKGROUND_BYTES: &[u8] = include_bytes!("../assets/images/menu_background.png");

fn load_menu_background(images: &mut Assets<Image>) -> Handle<Image> {
    let image = Image::from_buffer(
        MENU_BACKGROUND_BYTES,
        ImageType::Extension("png"),
        CompressedImageFormats::NONE,
        true,
        ImageSampler::Default,
        RenderAssetUsages::default(),
    )
    .expect("menu background image should decode");
    images.add(image)
}

pub fn spawn_character_select_ui(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let background = load_menu_background(&mut images);
    commands
        .spawn((
            CharacterSelectUi,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ImageNode::new(background),
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    padding: UiRect::all(Val::Px(16.0)),
                    ..Default::default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.55)),
            ))
            .with_children(|panel| {
                panel
                    .spawn((CharacterSelectButton, Button, Node::default()))
                    .with_children(|button| {
                        button.spawn((
                            Text::new("Darrow"),
                            TextColor(Color::WHITE),
                            TextShadow::default(),
                        ));
                    });
            });
        });
}

pub fn handle_character_select_button(
    interactions: Query<&Interaction, (Changed<Interaction>, With<CharacterSelectButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interactions {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Drilling);
        }
    }
}

pub fn despawn_character_select_ui(
    mut commands: Commands,
    query: Query<Entity, With<CharacterSelectUi>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
