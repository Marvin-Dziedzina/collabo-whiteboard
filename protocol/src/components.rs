use bevy::prelude::*;

pub(crate) struct ComponentsProtocolPlugin;

impl Plugin for ComponentsProtocolPlugin {
    fn build(&self, app: &mut App) {
        debug!("Components registered");
    }
}

// /// A component that will identify which player the box belongs to
// #[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
// pub struct PlayerId(ClientId);

// /// A component that will store the position of the box. We could also directly use the `Transform` component.
// #[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq)]
// pub struct PlayerPosition(Vec2);

// /// A component that will store the color of the box, so that each player can have a different color.
// #[derive(Component, Deserialize, Serialize, Clone, Debug, PartialEq)]
// pub struct PlayerColor(pub(crate) Color);

// pub struct ProtocolPlugin;

// impl Plugin for ProtocolPlugin{
//     fn build(&self, app: &mut App) {
//         app.register_component::<PlayerId>();

//         app.register_component::<PlayerPosition>();

//         app.register_component::<PlayerColor>();
//     }
// }
