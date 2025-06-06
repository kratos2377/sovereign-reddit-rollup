use crate::{utils::{get_post_address, get_sub_address, get_user_address}, Reddit};
use sov_modules_api::{prelude::jsonrpsee::core::RpcResult, ApiStateAccessor, Context, Spec, WorkingSet};
use sov_modules_api::macros::rpc_gen;
use sov_modules_api::prelude::jsonrpsee;
use sov_modules_api::Address;


#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(bound(
    serialize = "S::Address: serde::Serialize",
    deserialize = "S::Address: serde::Deserialize<'de>"
))]
/// Response for `getCollection` method
pub struct UserCollectionResponse<S: Spec> {
    pub username: String,
    pub user_address: S::Address,
}


#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(bound(
    serialize = "S::Address: serde::Serialize",
    deserialize = "S::Address: serde::Deserialize<'de>"
))]
/// Response for `getCollectionAddress` method
pub struct UserAddressResponse<S: Spec> {
    /// Address of the collection
    pub user_address: S::Address,
}



#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(bound(
    serialize = "S::Address: serde::Serialize",
    deserialize = "S::Address: serde::Deserialize<'de>"
))]
pub struct SubRedditCollectionResponse<S: Spec> {
    pub subname: String,
    pub desription: String,
    pub subaddress: S::Address,
    pub mods: Vec<S::Address>
}


#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(bound(
    serialize = "S::Address: serde::Serialize",
    deserialize = "S::Address: serde::Deserialize<'de>"
))]
/// Response for `getCollectionAddress` method
pub struct SubAddressResponse<S: Spec> {
    /// Address of the collection
    pub sub_address: S::Address,
}




#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(bound(
    serialize = "S::Address: serde::Serialize",
    deserialize = "S::Address: serde::Deserialize<'de>"
))]
pub struct PostCollectionResponse<S: Spec> {
    pub user_address: S::Address,
    pub sub_address: S::Address,
    pub post_address: S::Address,
    pub post_title: String,
    pub content: String,
    pub flair: String,
    pub status: String
   
}


#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(bound(
    serialize = "S::Address: serde::Serialize",
    deserialize = "S::Address: serde::Deserialize<'de>"
))]
pub struct PostAddressResponse<S: Spec> {
    /// Address of the collection
    pub sub_address: S::Address,
}



#[rpc_gen(client, server, namespace = "reddit")]
impl<S: Spec> Reddit<S> {
    #[rpc_method(name = "getUser")]
    /// Get the collection details
    pub fn get_user(
        &self,
        user_address: S::Address,
        state: &mut ApiStateAccessor<S>,
    ) -> RpcResult<UserCollectionResponse<S>> {
        let c = self
            .user_collections
            .get(&user_address, state)
            .unwrap();

        Ok(UserCollectionResponse {
            username: c.unwrap().get_username().to_string(),
            user_address: user_address.clone(),
        })
    }
    #[rpc_method(name = "getUserAddress")]
    /// Get the collection address
    pub fn get_collection_address(
        &self,
        user_add: S::Address,
        username: &str,
        state: &mut ApiStateAccessor<S>,
    ) -> RpcResult<UserAddressResponse<S>> {
        let ca = get_user_address::<S>(username, user_add.as_ref());
        Ok(UserAddressResponse {
            user_address: ca,
        })
    }





    #[rpc_method(name = "getSubreddit")]
    pub fn get_sub_reddit(
        &self,
        sub_address: S::Address,
        state: &mut ApiStateAccessor<S>,
    ) -> RpcResult<SubRedditCollectionResponse<S>> {
        let c = self
            .sub_collections
            .get(&sub_address, state)
            .unwrap().unwrap();

        Ok(SubRedditCollectionResponse { 
            subname: c.get_sub_name().to_string(), 
            desription: c.get_sub_description().to_string(), 
            subaddress: c.get_sub_address().clone(), 
            mods: c.get_mods().clone() 
        })
    }
    #[rpc_method(name = "getSubAddress")]
    pub fn get_sub_address(
        &self,
        suname: &str,
        state: &mut ApiStateAccessor<S>,
    ) -> RpcResult<SubAddressResponse<S>> {
        let ca = get_sub_address::<S>(suname);
        Ok(SubAddressResponse {
            sub_address: ca,
        })
    }




        #[rpc_method(name = "getPost")]
    pub fn get_post(
        &self,
        post_address: S::Address,
        state: &mut ApiStateAccessor<S>,
    ) -> RpcResult<PostCollectionResponse<S>> {
        let c = self
            .post_collections
            .get(&post_address, state).unwrap().unwrap();

        Ok(PostCollectionResponse { 
            user_address: c.get_user_address().clone(), 
            sub_address: c.get_sub_address().clone(), 
            post_address: c.get_post_address().clone(), 
            post_title: c.get_post_title().to_string(), 
            content: c.get_post_content().to_string(), 
            flair: c.get_post_flair().to_string(), 
            status: c.get_post_status().to_string() 
        })
    }

}
