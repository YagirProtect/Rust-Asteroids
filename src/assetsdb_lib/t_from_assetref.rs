use crate::assetsdb_lib::e_asset::Asset;

pub trait FromAssetRef: Sized {
    fn from_asset(a: &Asset) -> Option<&Self>;
}