use bevy::prelude::*;
use crate::components::*;

pub fn menu_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
    //assets: Res<ImageAssets>
){
    let start_button = btn_setup(&mut commands, &asset_server, "New Game");
    commands.entity(start_button).insert(StartButton{});
    commands.spawn(NodeBundle{
        style: Style{
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_self: AlignSelf::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        background_color: Color::BLACK.into(),
        ..default()
    })
    .insert(MenuUIRoot{})
    .with_children(|commands|{
        commands.spawn(TextBundle{
            style: Style{
                align_self: AlignSelf::Center,
                margin: UiRect::all(Val::Percent(3.0)),
                ..default()
            },
            text: Text::from_section(
                "Dead Punk",
                TextStyle
                 {
                    font: asset_server.load("MercutioNbpBasic.ttf"),
                    font_size: 96.0,
                    color: Color::WHITE
            }),
            ..default()
        });
    }).add_child(start_button);
}

//Button bundle function wich spawns button entity while called. It s just a styling with no functionality
fn btn_setup(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    text: &str
) -> Entity {
    commands.spawn(ButtonBundle{
        style: Style{
            width: Val::Percent(10.0),
            height: Val::Percent(10.0),
            align_self: AlignSelf::Center,
            justify_content: JustifyContent::Center,
            margin: UiRect::all(Val::Percent(2.0)),
            ..default()
        },
        background_color: Color::NONE.into(),
        ..default()
    }).with_children(|commands|{
        commands.spawn(TextBundle{
            style: Style{
                align_self: AlignSelf::Center,
                ..default()
            },
            text: Text::from_section(
                text,
                TextStyle {
                    font: asset_server.load("MercutioNbpBasic.ttf"),
                    font_size: 36.0,
                    color: Color::WHITE,
                }
            ),
            ..default()
        });
    }).id()
}

//Button Events
pub fn start_button_clicked(
    mut commands: Commands,
    interactions: Query<&Interaction, (With<StartButton>, Changed<Interaction>)>,
    menu_root: Query<Entity, With<MenuUIRoot>>,
    mut next_state: ResMut<NextState<GameState>>,
){
    for interaction in &interactions{
        if matches!(interaction, Interaction::Pressed){
            print!("Lol");
            let root_entity = menu_root.single();
            commands.entity(root_entity).despawn_recursive();
            next_state.set(GameState::Loading);
        }
    }
}