// Testing/Example
// Scrolling with the mousewheel isn't provided with the crate, you must implement it yourself (or you can copy the one here if you want)

// Currently scrollbars don't work very well when their parent has the min/max values set, so keep that in mind when using this crate

// Use CTRL do scroll horizontally
// Use PageUp/PageDown to change the UI scale

use bevy::{input::mouse::{MouseScrollUnit, MouseWheel}, picking::focus::HoverMap, prelude::*};
use bevy_simple_scrollbar::prelude::*;

fn main() {
    App::new()
    .add_plugins((DefaultPlugins, SimpleScrollbarPlugin::default()))
    .add_systems(Startup, setup)
    .add_systems(Update, (update_scroll_position, change_ui_scale))
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // Vertical scroll
    commands.spawn((
        Node {
            min_height: Val::Px(256.0),
            height: Val::Px(1.0), // You need to add a value along the axis of your scrollbar, otherwise it won't be visible.
            max_height: Val::Px(512.0),
            min_width: Val::Px(128.0),
            max_width: Val::Px(512.0),
            top: Val::Percent(10.0),
            left: Val::Percent(50.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Stretch,
            align_items: AlignItems::Stretch,
            ..default()
        },
        BackgroundColor (Color::Srgba(Srgba { red: 0.2, green: 0.2, blue: 0.2, alpha: 1.0 })),
    )).with_children(|parent| {
        let scroll_area_id = parent.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                overflow: Overflow::scroll_y(),
                ..default()
            },
        )).with_children(|parent| {
            // Spawning the scroll area items
            for i in 0..50 {
                parent.spawn((
                    Node {
                        margin: UiRect { left: Val::Px(3.0), right: Val::Px(3.0), top: Val::Px(3.0), bottom: Val::Px(3.0) },
                        ..default()
                    },
                    Text(format!("Item {i}")),
                    TextFont {
                        font: asset_server.load(
                            "fonts/Gantari-Regular.ttf",
                        ),
                        ..default()
                    },
                ))
                .insert(PickingBehavior {
                    should_block_lower: false,
                    ..default()
                });
            }
        }).id();
        // Vertical scrollbar
        // The scrollbar MUST have a parent with the node component
        parent.spawn((
            Node {
                width: Val::Px(8.0),
                height: Val::Percent(100.0), // Required
                top: Val::Px(0.0), // Required
                ..default()
            },
            BackgroundColor (Color::Srgba(Srgba { red: 0.3, green: 0.3, blue: 0.3, alpha: 1.0 })),
            Scrollbar::new(
                ScrollbarDirection::Vertical,
                scroll_area_id,
            ),
        ));
    });
}

fn update_scroll_position(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    mut scrolled_node_query: Query<&mut ScrollPosition>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        let (mut dx, mut dy) = match mouse_wheel_event.unit {
            MouseScrollUnit::Line => (
                mouse_wheel_event.x * 21.0,
                mouse_wheel_event.y * 21.0,
            ),
            MouseScrollUnit::Pixel => (mouse_wheel_event.x, mouse_wheel_event.y),
        };

        if keyboard_input.pressed(KeyCode::ControlLeft) || keyboard_input.pressed(KeyCode::ControlRight) {
            std::mem::swap(&mut dx, &mut dy);
        }

        for (_pointer, pointer_map) in hover_map.iter() {
            for (entity, _hit) in pointer_map.iter() {
                if let Ok(mut scroll_position) = scrolled_node_query.get_mut(*entity) {
                    scroll_position.offset_x -= dx;
                    scroll_position.offset_y -= dy;
                }
            }
        }
    }
}

fn change_ui_scale(
    mut ui_scale: ResMut<UiScale>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::PageUp) {
        ui_scale.0 += ui_scale.0/10.0;
    }
    if keyboard_input.just_pressed(KeyCode::PageDown) {
        ui_scale.0 -= ui_scale.0/10.0;
    }
    ui_scale.0 = ui_scale.0.clamp(0.25, 3.0)
}