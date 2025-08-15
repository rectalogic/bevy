use alloc::vec::Vec;

use crate::{AsAssetId, Asset, AssetId};
use bevy_ecs::{
    component::Component, prelude::Entity, system::EntityCommands, world::EntityWorldMut,
};

#[derive(Component, Debug)]
#[relationship(relationship_target = AssetDependent<A>)]
pub struct AssetDependencyOf<A: Asset> {
    #[relationship]
    pub dependent: Entity,
    asset_id: AssetId<A>,
}

impl<A: Asset> AssetDependencyOf<A> {
    pub fn new(dependent: Entity, asset_id: AssetId<A>) -> Self {
        Self {
            dependent,
            asset_id,
        }
    }

    pub fn asset_id(&self) -> AssetId<A> {
        self.asset_id
    }
}

impl<A: Asset> AsAssetId for AssetDependencyOf<A> {
    type Asset = A;

    fn as_asset_id(&self) -> AssetId<Self::Asset> {
        self.asset_id
    }
}

#[derive(Component, Debug)]
#[relationship_target(relationship = AssetDependencyOf<A>, linked_spawn)]
pub struct AssetDependent<A: Asset> {
    #[relationship]
    dependencies: Vec<Entity>,
    asset_id: AssetId<A>,
}

impl<A: Asset> AssetDependent<A> {
    pub fn asset_id(&self) -> AssetId<A> {
        self.asset_id
    }
}

impl<A: Asset> From<AssetId<A>> for AssetDependent<A> {
    fn from(asset_id: AssetId<A>) -> Self {
        Self {
            asset_id,
            dependencies: Vec::default(),
        }
    }
}

impl<A: Asset> AsAssetId for AssetDependent<A> {
    type Asset = A;

    fn as_asset_id(&self) -> AssetId<Self::Asset> {
        self.asset_id
    }
}

#[derive(Component, Debug)]
pub struct AssetDependencyChanged;

pub trait BuildAssetDependencyExt {
    fn insert_dependent_asset<A: Asset, AI: Into<AssetId<A>>>(
        &mut self,
        dependent_asset_id: AI,
    ) -> &mut Self;

    fn with_asset_dependency<A: Asset, AI: Into<AssetId<A>>>(
        &mut self,
        dependency_asset_id: AI,
    ) -> &mut Self;
}

impl BuildAssetDependencyExt for EntityCommands<'_> {
    fn insert_dependent_asset<A: Asset, AI: Into<AssetId<A>>>(
        &mut self,
        dependent_asset_id: AI,
    ) -> &mut Self {
        self.insert(AssetDependent::<A>::from(dependent_asset_id.into()));
        self
    }

    fn with_asset_dependency<A: Asset, AI: Into<AssetId<A>>>(
        &mut self,
        dependency_asset_id: AI,
    ) -> &mut Self {
        let dependent = self.id();
        self.commands_mut()
            .spawn::<AssetDependencyOf<A>>(AssetDependencyOf::new(
                dependent,
                dependency_asset_id.into(),
            ));
        self
    }
}

impl BuildAssetDependencyExt for EntityWorldMut<'_> {
    fn insert_dependent_asset<A: Asset, AI: Into<AssetId<A>>>(
        &mut self,
        dependent_asset_id: AI,
    ) -> &mut Self {
        self.insert(AssetDependent::<A>::from(dependent_asset_id.into()));
        self
    }

    fn with_asset_dependency<A: Asset, AI: Into<AssetId<A>>>(
        &mut self,
        dependency_asset_id: AI,
    ) -> &mut Self {
        let dependent = self.id();
        self.world_scope(|world| {
            world.spawn(AssetDependencyOf::new(
                dependent,
                dependency_asset_id.into(),
            ));
        });
        self
    }
}
