use crate::package::ContractInfoResponse;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, StdResult, Storage};
use cw20::Cw20Coin;
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, MultiIndex};

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Offering {
    pub token_id: String,
    pub contract_addr: Addr,
    pub seller: Addr,
    pub list_price: Cw20Coin,
}

/// OFFERINGS is a map which maps the offering_id to an offering. Offering_id is derived from OFFERINGS_COUNT.
pub const OFFERINGS: Map<&str, Offering> = Map::new("offerings");
pub const OFFERINGS_COUNT: Item<u64> = Item::new("num_offerings");
pub const CONTRACT_INFO: Item<ContractInfoResponse> = Item::new("marketplace_info");

pub fn num_offerings(storage: &dyn Storage) -> StdResult<u64> {
    Ok(OFFERINGS_COUNT.may_load(storage)?.unwrap_or_default())
}

pub fn increment_offerings(storage: &mut dyn Storage) -> StdResult<u64> {
    let val = num_offerings(storage)? + 1;
    OFFERINGS_COUNT.save(storage, &val)?;
    Ok(val)
}

pub struct OfferingIndexes<'a> {
    pub seller: MultiIndex<'a, (Addr, Vec<u8>), Offering>,
    pub contract: MultiIndex<'a, (Addr, Vec<u8>), Offering>,
}

impl<'a> IndexList<Offering> for OfferingIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Offering>> + '_> {
        let v: Vec<&dyn Index<Offering>> = vec![&self.seller, &self.contract];
        Box::new(v.into_iter())
    }
}

pub fn offerings<'a>() -> IndexedMap<'a, &'a str, Offering, OfferingIndexes<'a>> {
    let indexes = OfferingIndexes {
        seller: MultiIndex::new(|d: &Offering, k: Vec<u8>| (d.seller.clone(), k), "offerings", "offerings__seller"),
        contract: MultiIndex::new(
            |d: &Offering, k: Vec<u8>| (d.contract_addr.clone(), k),
            "offerings",
            "offerings__contract",
        ),
    };
    IndexedMap::new("offerings", indexes)
}
