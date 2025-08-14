use bevy::prelude::*;

pub(crate) struct InputProtocolPlugin;

impl Plugin for InputProtocolPlugin {
    fn build(&self, app: &mut App) {
        debug!("Inputs registered");
    }
}

// /// The different directions that the player can move the box
// #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
// pub struct Direction {
//     pub(crate) up: bool,
//     pub(crate) down: bool,
//     pub(crate) left: bool,
//     pub(crate) right: bool,
// }

// #[derive(Serialize, Deserialize, Debug, PartialEq, Reflect, Eq, Clone)]
// pub enum Inputs {
//     Direction(Direction),
//     Delete,
//     Spawn,
// }

// // All inputs need to implement the `MapEntities` trait
// impl MapEntities for Inputs {
//     fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {}
// }

// impl Plugin for ProtocolPlugin{
//   fn build(&self, app: &mut App) {
//     app.add_plugins(InputPlugin::<Inputs>::default());
//   }
// }
