pub const CHAIN_HASH: [u8; 32] = [49, 144, 196, 200, 162, 254, 44, 35, 57, 225, 235, 14, 127, 231, 223, 111, 213, 38, 72, 109, 106, 123, 138, 213, 1, 39, 28, 252, 41, 217, 148, 226];

#[allow(dead_code)]
pub const SCHEMA_JSON: &str = r#"{
  "types": [
    {
      "Struct": {
        "type_name": "Transaction",
        "template": null,
        "peekable": false,
        "fields": [
          {
            "display_name": "versioned_tx",
            "silent": false,
            "value": {
              "ByIndex": 1
            },
            "doc": ""
          }
        ]
      }
    },
    {
      "Enum": {
        "type_name": "VersionedTx",
        "variants": [
          {
            "name": "V0",
            "discriminant": 0,
            "template": null,
            "value": {
              "ByIndex": 2
            }
          }
        ],
        "hide_tag": false
      }
    },
    {
      "Tuple": {
        "template": null,
        "peekable": false,
        "fields": [
          {
            "value": {
              "ByIndex": 3
            },
            "silent": false,
            "doc": ""
          }
        ]
      }
    },
    {
      "Struct": {
        "type_name": "Version0",
        "template": null,
        "peekable": false,
        "fields": [
          {
            "display_name": "signature",
            "silent": false,
            "value": {
              "ByIndex": 4
            },
            "doc": ""
          },
          {
            "display_name": "pub_key",
            "silent": false,
            "value": {
              "ByIndex": 5
            },
            "doc": ""
          },
          {
            "display_name": "runtime_call",
            "silent": false,
            "value": {
              "ByIndex": 6
            },
            "doc": ""
          },
          {
            "display_name": "generation",
            "silent": false,
            "value": {
              "Immediate": {
                "Integer": [
                  "u64",
                  "Decimal"
                ]
              }
            },
            "doc": ""
          },
          {
            "display_name": "details",
            "silent": false,
            "value": {
              "ByIndex": 83
            },
            "doc": ""
          }
        ]
      }
    },
    {
      "Struct": {
        "type_name": "Ed25519Signature",
        "template": null,
        "peekable": false,
        "fields": [
          {
            "display_name": "msg_sig",
            "silent": false,
            "value": {
              "Immediate": {
                "ByteArray": {
                  "len": 64,
                  "display": "Hex"
                }
              }
            },
            "doc": ""
          }
        ]
      }
    },
    {
      "Struct": {
        "type_name": "Ed25519PublicKey",
        "template": null,
        "peekable": false,
        "fields": [
          {
            "display_name": "pub_key",
            "silent": false,
            "value": {
              "Immediate": {
                "ByteArray": {
                  "len": 32,
                  "display": "Hex"
                }
              }
            },
            "doc": ""
          }
        ]
      }
    },
    {
      "Enum": {
        "type_name": "RuntimeCall",
        "variants": [
          {
            "name": "Accounts",
            "discriminant": 0,
            "template": null,
            "value": {
              "ByIndex": 7
            }
          },
          {
            "name": "Uniqueness",
            "discriminant": 1,
            "template": null,
            "value": {
              "ByIndex": 12
            }
          },
          {
            "name": "Bank",
            "discriminant": 2,
            "template": null,
            "value": {
              "ByIndex": 14
            }
          },
          {
            "name": "SequencerRegistry",
            "discriminant": 3,
            "template": null,
            "value": {
              "ByIndex": 28
            }
          },
          {
            "name": "AttesterIncentives",
            "discriminant": 4,
            "template": null,
            "value": {
              "ByIndex": 35
            }
          },
          {
            "name": "ProverIncentives",
            "discriminant": 5,
            "template": null,
            "value": {
              "ByIndex": 40
            }
          },
          {
            "name": "ExampleModule",
            "discriminant": 6,
            "template": null,
            "value": {
              "ByIndex": 44
            }
          },
          {
            "name": "RedditModule",
            "discriminant": 7,
            "template": null,
            "value": {
              "ByIndex": 47
            }
          },
          {
            "name": "ChainState",
            "discriminant": 8,
            "template": null,
            "value": {
              "ByIndex": 52
            }
          },
          {
            "name": "BlobStorage",
            "discriminant": 9,
            "template": null,
            "value": {
              "ByIndex": 53
            }
          },
          {
            "name": "Paymaster",
            "discriminant": 10,
            "template": null,
            "value": {
              "ByIndex": 54
            }
          }
        ],
        "hide_tag": false
      }
    },
    {
      "Tuple": {
        "template": null,
        "peekable": false,
        "fields": [
          {
            "value": {
              "ByIndex": 8
            },
            "silent": false,
            "doc": ""
          }
        ]
      }
    },
    {
      "Enum": {
        "type_name": "CallMessage",
        "variants": [
          {
            "name": "InsertCredentialId",
            "discriminant": 0,
            "template": null,
            "value": {
              "ByIndex": 9
            }
          }
        ],
        "hide_tag": false
      }
    },
    {
      "Tuple": {
        "template": null,
        "peekable": false,
        "fields": [
          {
            "value": {
              "ByIndex": 10
            },
            "silent": false,
            "doc": ""
          }
        ]
      }
    },
    {
      "Tuple": {
        "template": null,
        "peekable": false,
        "fields": [
          {
            "value": {
              "ByIndex": 11
            },
            "silent": false,
            "doc": ""
          }
        ]
      }
    },
    {
      "Tuple": {
        "template": null,
        "peekable": false,
        "fields": [
          {
            "value": {
              "Immediate": {
                "ByteArray": {
                  "len": 32,
                  "display": "Hex"
                }
              }
            },
            "silent": false,
            "doc": ""
          }
        ]
      }
    },
    {
      "Tuple": {
        "template": null,
        "peekable": false,
        "fields": [
          {
            "value": {
              "ByIndex": 13
            },
            "silent": false,
            "doc": ""
          }
        ]
      }
    },
    {
      "Enum": {
        "type_name": "NotInstantiable",
        "variants": [],
        "hide_tag": false
      }
    },
    {
      "Tuple": {
        "template": null,
        "peekable": false,
        "fields": [
          {
            "value": {
              "ByIndex": 15
            },
            "silent": false,
            "doc": ""
          }
        ]
      }
    },
    {
      "Enum": {
        "type_name": "CallMessage",
        "variants": [
          {
            "name": "CreateToken",
            "discriminant": 0,
            "template": null,
            "value": {
              "ByIndex": 16
            }
          },
          {
            "name": "Transfer",
            "discriminant": 1,
            "template": "Transfer to address {} {}.",
            "value": {
              "ByIndex": 22
            }
          },
          {
            "name": "Burn",
            "discriminant": 2,
            "template": null,
            "value": {
              "ByIndex": 25
            }
          },
          {
            "name": "Mint",
            "discriminant": 3,
            "template": null,
            "value": {
              "ByIndex": 26
            }
          },
          {
            "name": "Freeze",
            "discriminant": 4,
            "template": null,
            "value": {
              "ByIndex": 27
            }
          }
        ],
        "hide_tag": false
      }
    },
    {
      "Struct": {
        "type_name": "__SovVirtualWallet_CallMessage_CreateToken",
        "template": null,
        "peekable": false,
        "fields": [
          {
            "display_name": "token_name",
            "silent": false,
            "value": {
              "Immediate": "String"
            },
            "doc": ""
          },
          {
            "display_name": "token_decimals",
            "silent": false,
            "value": {
              "ByIndex": 17
            },
            "doc": ""
          },
          {
            "display_name": "initial_balance",
            "silent": false,
            "value": {
              "ByIndex": 18
            },
            "doc": ""
          },
          {
            "display_name": "mint_to_address",
            "silent": false,
            "value": {
              "ByIndex": 19
            },
            "doc": ""
          },
          {
            "display_name": "admins",
            "silent": false,
            "value": {
              "ByIndex": 20
            },
            "doc": ""
          },
          {
            "display_name": "supply_cap",
            "silent": false,
            "value": {
              "ByIndex": 21
            },
            "doc": ""
          }
        ]
      }
    },
    {
      "Option": {
        "value": {
          "Immediate": {
            "Integer": [
              "u8",
              "Decimal"
            ]
          }
        }
      }
    },
    {
      "Tuple": {
        "template": null,
        "peekable": false,
        "fields": [
          {
            "value": {
              "Immediate": {
                "Integer": [
                  "u128",
                  "Decimal"
                ]
              }
            },
            "silent": false,
            "doc": ""
          }
        ]
      }
    },
    {
      "Tuple": {
        "template": null,
        "peekable": false,
        "fields": [
          {
            "value": {
              "Immediate": {
                "ByteArray": {
                  "len": 32,
                  "display": "Base58"
                }
              }
            },
            "silent": false,
            "doc": ""
          }
        ]
      }
    },
    {
      "Vec": {
        "value": {
          "ByIndex": 19
        }
      }
    },
    {
      "Option": {
        "value": {
          "ByIndex": 18
        }
      }
    },
    {
      "Struct": {
        "type_name": "__SovVirtualWallet_CallMessage_Transfer",
        "template": "Transfer to address {} {}.",
        "peekable": false,
        "fields": [
          {
            "display_name": "to",
            "silent": false,
            "value": {
              "ByIndex": 19
            },
            "doc": ""
          },
          {
            "display_name": "coins",
            "silent": false,
            "value": {
              "ByIndex": 23
            },
            "doc": ""
          }
        ]
      }
    },
    {
      "Struct": {
        "type_name": "Coins",
        "template": "{} coins of token ID {}",
        "peekable": true,
        "fields": [
          {
            "display_name": "amount",
            "silent": false,
            "value": {
              "Immediate": {
                "Integer": [
                  "u128",
                  {
                    "FixedPoint": {
                      "FromSiblingField": {
                        "field_index": 1,
                        "byte_offset": 31
                      }
                    }
                  }
                ]
              }
            },
            "doc": ""
          },
          {
            "display_name": "token_id",
            "silent": false,
            "value": {
              "ByIndex": 24
            },
            "doc": ""
          }
        ]
      }
    },
    {
      "Tuple": {
        "template": null,
        "peekable": false,
        "fields": [
          {
            "value": {
              "Immediate": {
                "ByteArray": {
                  "len": 32,
                  "display": {
                    "Bech32m": {
                      "prefix": "token_"
                    }
                  }
                }
              }
            },
            "silent": false,
            "doc": ""
          }
        ]
      }
    },
    {
      "Struct": {
        "type_name": "__SovVirtualWallet_CallMessage_Burn",
        "template": null,
        "peekable": false,
        "fields": [
          {
            "display_name": "coins",
            "silent": false,
            "value": {
              "ByIndex": 23
            },
            "doc": ""
          }
        ]
      }
    },
    {
      "Struct": {
        "type_name": "__SovVirtualWallet_CallMessage_Mint",
        "template": null,
        "peekable": false,
        "fields": [
          {
            "display_name": "coins",
            "silent": false,
            "value": {
              "ByIndex": 23
            },
            "doc": ""
          },
          {
            "display_name": "mint_to_address",
            "silent": false,
            "value": {
              "ByIndex": 19
            },
            "doc": ""
          }
        ]
      }
    },
    {
      "Struct": {
        "type_name": "__SovVirtualWallet_CallMessage_Freeze",
        "template": null,
        "peekable": false,
        "fields": [
          {
            "display_name": "token_id",
            "silent": false,
            "value": {
              "ByIndex": 24
            },
            "doc": ""
          }
        ]
      }
    },
    {
      "Tuple": {
        "template": null,
        "peekable": false,
        "fields": [
          {
            "value": {
              "ByIndex": 29
            },
            "silent": false,
            "doc": ""
          }
        ]
      }
    },
    {
      "Enum": {
        "type_name": "CallMessage",
        "variants": [
          {
            "name": "Register",
            "discriminant": 0,
            "template": null,
            "value": {
              "ByIndex": 30
            }
          },
          {
            "name": "Deposit",
            "discriminant": 1,
            "template": null,
            "value": {
              "ByIndex": 32
            }
          },
          {
            "name": "InitiateWithdrawal",
            "discriminant": 2,
            "template": null,
            "value": {
              "ByIndex": 33
            }
          },
          {
            "name": "Withdraw",
            "discriminant": 3,
            "template": null,
            "value": {
              "ByIndex": 34
            }
          }
        ],
        "hide_tag": false
      }
    },
    {
      "Struct": {
        "type_name": "__SovVirtualWallet_CallMessage_Register",
        "template": null,
        "peekable": false,
        "fields": [
          {
            "display_name": "da_address",
            "silent": false,
            "value": {
              "ByIndex": 31
            },
            "doc": ""
          },
          {
            "display_name": "amount",
            "silent": false,
            "value": {
              "ByIndex": 18
            },
            "doc": ""
          }
        ]
      }
    },
    {
      "Tuple": {
        "template": null,
        "peekable": false,
        "fields": [
          {
            "value": {
              "Immediate": {
                "ByteArray": {
                  "len": 32,
                  "display": "Hex"
                }
              }
            },
            "silent": false,
            "doc": ""
          }
        ]
      }
    },
    {
      "Struct": {
        "type_name": "__SovVirtualWallet_CallMessage_Deposit",
        "template": null,
        "peekable": false,
        "fields": [
          {
            "display_name": "da_address",
            "silent": false,
            "value": {
              "ByIndex": 31
            },
            "doc": ""
          },
          {
            "display_name": "amount",
            "silent": false,
            "value": {
              "ByIndex": 18
            },
            "doc": ""
          }
        ]
      }
    },
    {
      "Struct": {
        "type_name": "__SovVirtualWallet_CallMessage_InitiateWithdrawal",
        "template": null,
        "peekable": false,
        "fields": [
          {
            "display_name": "da_address",
            "silent": false,
            "value": {
              "ByIndex": 31
            },
            "doc": ""
          }
        ]
      }
    },
    {
      "Struct": {
        "type_name": "__SovVirtualWallet_CallMessage_Withdraw",
        "template": null,
        "peekable": false,
        "fields": [
          {
            "display_name": "da_address",
            "silent": false,
            "value": {
              "ByIndex": 31
            },
            "doc": ""
          }
        ]
      }
    },
    {
      "Tuple": {
        "template": null,
        "peekable": false,
        "fields": [
          {
            "value": {
              "ByIndex": 36
            },
            "silent": false,
            "doc": ""
          }
        ]
      }
    },
    {
      "Enum": {
        "type_name": "CallMessage",
        "variants": [
          {
            "name": "RegisterAttester",
            "discriminant": 0,
            "template": null,
            "value": {
              "ByIndex": 37
            }
          },
          {
            "name": "BeginExitAttester",
            "discriminant": 1,
            "template": null,
            "value": null
          },
          {
            "name": "ExitAttester",
            "discriminant": 2,
            "template": null,
            "value": null
          },
          {
            "name": "RegisterChallenger",
            "discriminant": 3,
            "template": null,
            "value": {
              "ByIndex": 38
            }
          },
          {
            "name": "ExitChallenger",
            "discriminant": 4,
            "template": null,
            "value": null
          },
          {
            "name": "DepositAttester",
            "discriminant": 5,
            "template": null,
            "value": {
              "ByIndex": 39
            }
          }
        ],
        "hide_tag": false
      }
    },
    {
      "Tuple": {
        "template": null,
        "peekable": false,
        "fields": [
          {
            "value": {
              "ByIndex": 18
            },
            "silent": false,
            "doc": ""
          }
        ]
      }
    },
    {
      "Tuple": {
        "template": null,
        "peekable": false,
        "fields": [
          {
            "value": {
              "ByIndex": 18
            },
            "silent": false,
            "doc": ""
          }
        ]
      }
    },
    {
      "Tuple": {
        "template": null,
        "peekable": false,
        "fields": [
          {
            "value": {
              "ByIndex": 18
            },
            "silent": false,
            "doc": ""
          }
        ]
      }
    },
    {
      "Tuple": {
        "template": null,
        "peekable": false,
        "fields": [
          {
            "value": {
              "ByIndex": 41
            },
            "silent": false,
            "doc": ""
          }
        ]
      }
    },
    {
      "Enum": {
        "type_name": "CallMessage",
        "variants": [
          {
            "name": "Register",
            "discriminant": 0,
            "template": null,
            "value": {
              "ByIndex": 42
            }
          },
          {
            "name": "Deposit",
            "discriminant": 1,
            "template": null,
            "value": {
              "ByIndex": 43
            }
          },
          {
            "name": "Exit",
            "discriminant": 2,
            "template": null,
            "value": null
          }
        ],
        "hide_tag": false
      }
    },
    {
      "Tuple": {
        "template": null,
        "peekable": false,
        "fields": [
          {
            "value": {
              "ByIndex": 18
            },
            "silent": false,
            "doc": ""
          }
        ]
      }
    },
    {
      "Tuple": {
        "template": null,
        "peekable": false,
        "fields": [
          {
            "value": {
              "ByIndex": 18
            },
            "silent": false,
            "doc": ""
          }
        ]
      }
    },
    {
      "Tuple": {
        "template": null,
        "peekable": false,
        "fields": [
          {
            "value": {
              "ByIndex": 45
            },
            "silent": false,
            "doc": ""
          }
        ]
      }
    },
    {
      "Enum": {
        "type_name": "CallMessage",
        "variants": [
          {
            "name": "SetValue",
            "discriminant": 0,
            "template": null,
            "value": {
              "ByIndex": 46
            }
          }
        ],
        "hide_tag": false
      }
    },
    {
      "Tuple": {
        "template": null,
        "peekable": false,
        "fields": [
          {
            "value": {
              "Immediate": {
                "Integer": [
                  "u32",
                  "Decimal"
                ]
              }
            },
            "silent": false,
            "doc": ""
          }
        ]
      }
    },
    {
      "Tuple": {
        "template": null,
        "peekable": false,
        "fields": [
          {
            "value": {
              "ByIndex": 48
            },
            "silent": false,
            "doc": ""
          }
        ]
      }
    },
    {
      "Enum": {
        "type_name": "CallMessage",
        "variants": [
          {
            "name": "CreateUser",
            "discriminant": 0,
            "template": null,
            "value": {
              "ByIndex": 49
            }
          },
          {
            "name": "CreateSubReddit",
            "discriminant": 1,
            "template": null,
            "value": {
              "ByIndex": 50
            }
          },
          {
            "name": "CreatePost",
            "discriminant": 2,
            "template": null,
            "value": {
              "ByIndex": 51
            }
          }
        ],
        "hide_tag": false
      }
    },
    {
      "Struct": {
        "type_name": "__SovVirtualWallet_CallMessage_CreateUser",
        "template": null,
        "peekable": false,
        "fields": [
          {
            "display_name": "username",
            "silent": false,
            "value": {
              "Immediate": "String"
            },
            "doc": ""
          }
        ]
      }
    },
    {
      "Struct": {
        "type_name": "__SovVirtualWallet_CallMessage_CreateSubReddit",
        "template": null,
        "peekable": false,
        "fields": [
          {
            "display_name": "user_address",
            "silent": false,
            "value": {
              "ByIndex": 19
            },
            "doc": ""
          },
          {
            "display_name": "subname",
            "silent": false,
            "value": {
              "Immediate": "String"
            },
            "doc": ""
          },
          {
            "display_name": "description",
            "silent": false,
            "value": {
              "Immediate": "String"
            },
            "doc": ""
          }
        ]
      }
    },
    {
      "Struct": {
        "type_name": "__SovVirtualWallet_CallMessage_CreatePost",
        "template": null,
        "peekable": false,
        "fields": [
          {
            "display_name": "title",
            "silent": false,
            "value": {
              "Immediate": "String"
            },
            "doc": ""
          },
          {
            "display_name": "flair",
            "silent": false,
            "value": {
              "Immediate": "String"
            },
            "doc": ""
          },
          {
            "display_name": "content",
            "silent": false,
            "value": {
              "Immediate": "String"
            },
            "doc": ""
          },
          {
            "display_name": "subaddress",
            "silent": false,
            "value": {
              "ByIndex": 19
            },
            "doc": ""
          }
        ]
      }
    },
    {
      "Tuple": {
        "template": null,
        "peekable": false,
        "fields": [
          {
            "value": {
              "ByIndex": 13
            },
            "silent": false,
            "doc": ""
          }
        ]
      }
    },
    {
      "Tuple": {
        "template": null,
        "peekable": false,
        "fields": [
          {
            "value": {
              "ByIndex": 13
            },
            "silent": false,
            "doc": ""
          }
        ]
      }
    },
    {
      "Tuple": {
        "template": null,
        "peekable": false,
        "fields": [
          {
            "value": {
              "ByIndex": 55
            },
            "silent": false,
            "doc": ""
          }
        ]
      }
    },
    {
      "Enum": {
        "type_name": "CallMessage",
        "variants": [
          {
            "name": "RegisterPaymaster",
            "discriminant": 0,
            "template": null,
            "value": {
              "ByIndex": 56
            }
          },
          {
            "name": "SetPayerForSequencer",
            "discriminant": 1,
            "template": null,
            "value": {
              "ByIndex": 72
            }
          },
          {
            "name": "UpdatePolicy",
            "discriminant": 2,
            "template": null,
            "value": {
              "ByIndex": 73
            }
          }
        ],
        "hide_tag": false
      }
    },
    {
      "Struct": {
        "type_name": "__SovVirtualWallet_CallMessage_RegisterPaymaster",
        "template": null,
        "peekable": false,
        "fields": [
          {
            "display_name": "policy",
            "silent": false,
            "value": {
              "ByIndex": 57
            },
            "doc": ""
          }
        ]
      }
    },
    {
      "Struct": {
        "type_name": "PaymasterPolicyInitializer",
        "template": null,
        "peekable": false,
        "fields": [
          {
            "display_name": "default_payee_policy",
            "silent": false,
            "value": {
              "ByIndex": 58
            },
            "doc": ""
          },
          {
            "display_name": "payees",
            "silent": false,
            "value": {
              "ByIndex": 67
            },
            "doc": ""
          },
          {
            "display_name": "authorized_updaters",
            "silent": false,
            "value": {
              "ByIndex": 20
            },
            "doc": ""
          },
          {
            "display_name": "authorized_sequencers",
            "silent": false,
            "value": {
              "ByIndex": 69
            },
            "doc": ""
          }
        ]
      }
    },
    {
      "Enum": {
        "type_name": "PayeePolicy",
        "variants": [
          {
            "name": "Allow",
            "discriminant": 0,
            "template": null,
            "value": {
              "ByIndex": 59
            }
          },
          {
            "name": "Deny",
            "discriminant": 1,
            "template": null,
            "value": null
          }
        ],
        "hide_tag": false
      }
    },
    {
      "Struct": {
        "type_name": "__SovVirtualWallet_PayeePolicy_Allow",
        "template": null,
        "peekable": false,
        "fields": [
          {
            "display_name": "max_fee",
            "silent": false,
            "value": {
              "ByIndex": 21
            },
            "doc": ""
          },
          {
            "display_name": "gas_limit",
            "silent": false,
            "value": {
              "ByIndex": 60
            },
            "doc": ""
          },
          {
            "display_name": "max_gas_price",
            "silent": false,
            "value": {
              "ByIndex": 63
            },
            "doc": ""
          },
          {
            "display_name": "transaction_limit",
            "silent": false,
            "value": {
              "ByIndex": 66
            },
            "doc": ""
          }
        ]
      }
    },
    {
      "Option": {
        "value": {
          "ByIndex": 61
        }
      }
    },
    {
      "Tuple": {
        "template": null,
        "peekable": false,
        "fields": [
          {
            "value": {
              "ByIndex": 62
            },
            "silent": false,
            "doc": ""
          }
        ]
      }
    },
    {
      "Array": {
        "len": 2,
        "value": {
          "Immediate": {
            "Integer": [
              "u64",
              "Decimal"
            ]
          }
        }
      }
    },
    {
      "Option": {
        "value": {
          "ByIndex": 64
        }
      }
    },
    {
      "Struct": {
        "type_name": "GasPrice",
        "template": null,
        "peekable": false,
        "fields": [
          {
            "display_name": "value",
            "silent": false,
            "value": {
              "ByIndex": 65
            },
            "doc": ""
          }
        ]
      }
    },
    {
      "Array": {
        "len": 2,
        "value": {
          "ByIndex": 18
        }
      }
    },
    {
      "Option": {
        "value": {
          "Immediate": {
            "Integer": [
              "u64",
              "Decimal"
            ]
          }
        }
      }
    },
    {
      "Vec": {
        "value": {
          "ByIndex": 68
        }
      }
    },
    {
      "Tuple": {
        "template": null,
        "peekable": false,
        "fields": [
          {
            "value": {
              "ByIndex": 19
            },
            "silent": false,
            "doc": ""
          },
          {
            "value": {
              "ByIndex": 58
            },
            "silent": false,
            "doc": ""
          }
        ]
      }
    },
    {
      "Enum": {
        "type_name": "AuthorizedSequencers",
        "variants": [
          {
            "name": "All",
            "discriminant": 0,
            "template": null,
            "value": null
          },
          {
            "name": "Some",
            "discriminant": 1,
            "template": null,
            "value": {
              "ByIndex": 70
            }
          }
        ],
        "hide_tag": false
      }
    },
    {
      "Tuple": {
        "template": null,
        "peekable": false,
        "fields": [
          {
            "value": {
              "ByIndex": 71
            },
            "silent": false,
            "doc": ""
          }
        ]
      }
    },
    {
      "Vec": {
        "value": {
          "ByIndex": 31
        }
      }
    },
    {
      "Struct": {
        "type_name": "__SovVirtualWallet_CallMessage_SetPayerForSequencer",
        "template": null,
        "peekable": false,
        "fields": [
          {
            "display_name": "payer",
            "silent": false,
            "value": {
              "ByIndex": 19
            },
            "doc": ""
          }
        ]
      }
    },
    {
      "Struct": {
        "type_name": "__SovVirtualWallet_CallMessage_UpdatePolicy",
        "template": null,
        "peekable": false,
        "fields": [
          {
            "display_name": "payer",
            "silent": false,
            "value": {
              "ByIndex": 19
            },
            "doc": ""
          },
          {
            "display_name": "update",
            "silent": false,
            "value": {
              "ByIndex": 74
            },
            "doc": ""
          }
        ]
      }
    },
    {
      "Struct": {
        "type_name": "PolicyUpdate",
        "template": null,
        "peekable": false,
        "fields": [
          {
            "display_name": "sequencer_update",
            "silent": false,
            "value": {
              "ByIndex": 75
            },
            "doc": ""
          },
          {
            "display_name": "updaters_to_add",
            "silent": false,
            "value": {
              "ByIndex": 80
            },
            "doc": ""
          },
          {
            "display_name": "updaters_to_remove",
            "silent": false,
            "value": {
              "ByIndex": 80
            },
            "doc": ""
          },
          {
            "display_name": "payee_policies_to_set",
            "silent": false,
            "value": {
              "ByIndex": 81
            },
            "doc": ""
          },
          {
            "display_name": "payee_policies_to_delete",
            "silent": false,
            "value": {
              "ByIndex": 80
            },
            "doc": ""
          },
          {
            "display_name": "default_policy",
            "silent": false,
            "value": {
              "ByIndex": 82
            },
            "doc": ""
          }
        ]
      }
    },
    {
      "Option": {
        "value": {
          "ByIndex": 76
        }
      }
    },
    {
      "Enum": {
        "type_name": "SequencerSetUpdate",
        "variants": [
          {
            "name": "AllowAll",
            "discriminant": 0,
            "template": null,
            "value": null
          },
          {
            "name": "Update",
            "discriminant": 1,
            "template": null,
            "value": {
              "ByIndex": 77
            }
          }
        ],
        "hide_tag": false
      }
    },
    {
      "Tuple": {
        "template": null,
        "peekable": false,
        "fields": [
          {
            "value": {
              "ByIndex": 78
            },
            "silent": false,
            "doc": ""
          }
        ]
      }
    },
    {
      "Struct": {
        "type_name": "AllowedSequencerUpdate",
        "template": null,
        "peekable": false,
        "fields": [
          {
            "display_name": "to_add",
            "silent": false,
            "value": {
              "ByIndex": 79
            },
            "doc": ""
          },
          {
            "display_name": "to_remove",
            "silent": false,
            "value": {
              "ByIndex": 79
            },
            "doc": ""
          }
        ]
      }
    },
    {
      "Option": {
        "value": {
          "ByIndex": 71
        }
      }
    },
    {
      "Option": {
        "value": {
          "ByIndex": 20
        }
      }
    },
    {
      "Option": {
        "value": {
          "ByIndex": 67
        }
      }
    },
    {
      "Option": {
        "value": {
          "ByIndex": 58
        }
      }
    },
    {
      "Struct": {
        "type_name": "TxDetails",
        "template": null,
        "peekable": false,
        "fields": [
          {
            "display_name": "max_priority_fee_bips",
            "silent": false,
            "value": {
              "ByIndex": 84
            },
            "doc": ""
          },
          {
            "display_name": "max_fee",
            "silent": false,
            "value": {
              "ByIndex": 18
            },
            "doc": ""
          },
          {
            "display_name": "gas_limit",
            "silent": false,
            "value": {
              "ByIndex": 60
            },
            "doc": ""
          },
          {
            "display_name": "chain_id",
            "silent": false,
            "value": {
              "Immediate": {
                "Integer": [
                  "u64",
                  "Decimal"
                ]
              }
            },
            "doc": ""
          }
        ]
      }
    },
    {
      "Tuple": {
        "template": null,
        "peekable": false,
        "fields": [
          {
            "value": {
              "Immediate": {
                "Integer": [
                  "u64",
                  "Decimal"
                ]
              }
            },
            "silent": false,
            "doc": ""
          }
        ]
      }
    },
    {
      "Struct": {
        "type_name": "UnsignedTransaction",
        "template": null,
        "peekable": false,
        "fields": [
          {
            "display_name": "runtime_call",
            "silent": false,
            "value": {
              "ByIndex": 6
            },
            "doc": ""
          },
          {
            "display_name": "generation",
            "silent": false,
            "value": {
              "Immediate": {
                "Integer": [
                  "u64",
                  "Decimal"
                ]
              }
            },
            "doc": ""
          },
          {
            "display_name": "details",
            "silent": false,
            "value": {
              "ByIndex": 83
            },
            "doc": ""
          }
        ]
      }
    }
  ],
  "root_type_indices": [
    0,
    85,
    6,
    19
  ],
  "chain_data": {
    "chain_id": 4321,
    "chain_name": "TestChain"
  },
  "templates": [
    {},
    {},
    {
      "transfer": {
        "preencoded_bytes": [
          2,
          1
        ],
        "inputs": [
          [
            "to",
            {
              "type_link": {
                "ByIndex": 19
              },
              "offset": 2
            }
          ],
          [
            "amount",
            {
              "type_link": {
                "Immediate": {
                  "Integer": [
                    "u128",
                    {
                      "FixedPoint": {
                        "FromSiblingField": {
                          "field_index": 1,
                          "byte_offset": 31
                        }
                      }
                    }
                  ]
                }
              },
              "offset": 2
            }
          ],
          [
            "token_id",
            {
              "type_link": {
                "ByIndex": 24
              },
              "offset": 2
            }
          ]
        ]
      }
    },
    {}
  ],
  "serde_metadata": [
    {
      "name": "Transaction",
      "fields_or_variants": [
        {
          "name": "versioned_tx"
        }
      ]
    },
    {
      "name": "VersionedTx",
      "fields_or_variants": [
        {
          "name": "V0"
        }
      ]
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "Version0",
      "fields_or_variants": [
        {
          "name": "signature"
        },
        {
          "name": "pub_key"
        },
        {
          "name": "runtime_call"
        },
        {
          "name": "generation"
        },
        {
          "name": "details"
        }
      ]
    },
    {
      "name": "Ed25519Signature",
      "fields_or_variants": [
        {
          "name": "msg_sig"
        }
      ]
    },
    {
      "name": "Ed25519PublicKey",
      "fields_or_variants": [
        {
          "name": "pub_key"
        }
      ]
    },
    {
      "name": "RuntimeCall",
      "fields_or_variants": [
        {
          "name": "accounts"
        },
        {
          "name": "uniqueness"
        },
        {
          "name": "bank"
        },
        {
          "name": "sequencer_registry"
        },
        {
          "name": "attester_incentives"
        },
        {
          "name": "prover_incentives"
        },
        {
          "name": "example_module"
        },
        {
          "name": "reddit_module"
        },
        {
          "name": "chain_state"
        },
        {
          "name": "blob_storage"
        },
        {
          "name": "paymaster"
        }
      ]
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "CallMessage",
      "fields_or_variants": [
        {
          "name": "insert_credential_id"
        }
      ]
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "NotInstantiable",
      "fields_or_variants": []
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "CallMessage",
      "fields_or_variants": [
        {
          "name": "create_token"
        },
        {
          "name": "transfer"
        },
        {
          "name": "burn"
        },
        {
          "name": "mint"
        },
        {
          "name": "freeze"
        }
      ]
    },
    {
      "name": "__SovVirtualWallet_CallMessage_CreateToken",
      "fields_or_variants": [
        {
          "name": "token_name"
        },
        {
          "name": "token_decimals"
        },
        {
          "name": "initial_balance"
        },
        {
          "name": "mint_to_address"
        },
        {
          "name": "admins"
        },
        {
          "name": "supply_cap"
        }
      ]
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "__SovVirtualWallet_CallMessage_Transfer",
      "fields_or_variants": [
        {
          "name": "to"
        },
        {
          "name": "coins"
        }
      ]
    },
    {
      "name": "Coins",
      "fields_or_variants": [
        {
          "name": "amount"
        },
        {
          "name": "token_id"
        }
      ]
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "__SovVirtualWallet_CallMessage_Burn",
      "fields_or_variants": [
        {
          "name": "coins"
        }
      ]
    },
    {
      "name": "__SovVirtualWallet_CallMessage_Mint",
      "fields_or_variants": [
        {
          "name": "coins"
        },
        {
          "name": "mint_to_address"
        }
      ]
    },
    {
      "name": "__SovVirtualWallet_CallMessage_Freeze",
      "fields_or_variants": [
        {
          "name": "token_id"
        }
      ]
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "CallMessage",
      "fields_or_variants": [
        {
          "name": "register"
        },
        {
          "name": "deposit"
        },
        {
          "name": "initiate_withdrawal"
        },
        {
          "name": "withdraw"
        }
      ]
    },
    {
      "name": "__SovVirtualWallet_CallMessage_Register",
      "fields_or_variants": [
        {
          "name": "da_address"
        },
        {
          "name": "amount"
        }
      ]
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "__SovVirtualWallet_CallMessage_Deposit",
      "fields_or_variants": [
        {
          "name": "da_address"
        },
        {
          "name": "amount"
        }
      ]
    },
    {
      "name": "__SovVirtualWallet_CallMessage_InitiateWithdrawal",
      "fields_or_variants": [
        {
          "name": "da_address"
        }
      ]
    },
    {
      "name": "__SovVirtualWallet_CallMessage_Withdraw",
      "fields_or_variants": [
        {
          "name": "da_address"
        }
      ]
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "CallMessage",
      "fields_or_variants": [
        {
          "name": "register_attester"
        },
        {
          "name": "begin_exit_attester"
        },
        {
          "name": "exit_attester"
        },
        {
          "name": "register_challenger"
        },
        {
          "name": "exit_challenger"
        },
        {
          "name": "deposit_attester"
        }
      ]
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "CallMessage",
      "fields_or_variants": [
        {
          "name": "register"
        },
        {
          "name": "deposit"
        },
        {
          "name": "exit"
        }
      ]
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "CallMessage",
      "fields_or_variants": [
        {
          "name": "set_value"
        }
      ]
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "CallMessage",
      "fields_or_variants": [
        {
          "name": "create_user"
        },
        {
          "name": "create_sub_reddit"
        },
        {
          "name": "create_post"
        }
      ]
    },
    {
      "name": "__SovVirtualWallet_CallMessage_CreateUser",
      "fields_or_variants": [
        {
          "name": "username"
        }
      ]
    },
    {
      "name": "__SovVirtualWallet_CallMessage_CreateSubReddit",
      "fields_or_variants": [
        {
          "name": "user_address"
        },
        {
          "name": "subname"
        },
        {
          "name": "description"
        }
      ]
    },
    {
      "name": "__SovVirtualWallet_CallMessage_CreatePost",
      "fields_or_variants": [
        {
          "name": "title"
        },
        {
          "name": "flair"
        },
        {
          "name": "content"
        },
        {
          "name": "subaddress"
        }
      ]
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "CallMessage",
      "fields_or_variants": [
        {
          "name": "register_paymaster"
        },
        {
          "name": "set_payer_for_sequencer"
        },
        {
          "name": "update_policy"
        }
      ]
    },
    {
      "name": "__SovVirtualWallet_CallMessage_RegisterPaymaster",
      "fields_or_variants": [
        {
          "name": "policy"
        }
      ]
    },
    {
      "name": "PaymasterPolicyInitializer",
      "fields_or_variants": [
        {
          "name": "default_payee_policy"
        },
        {
          "name": "payees"
        },
        {
          "name": "authorized_updaters"
        },
        {
          "name": "authorized_sequencers"
        }
      ]
    },
    {
      "name": "PayeePolicy",
      "fields_or_variants": [
        {
          "name": "allow"
        },
        {
          "name": "deny"
        }
      ]
    },
    {
      "name": "__SovVirtualWallet_PayeePolicy_Allow",
      "fields_or_variants": [
        {
          "name": "max_fee"
        },
        {
          "name": "gas_limit"
        },
        {
          "name": "max_gas_price"
        },
        {
          "name": "transaction_limit"
        }
      ]
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "GasPrice",
      "fields_or_variants": [
        {
          "name": "value"
        }
      ]
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "AuthorizedSequencers",
      "fields_or_variants": [
        {
          "name": "all"
        },
        {
          "name": "some"
        }
      ]
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "__SovVirtualWallet_CallMessage_SetPayerForSequencer",
      "fields_or_variants": [
        {
          "name": "payer"
        }
      ]
    },
    {
      "name": "__SovVirtualWallet_CallMessage_UpdatePolicy",
      "fields_or_variants": [
        {
          "name": "payer"
        },
        {
          "name": "update"
        }
      ]
    },
    {
      "name": "PolicyUpdate",
      "fields_or_variants": [
        {
          "name": "sequencer_update"
        },
        {
          "name": "updaters_to_add"
        },
        {
          "name": "updaters_to_remove"
        },
        {
          "name": "payee_policies_to_set"
        },
        {
          "name": "payee_policies_to_delete"
        },
        {
          "name": "default_policy"
        }
      ]
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "SequencerSetUpdate",
      "fields_or_variants": [
        {
          "name": "allow_all"
        },
        {
          "name": "update"
        }
      ]
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "AllowedSequencerUpdate",
      "fields_or_variants": [
        {
          "name": "to_add"
        },
        {
          "name": "to_remove"
        }
      ]
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "TxDetails",
      "fields_or_variants": [
        {
          "name": "max_priority_fee_bips"
        },
        {
          "name": "max_fee"
        },
        {
          "name": "gas_limit"
        },
        {
          "name": "chain_id"
        }
      ]
    },
    {
      "name": "",
      "fields_or_variants": []
    },
    {
      "name": "UnsignedTransaction",
      "fields_or_variants": [
        {
          "name": "runtime_call"
        },
        {
          "name": "generation"
        },
        {
          "name": "details"
        }
      ]
    }
  ]
}"#;
