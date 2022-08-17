use bevy::prelude::*;

pub(crate) struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(button)
            // .add_system(input)
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
        .spawn_bundle(NodeBundle {
            color: Color::NONE.into(),
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                margin: UiRect::all(Val::Auto),
                ..Style::default()
            },
            ..NodeBundle::default()
        })
        .insert(Name::new("form"))
        .with_children(|parent| {
            parent
                .spawn_bundle(
                    TextBundle::from_section(
                        "username",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 48.0,
                            color: Color::rgb(0.5, 0.5, 0.5),
                        },
                    )
                    .with_style(Style {
                        max_size: Size::new(Val::Percent(100.0), Val::Px(48.0)),
                        ..Style::default()
                    }),
                )
                .insert(Name::new("username"));

            parent
                .spawn_bundle(
                    TextBundle::from_section(
                        "password",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 48.0,
                            color: Color::rgb(0.5, 0.5, 0.5),
                        },
                    )
                    .with_style(Style {
                        max_size: Size::new(Val::Percent(100.0), Val::Px(48.0)),
                        ..Style::default()
                    }),
                )
                .insert(Name::new("password"));

            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Style::default()
                    },
                    color: Color::rgb(0.56, 0.96, 0.27).into(),
                    ..ButtonBundle::default()
                })
                .insert(Name::new("login"))
                .add_children(|parent| {
                    parent.spawn_bundle(
                        TextBundle::from_section(
                            "login",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 48.0,
                                color: Color::rgb(0.5, 0.5, 0.5),
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(8.0)),
                            ..Style::default()
                        }),
                    );
                });
        });
}
