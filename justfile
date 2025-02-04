anchor *args:
    anchor {{args}}

build program:
    just remove {{program}}
    anchor build -p {{program}}
    just copy-idl

build-all:
    anchor build
    just copy-idl

deploy program:
    just remove-idl {{program}}
    anchor build -p {{program}}
    anchor deploy -p {{program}}
    just copy-idl

deploy-all:
    just remove-idl-all
    anchor build
    anchor deploy
    just copy-idl

test program:
    anchor run {{program}}

test-all:
    anchor test --skip-build --skip-deploy --skip-local-validator

recover program:
    solana-keygen recover -o ./target/deploy/intermediate-deploy-{{program}}.json -f

redeploy program:
    solana program deploy --buffer ./target/deploy/intermediate-deploy-{{program}}.json --program-id ./target/deploy/{{program}}-keypair.json  ./target/deploy/{{program}}.so -v

extend program amount:
    solana program extend ./target/deploy/{{program}}-keypair.json {{amount}}

close program:
    solana program close -k ~/.config/solana/deployer.json ./target/deploy/intermediate-deploy-{{program}}.json

remove-idl-all:
    rm -f ./target/idl/*
    rm -f ./target/types/*
    find ./protocol/src/idl -mindepth 1 ! -name 'index.ts' -exec rm -rf {} +

remove-idl program:
    rm -f ./target/idl/{{program}}.json
    rm -f ./target/types/{{program}}.ts
    rm -f ./protocol/src/idl/{{program}}.ts
    rm -f ./protocol/src/idl/{{program}}.json
	
copy-idl:
    cp -v ./target/idl/* ./protocol/src/idl
    cp -v ./target/types/* ./protocol/src/idl

update-deps:
    cargo update -p solana-zk-token-sdk@2.0.1 --precise 1.18.17

run-validator:
    solana-test-validator --reset --bpf-program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb ./spl/spl_token_2022.so