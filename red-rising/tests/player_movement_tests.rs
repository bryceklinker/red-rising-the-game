#[cfg(test)]
mod player_movement_tests {
    use bevy::{MinimalPlugins};
    use bevy::prelude::{App, ButtonInput, KeyCode, Transform};
    use red_rising::plugins::player_movement_plugin::player_movement_plugin;

    fn setup_testing_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(player_movement_plugin);
        app.insert_resource(ButtonInput::<KeyCode>::default());
        app.update();
        return app;
    }

    #[test]
    fn when_w_is_pressed_then_player_moves_forward() {
        let mut app = setup_testing_app();

        press_key(& mut app, KeyCode::KeyW);

        let transform = app.world_mut().query::<&Transform>().single(app.world()).unwrap();
        assert_eq!(transform.translation.x, 0.0);
        assert_eq!(transform.translation.y, 1.0);
    }

    fn press_key(app: &mut App, key: KeyCode) {
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(key);
        app.update();
    }
}