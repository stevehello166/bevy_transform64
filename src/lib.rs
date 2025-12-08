use bevy::{ecs::schedule::ScheduleLabel, math::*, prelude::*};

pub mod commands;
pub mod components;
pub mod systems;

use systems::*;

#[doc(hidden)]
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{commands::BuildChildrenDTransformExt, components::*, DTransformPlugin};
}

#[derive(Resource, Clone, Debug)]
pub enum WorldOrigin {
    Entity(Entity),
    Position(DVec3),
}

#[derive(Resource, Clone, Debug)]
pub struct SimpleWorldOrigin {
    pub origin: DVec3,
}

use prelude::{DGlobalTransform, DTransform};

/// Set enum for the systems relating to transform propagation
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet, ScheduleLabel)]
pub enum DTransformSystem {
    /// Propagates changes in transform to children's [`GlobalTransform`](crate::components::GlobalTransform)
    TransformPropagate,
}

/// The base plugin for handling [`Transform`] components
#[derive(Default)]
pub struct DTransformPlugin;

impl Plugin for DTransformPlugin {
    fn build(&self, app: &mut App) {
        // A set for `propagate_transforms` to mark it as ambiguous with `sync_simple_transforms`.
        // Used instead of the `SystemTypeSet` as that would not allow multiple instances of the system.
        #[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
        struct PropagateTransformsSet;

        #[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
        struct SyncTransforms;

        app.register_type::<DTransform>()
            .register_type::<DGlobalTransform>()
            .insert_resource(WorldOrigin::Position(DVec3::ZERO))
            .insert_resource(SimpleWorldOrigin {
                origin: DVec3::ZERO,
            })
            // add transform systems to startup so the first update is "correct"
            .configure_sets(PostUpdate, DTransformSystem::TransformPropagate)
            .configure_sets(
                PostUpdate,
                SyncTransforms
                    .after(DTransformSystem::TransformPropagate)
                    .after(bevy::transform::TransformSystems::Propagate),
            )
            .configure_sets(
                PostUpdate,
                PropagateTransformsSet.in_set(DTransformSystem::TransformPropagate),
            )
            .edit_schedule(Startup, |schedule| {
                schedule.configure_sets(DTransformSystem::TransformPropagate);
            })
            .add_systems(
                Startup,
                (
                    sync_simple_transforms
                        .in_set(DTransformSystem::TransformPropagate)
                        // FIXME: https://github.com/bevyengine/bevy/issues/4381
                        // These systems cannot access the same entities,
                        // due to subtle query filtering that is not yet correctly computed in the ambiguity detector
                        .ambiguous_with(PropagateTransformsSet),
                    propagate_transforms.in_set(PropagateTransformsSet),
                ),
            )
            .add_systems(
                PostUpdate,
                sync_simple_transforms
                    .ambiguous_with(PropagateTransformsSet)
                    .in_set(DTransformSystem::TransformPropagate),
            )
            .add_systems(
                PostUpdate,
                propagate_transforms.in_set(PropagateTransformsSet),
            )
            .add_systems(PostUpdate, sync_f64_f32.in_set(SyncTransforms))
            .add_systems(
                PostUpdate,
                convert_world_origin
                    .after(sync_simple_transforms)
                    .in_set(DTransformSystem::TransformPropagate),
            );
    }
}
