# opengr2-bevy
A Granny2 model loader plugin for Bevy.

This library is currently under heavy development and doesn't support all features.

## Usage
1. Add to ``Cargo.toml``
    ```toml
    [dependencies]
    opengr2-bevy = "0.9"
    ```
2. Include the GrannyPlugin
    ```rust
    use opengr2_bevy::GrannyPlugin;
    ```
3. Add the Plugin to your app
    ```rust
    fn main() {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugin(GrannyPlugin)
            .run();
    } 
    ```
4. Load gr2 file
    ```rust
    let handle = server.load("test.gr2#default");
    commands.spawn(SceneBundle {
        scene: handle,
        ..default()
    });
    ```
    **Note:** We load the models in a Granny2 file as different scenes labeled with their model name.

## Features
| Functionality     | Status                                           |
|-------------------|--------------------------------------------------|
| Static models     | ⚠️ (Multiple meshes per model not supported yet) |
| Animations        | ❌                                                |
| External textures | ✔️                                               |
| Embedded textures | ❌                                                |

Also look at the [supported features](https://github.com/NoFr1ends/opengr2-rs/blob/main/README.md#features) of the 
underlying parsing library.

## Support
opengr2-bevy crate version follows bevy's version as shown:

| bevy  | opengr2-bevy |
|-------|--------------|
| 0.X.Y | 0.X          |