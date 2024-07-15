/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/marketplace_transfer_controller.json`.
 */
export type MarketplaceTransferController = {
  "address": "Bi2dB1dvse6p9nEDSseRC2qgnWXWFHadFSTxTjc4f5EF",
  "metadata": {
    "name": "marketplaceTransferController",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "royalties",
      "discriminator": [
        17,
        30,
        97,
        194,
        26,
        175,
        42,
        18
      ],
      "accounts": [
        {
          "name": "sourceTokenAccount"
        },
        {
          "name": "serviceTicketMint"
        },
        {
          "name": "receiverTokenAccount"
        },
        {
          "name": "reseller"
        },
        {
          "name": "extraAccountMetasList"
        },
        {
          "name": "mintRoyaltyConfig"
        },
        {
          "name": "providerWsolTokenAccount",
          "writable": true
        },
        {
          "name": "resellerWsolTokenAccount",
          "writable": true
        },
        {
          "name": "mintRoyaltyWsolTokenAccount",
          "writable": true
        },
        {
          "name": "wsolMint"
        },
        {
          "name": "tokenProgram"
        },
        {
          "name": "associatedTokenProgram"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "royaltyInit",
      "discriminator": [
        250,
        237,
        160,
        206,
        202,
        136,
        66,
        212
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "serviceTicketMint"
        },
        {
          "name": "wsolMint"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "provider"
        },
        {
          "name": "serviceAccount"
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
            ]
          }
        },
        {
          "name": "mintRoyaltyWsolTokenAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "mintRoyaltyConfig"
              },
              {
                "kind": "account",
                "path": "tokenProgramClassic"
              },
              {
                "kind": "account",
                "path": "wsolMint"
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
          "name": "tokenProgramClassic"
        },
        {
          "name": "associatedTokenProgram"
        }
      ],
      "args": []
    },
    {
      "name": "royaltyInitExtraMetas",
      "discriminator": [
        26,
        219,
        183,
        117,
        181,
        210,
        216,
        241
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "serviceTicketMint",
          "writable": true,
          "signer": true
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
            ]
          }
        },
        {
          "name": "wsolMint"
        },
        {
          "name": "transferHookProgramId",
          "address": "Bi2dB1dvse6p9nEDSseRC2qgnWXWFHadFSTxTjc4f5EF"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "serviceAccount"
        },
        {
          "name": "providerWsolTokenAccount"
        },
        {
          "name": "tokenProgramClassic"
        },
        {
          "name": "associatedTokenProgram"
        }
      ],
      "args": []
    },
    {
      "name": "royaltyUpdate",
      "discriminator": [
        63,
        75,
        139,
        30,
        239,
        179,
        5,
        250
      ],
      "accounts": [
        {
          "name": "serviceTicketMint"
        },
        {
          "name": "serviceAccount",
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
            ]
          }
        }
      ],
      "args": [
        {
          "name": "isSelling",
          "type": "bool"
        }
      ]
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
      "name": "overflowOccurred",
      "msg": "Overflow occurred"
    }
  ],
  "types": [
    {
      "name": "mintRoyaltyConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "provider",
            "type": "pubkey"
          },
          {
            "name": "mint",
            "type": "pubkey"
          },
          {
            "name": "isSelling",
            "type": "bool"
          },
          {
            "name": "isInitialized",
            "type": "bool"
          }
        ]
      }
    }
  ]
};
