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
          "writable": true,
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
          "name": "buyerServiceAccount",
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
              "path": "transferHookProgramId"
            }
          }
        },
        {
          "name": "programId"
        },
        {
          "name": "transferHookProgramId"
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
        },
        {
          "name": "providerWsolTokenAccount"
        },
        {
          "name": "wsolMint"
        },
        {
          "name": "tokenProgramClassic"
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
          "name": "serviceTicketMint"
        },
        {
          "name": "serviceMint"
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "reseller",
          "signer": true
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
          "name": "payerServiceTicketToken",
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
          "name": "serviceTicketMint",
          "writable": true
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
                "path": "serviceTicketMint"
              }
            ]
          }
        },
        {
          "name": "reseller",
          "signer": true
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "extraAccountMetasList",
          "writable": true
        },
        {
          "name": "wsolMint"
        },
        {
          "name": "mintRoyaltyWsolTokenAccount",
          "writable": true
        },
        {
          "name": "resellerWsolTokenAccount"
        },
        {
          "name": "providerWsolTokenAccount"
        },
        {
          "name": "mintRoyaltyConfig",
          "writable": true
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
          "name": "transferHookProgramId"
        },
        {
          "name": "tokenProgramClassic"
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        }
      ],
      "args": []
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
          "writable": true
        },
        {
          "name": "mintRoyaltyWsolTokenAccount",
          "writable": true
        },
        {
          "name": "tokenProgramClassic"
        },
        {
          "name": "associatedTokenProgram"
        },
        {
          "name": "transferHookProgram",
          "docs": [
            "CHECK"
          ]
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
          "name": "serviceTicketMint"
        },
        {
          "name": "serviceMint"
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "reseller",
          "signer": true
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
          },
          {
            "name": "isSale",
            "type": "bool"
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
