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
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "receiver"
        },
        {
          "name": "provider",
          "writable": true,
          "relations": [
            "serviceAccount"
          ]
        },
        {
          "name": "serviceTicketMint",
          "writable": true,
          "signer": true
        },
        {
          "name": "serviceMint",
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
                "kind": "const",
                "value": [
                  115,
                  101,
                  114,
                  118,
                  105,
                  99,
                  101,
                  45,
                  97,
                  99,
                  99,
                  111,
                  117,
                  110,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "serviceMint"
              }
            ]
          }
        },
        {
          "name": "serviceTicketTokenAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "receiver"
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
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
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
      "name": "joinService",
      "discriminator": [
        23,
        153,
        219,
        144,
        212,
        239,
        189,
        210
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
          "name": "serviceAccount",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  101,
                  114,
                  118,
                  105,
                  99,
                  101,
                  45,
                  97,
                  99,
                  99,
                  111,
                  117,
                  110,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "serviceMint"
              }
            ]
          }
        },
        {
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
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
                "kind": "const",
                "value": [
                  115,
                  101,
                  114,
                  118,
                  105,
                  99,
                  101,
                  45,
                  97,
                  99,
                  99,
                  111,
                  117,
                  110,
                  116
                ]
              },
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
    }
  ],
  "accounts": [
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
    }
  ],
  "types": [
    {
      "name": "serviceAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "provider",
            "type": "pubkey"
          },
          {
            "name": "serviceMint",
            "type": "pubkey"
          },
          {
            "name": "bump",
            "type": "u8"
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
