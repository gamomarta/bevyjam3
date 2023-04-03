use bevy::prelude::*;

#[derive(Default, Deref, DerefMut, Resource)]
pub(super) struct AssetsLoading(Vec<HandleUntyped>);
