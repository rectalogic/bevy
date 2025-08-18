use alloc::vec::Vec;

use crate::{AsAssetId, Asset, AssetId};
use bevy_ecs::{
    bundle::Bundle,
    component::Component,
    entity::Entity,
    relationship::Relationship,
    system::{Commands, EntityCommands},
    world::EntityWorldMut,
};

#[derive(Component, Debug)]
#[relationship(relationship_target = Dependency)]
pub struct Dependent {
    #[relationship]
    pub dependency: Entity,
}

#[derive(Component, Debug, Default)]
#[relationship_target(relationship = Dependent, linked_spawn)]
pub struct Dependency {
    #[relationship]
    dependents: Vec<Entity>,
}

impl Dependency {
    pub fn dependents(&self) -> &[Entity] {
        &self.dependents
    }
}

#[derive(Component, Debug)]
pub struct AssetDependency<A: Asset>(AssetId<A>);

impl<A: Asset> AsAssetId for AssetDependency<A> {
    type Asset = A;

    fn as_asset_id(&self) -> AssetId<Self::Asset> {
        self.0
    }
}

#[derive(Component, Debug)]
pub struct AssetDependent<A: Asset>(AssetId<A>);

impl<A: Asset> AsAssetId for AssetDependent<A> {
    type Asset = A;

    fn as_asset_id(&self) -> AssetId<Self::Asset> {
        self.0
    }
}

#[derive(Component, Debug)]
pub struct AssetDependencyChanged;

pub trait CommandsAssetDependencyExt {
    fn spawn_asset_dependency<A: Asset, AI: Into<AssetId<A>>>(
        &mut self,
        dependency_asset_id: AI,
    ) -> EntityCommands<'_>;
}

impl CommandsAssetDependencyExt for Commands<'_, '_> {
    fn spawn_asset_dependency<A: Asset, AI: Into<AssetId<A>>>(
        &mut self,
        dependency_asset_id: AI,
    ) -> EntityCommands<'_> {
        self.spawn((
            Dependency::default(),
            AssetDependency(dependency_asset_id.into()),
        ))
    }
}

pub trait BuildAssetDependencyExt {
    fn with_dependent_asset<A: Asset, AI: Into<AssetId<A>>>(
        &mut self,
        dependent_asset_id: AI,
        bundle: impl Bundle,
    ) -> &mut Self;
}

impl BuildAssetDependencyExt for EntityCommands<'_> {
    fn with_dependent_asset<A: Asset, AI: Into<AssetId<A>>>(
        &mut self,
        dependent_asset_id: AI,
        bundle: impl Bundle,
    ) -> &mut Self {
        let dependency = self.id();
        self.commands_mut().spawn((
            <Dependent as Relationship>::from(dependency),
            AssetDependent(dependent_asset_id.into()),
            bundle,
        ));
        self
    }
}

impl BuildAssetDependencyExt for EntityWorldMut<'_> {
    fn with_dependent_asset<A: Asset, AI: Into<AssetId<A>>>(
        &mut self,
        dependent_asset_id: AI,
        bundle: impl Bundle,
    ) -> &mut Self {
        let dependency = self.id();
        self.world_scope(|world| {
            world.spawn((
                <Dependent as Relationship>::from(dependency),
                AssetDependent(dependent_asset_id.into()),
                bundle,
            ));
        });
        self
    }
}
