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
#[relationship(relationship_target = Dependencies)]
pub struct DependencyOf {
    #[relationship]
    pub dependent: Entity,
}

#[derive(Component, Debug, Default)]
#[relationship_target(relationship = DependencyOf, linked_spawn)]
pub struct Dependencies {
    #[relationship]
    dependencies: Vec<Entity>,
}

impl Dependencies {
    pub fn dependencies(&self) -> &[Entity] {
        &self.dependencies
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
    fn spawn_asset<A: Asset, AI: Into<AssetId<A>>>(&mut self, asset_id: AI) -> EntityCommands<'_>;
}

impl CommandsAssetDependencyExt for Commands<'_, '_> {
    fn spawn_asset<A: Asset, AI: Into<AssetId<A>>>(&mut self, asset_id: AI) -> EntityCommands<'_> {
        self.spawn((Dependencies::default(), AssetDependent(asset_id.into())))
    }
}

pub trait BuildAssetDependencyExt {
    fn with_asset_dependency<A: Asset, AI: Into<AssetId<A>>>(
        &mut self,
        asset_id: AI,
        bundle: impl Bundle,
    ) -> &mut Self;
}

impl BuildAssetDependencyExt for EntityCommands<'_> {
    fn with_asset_dependency<A: Asset, AI: Into<AssetId<A>>>(
        &mut self,
        asset_id: AI,
        bundle: impl Bundle,
    ) -> &mut Self {
        let dependent = self.id();
        self.commands_mut().spawn((
            <DependencyOf as Relationship>::from(dependent),
            AssetDependency(asset_id.into()),
            bundle,
        ));
        self
    }
}

impl BuildAssetDependencyExt for EntityWorldMut<'_> {
    fn with_asset_dependency<A: Asset, AI: Into<AssetId<A>>>(
        &mut self,
        asset_id: AI,
        bundle: impl Bundle,
    ) -> &mut Self {
        let dependent = self.id();
        self.world_scope(|world| {
            world.spawn((
                <DependencyOf as Relationship>::from(dependent),
                AssetDependency(asset_id.into()),
                bundle,
            ));
        });
        self
    }
}
