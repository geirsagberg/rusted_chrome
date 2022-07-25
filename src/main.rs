use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(animate_sprites)
        .run();
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Aim {
    direction_radians: f32,
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tile_size = Vec2::new(48., 48.);
    let columns = 4;
    let rows = 1;
    let cyborg_idle_atlas = load_texture_atlas(
        &assets,
        texture_atlases,
        "cyborg/Idle1.png",
        tile_size,
        columns,
        rows,
    );
    let cyborg_hand = assets.load("cyborg/hands/3.png");
    let gun_10 = assets.load("guns/10_1.png");

    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale /= 2.;
    commands.spawn_bundle(camera);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: cyborg_idle_atlas,
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteBundle {
                    texture: cyborg_hand,
                    transform: Transform::from_xyz(-6., 0., -0.1),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(SpriteBundle {
                        texture: gun_10,
                        transform: Transform::from_xyz(10., 1., 0.1),
                        ..default()
                    });
                });
        });
}

fn load_texture_atlas(
    assets: &Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    path: &str,
    tile_size: Vec2,
    columns: usize,
    rows: usize,
) -> Handle<TextureAtlas> {
    let texture_handle = assets.load(path);

    let texture_atlas = TextureAtlas::from_grid(texture_handle, tile_size, columns, rows);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    texture_atlas_handle
}

fn animate_sprites(
    time: Res<Time>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}
