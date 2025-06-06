use anyhow::{anyhow, bail};
use serde::{Deserialize, Serialize};
use sov_modules_api::{Context, Spec, StateMap, TxState, WorkingSet};

use crate::{utils::get_user_address};

#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Deserialize , Serialize, Debug, PartialEq, Clone , Eq)]
/// Defines an nft collection
pub struct User<S: Spec> {
    username: String,
    karma: u64,
    user_address: S::Address
}



impl<S: Spec> User<S> {
 pub fn new(
    username: &str,
    user_collections: &StateMap<S::Address , User<S>>,
    context: &Context<S>,
        state: &mut impl TxState<S>,
 ) -> anyhow::Result<(S::Address, User<S>)> {


    let creator = context.sender();

    let user_address = get_user_address::<S>(username, creator.as_ref());


    let user_add = user_collections.get(&user_address, state)?;

    if user_add.is_some() {
        Err(anyhow!( "User with username={} already exists", username ))
    } else {
        Ok( (user_address.clone() , User { username: username.to_string(), karma: 0, user_address:  user_address.clone()}))
    }


 }

 #[allow(dead_code)]
 pub fn get_username(&self) -> &str {
    &self.username
 }

#[allow(dead_code)]
 pub fn get_karma(&self) -> u64 {
    self.karma
 }


 #[allow(dead_code)]
 pub fn get_user_address(&self) -> S::Address{
    self.user_address.clone()
 }

}