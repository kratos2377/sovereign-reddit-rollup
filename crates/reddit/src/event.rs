use borsh::{BorshDeserialize, BorshSerialize};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    BorshDeserialize, BorshSerialize, Serialize, Deserialize, JsonSchema, Debug, PartialEq, Clone,
)]
#[serde(rename_all = "snake_case")]
pub enum Event {
    /// Example event set value
    UserCreatedEvent { username: String, user_address: String  },


    SubRedditCreatedEvent { subname: String , description: String, subaddress: String , mods: Vec<String> },

    PostCreatedEvent {
        title: String,
        content: String,
        subaddress: String,
        post_address: String
    }
}
