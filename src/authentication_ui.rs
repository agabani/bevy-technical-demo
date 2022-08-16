use bevy::prelude::*;

pub(crate) struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(button)
            .add_system(input)
            .add_startup_system(setup);
    }
}

#[derive(Component)]
pub(crate) struct AuthenticationUi;

#[derive(Component)]
pub(crate) struct UsernameText;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn button(
    mut interactions: Query<
        (&Interaction, &mut UiColor, &Children, &AuthenticationUi),
        (Changed<Interaction>, With<Button>),
    >,
    mut text: Query<&mut Text>,
) {
    for (interaction, mut color, children, _) in &mut interactions {
        let mut text = text.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Logging in...".into();
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn input(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut string: Local<String>,
    mut username: Query<(&AuthenticationUi, &mut Text), With<UsernameText>>,
) {
    for ev in char_evr.iter() {
        string.push(ev.char);

        let (_, mut username) = username.get_single_mut().unwrap();
        username.sections[0].value = string.clone();
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(Name::new("camera"))
        .insert(AuthenticationUi);

    commands
        .spawn_bundle(
            TextBundle::from_section(
                "abc",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            )
            .with_text_alignment(TextAlignment::TOP_CENTER)
            .with_style(Style {
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Style::default()
            }),
        )
        .insert(Name::new("username"))
        .insert(AuthenticationUi)
        .insert(UsernameText);

    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Style::default()
            },
            color: NORMAL_BUTTON.into(),
            ..ButtonBundle::default()
        })
        .insert(Name::new("button"))
        .insert(AuthenticationUi)
        .add_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Login",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..TextStyle::default()
                },
            ));
        });
}
