/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/marketplace.json`.
 */
export type Marketplace = {
  "address": "HYLnmvP84H2xnoVq1RSduzvdBVSibs3ZyxNfK6ak8VwL",
  "metadata": {
    "name": "marketplace",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "buyService",
      "discriminator": [
        175,
        174,
        174,
        180,
        246,
        138,
        81,
        21
      ],
      "accounts": [
        {
          "name": "buyer",
          "writable": true,
          "signer": true
        },
        {
          "name": "provider",
          "writable": true
        },
        {
          "name": "serviceTicketMint",
          "writable": true,
          "signer": true
        },
        {
          "name": "serviceMint",
          "writable": true
        },
        {
          "name": "providerServiceAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "serviceMint"
              }
            ]
          }
        },
        {
          "name": "buyerServiceAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "serviceTicketMint"
              }
            ]
          }
        },
        {
          "name": "serviceTicketTokenAccount",
          "writable": true
        },
        {
          "name": "extraAccountMetasList",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  120,
                  116,
                  114,
                  97,
                  45,
                  97,
                  99,
                  99,
                  111,
                  117,
                  110,
                  116,
                  45,
                  109,
                  101,
                  116,
                  97,
                  115
                ]
              },
              {
                "kind": "account",
                "path": "serviceTicketMint"
              }
            ],
            "program": {
              "kind": "account",
              "path": "transferHookProgram"
            }
          }
        },
        {
          "name": "transferHookProgram"
        },
        {
          "name": "transferHookProgramAccount"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        }
      ],
      "args": []
    },
    {
      "name": "listService",
      "discriminator": [
        1,
        210,
        220,
        97,
        94,
        11,
        206,
        34
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "provider",
          "writable": true
        },
        {
          "name": "serviceMint",
          "writable": true,
          "signer": true
        },
        {
          "name": "serviceTokenAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "payer"
              },
              {
                "kind": "account",
                "path": "tokenProgram"
              },
              {
                "kind": "account",
                "path": "serviceMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "serviceAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "serviceMint"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        }
      ],
      "args": [
        {
          "name": "serviceAgreementConfig",
          "type": {
            "defined": {
              "name": "serviceAgreementConfig"
            }
          }
        }
      ]
    },
    {
      "name": "relistService",
      "discriminator": [
        239,
        173,
        140,
        205,
        16,
        192,
        36,
        203
      ],
      "accounts": [
        {
          "name": "serviceTicketToken",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "holder"
              },
              {
                "kind": "account",
                "path": "tokenProgram"
              },
              {
                "kind": "account",
                "path": "serviceTicketMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "serviceTicketMint"
        },
        {
          "name": "holder",
          "signer": true,
          "relations": [
            "serviceAccount"
          ]
        },
        {
          "name": "serviceAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "serviceTicketMint"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        }
      ],
      "args": []
    },
    {
      "name": "resellService",
      "discriminator": [
        184,
        6,
        198,
        101,
        252,
        183,
        164,
        55
      ],
      "accounts": [
        {
          "name": "resellerServiceTicketToken",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "reseller"
              },
              {
                "kind": "account",
                "path": "tokenProgram"
              },
              {
                "kind": "account",
                "path": "serviceTicketMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "buyerServiceTicketToken",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "buyer"
              },
              {
                "kind": "account",
                "path": "tokenProgram"
              },
              {
                "kind": "account",
                "path": "serviceTicketMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "serviceTicketMint"
        },
        {
          "name": "serviceAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "serviceTicketMint"
              }
            ]
          }
        },
        {
          "name": "provider",
          "writable": true
        },
        {
          "name": "reseller",
          "writable": true
        },
        {
          "name": "buyer",
          "writable": true,
          "signer": true
        },
        {
          "name": "mintRoyaltyConfig",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "serviceTicketMint"
              }
            ],
            "program": {
              "kind": "account",
              "path": "transferHookProgram"
            }
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "transferHookProgram",
          "address": "Bi2dB1dvse6p9nEDSseRC2qgnWXWFHadFSTxTjc4f5EF"
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        }
      ],
      "args": []
    },
    {
      "name": "royaltiesInit",
      "discriminator": [
        181,
        240,
        42,
        153,
        52,
        73,
        183,
        185
      ],
      "accounts": [
        {
          "name": "holder",
          "writable": true,
          "signer": true,
          "relations": [
            "serviceAccount"
          ]
        },
        {
          "name": "serviceTicketToken",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "holder"
              },
              {
                "kind": "account",
                "path": "tokenProgram"
              },
              {
                "kind": "account",
                "path": "serviceTicketMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "serviceTicketMint"
        },
        {
          "name": "serviceAccount",
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "serviceTicketMint"
              }
            ]
          }
        },
        {
          "name": "provider"
        },
        {
          "name": "mintRoyaltyConfig",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "serviceTicketMint"
              }
            ],
            "program": {
              "kind": "account",
              "path": "transferHookProgram"
            }
          }
        },
        {
          "name": "transferHookProgram",
          "address": "Bi2dB1dvse6p9nEDSseRC2qgnWXWFHadFSTxTjc4f5EF"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        }
      ],
      "args": []
    },
    {
      "name": "unlistService",
      "discriminator": [
        178,
        236,
        14,
        7,
        219,
        44,
        167,
        27
      ],
      "accounts": [
        {
          "name": "serviceTicketToken",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "holder"
              },
              {
                "kind": "account",
                "path": "tokenProgram"
              },
              {
                "kind": "account",
                "path": "serviceTicketMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "serviceTicketMint"
        },
        {
          "name": "provider"
        },
        {
          "name": "holder",
          "signer": true,
          "relations": [
            "serviceAccount"
          ]
        },
        {
          "name": "serviceAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "serviceTicketMint"
              }
            ]
          }
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "mintRoyaltyConfig",
      "discriminator": [
        130,
        208,
        45,
        78,
        108,
        125,
        243,
        222
      ]
    },
    {
      "name": "serviceAccount",
      "discriminator": [
        72,
        33,
        73,
        146,
        208,
        186,
        107,
        192
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "serviceAccountAlreadyInitialized",
      "msg": "Service Account already initialized"
    },
    {
      "code": 6001,
      "name": "metadataKeyMismatch",
      "msg": "Metadata Key does not match"
    },
    {
      "code": 6002,
      "name": "metadataValueParseError",
      "msg": "Could not parse Metadata value"
    },
    {
      "code": 6003,
      "name": "serviceAccountMismatch",
      "msg": "Service Account keys did not match the provided account key"
    },
    {
      "code": 6004,
      "name": "isNotCurrentlyTransferring",
      "msg": "The token is not currently transferring"
    },
    {
      "code": 6005,
      "name": "isNotCurrentlyReselling",
      "msg": "The token is not currently on being sold"
    },
    {
      "code": 6006,
      "name": "isNotListed",
      "msg": "The ticket is not listed"
    },
    {
      "code": 6007,
      "name": "overflowOccurred",
      "msg": "Overflow occurred"
    },
    {
      "code": 6008,
      "name": "mintRoyaltyParseError",
      "msg": "Failed to parse mint royalty config"
    }
  ],
  "types": [
    {
      "name": "mintRoyaltyConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mint",
            "type": "pubkey"
          },
          {
            "name": "isSelling",
            "type": "bool"
          },
          {
            "name": "isEnabled",
            "type": "bool"
          },
          {
            "name": "isInitialized",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "serviceAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "holder",
            "type": "pubkey"
          },
          {
            "name": "mint",
            "type": "pubkey"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "isListed",
            "type": "bool"
          },
          {
            "name": "isInitialized",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "serviceAgreementConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "symbol",
            "type": "string"
          },
          {
            "name": "uri",
            "type": "string"
          },
          {
            "name": "description",
            "type": "string"
          },
          {
            "name": "price",
            "type": "u64"
          },
          {
            "name": "feeBasisPoints",
            "type": "u16"
          },
          {
            "name": "maximumFee",
            "type": "u64"
          },
          {
            "name": "transferable",
            "type": "bool"
          }
        ]
      }
    }
  ]
};
