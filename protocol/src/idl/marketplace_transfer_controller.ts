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
      "name": "royaltyConfigInit",
      "discriminator": [
        156,
        63,
        219,
        246,
        143,
        86,
        23,
        237
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
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "royaltyConfigUpdate",
      "discriminator": [
        133,
        58,
        181,
        201,
        39,
        121,
        242,
        207
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
    },
    {
      "name": "transferControl",
      "discriminator": [
        203,
        120,
        44,
        119,
        23,
        27,
        11,
        129
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
          "name": "extraAccountMetasList",
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
          "name": "mintRoyaltyConfig"
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
      "name": "transferControlInit",
      "discriminator": [
        244,
        236,
        98,
        172,
        121,
        209,
        177,
        95
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
          "name": "transferHookProgramId",
          "address": "Bi2dB1dvse6p9nEDSseRC2qgnWXWFHadFSTxTjc4f5EF"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
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
      "name": "transferOutsideMarketplaceNotAllowed",
      "msg": "Transfer not allowed outside of marketplace"
    },
    {
      "code": 6007,
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
    }
  ]
};
