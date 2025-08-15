use alloc::vec::Vec;

use crate::{AsAssetId, Asset, AssetId};
use bevy_ecs::{
    bundle::Bundle,
    component::Component,
    prelude::Entity,
    system::{Commands, EntityCommands},
    world::EntityWorldMut,
};

#[derive(Component, Debug)]
#[relationship(relationship_target = AssetDependency<A>)]
pub struct AssetDependent<A: Asset> {
    #[relationship]
    pub dependency: Entity,
    asset_id: AssetId<A>,
}

impl<A: Asset> AssetDependent<A> {
    pub fn new(dependency: Entity, asset_id: AssetId<A>) -> Self {
        Self {
            dependency,
            asset_id,
        }
    }

    pub fn asset_id(&self) -> AssetId<A> {
        self.asset_id
    }
}

impl<A: Asset> AsAssetId for AssetDependent<A> {
    type Asset = A;

    fn as_asset_id(&self) -> AssetId<Self::Asset> {
        self.asset_id
    }
}

#[derive(Component, Debug)]
#[relationship_target(relationship = AssetDependent<A>, linked_spawn)]
pub struct AssetDependency<A: Asset> {
    #[relationship]
    dependents: Vec<Entity>,
    asset_id: AssetId<A>,
}

impl<A: Asset> AssetDependency<A> {
    pub fn asset_id(&self) -> AssetId<A> {
        self.asset_id
    }

    pub fn dependents(&self) -> &[Entity] {
        &self.dependents
    }
}

impl<A: Asset> From<AssetId<A>> for AssetDependency<A> {
    fn from(asset_id: AssetId<A>) -> Self {
        Self {
            asset_id,
            dependents: Vec::default(),
        }
    }
}

impl<A: Asset> AsAssetId for AssetDependency<A> {
    type Asset = A;

    fn as_asset_id(&self) -> AssetId<Self::Asset> {
        self.asset_id
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
        self.spawn(AssetDependency::<A>::from(dependency_asset_id.into()))
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
            AssetDependent::<A>::new(dependency, dependent_asset_id.into()),
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
                AssetDependent::<A>::new(dependency, dependent_asset_id.into()),
                bundle,
            ));
        });
        self
    }
}
