import {
	getAssociatedTokenAddressSync,
	NATIVE_MINT,
	TOKEN_2022_PROGRAM_ID,
	TOKEN_PROGRAM_ID,
} from "@solana/spl-token"
import { Keypair, PublicKey } from "@solana/web3.js"

export const SERVICE_ACCOUNT_SEEDS = "service-account"

export class User {
	keypair: Keypair
	serviceMint: PublicKey
	programId: PublicKey
	transferHoodId: PublicKey

	constructor(config: {
		keypair: Keypair
		serviceMint: PublicKey
		programId: PublicKey
		transferHoodId: PublicKey
	}) {}

	static new(config: {
		serviceMint: PublicKey
		programId: PublicKey
		transferHoodId: PublicKey
	}) {
		const keypair = Keypair.generate()
		return new User({ ...config, keypair })
	}

  get publicKey() {
    return this.keypair.publicKey
  }

  get wsolAddress() {
    return MarketplaceHelpers.wsol(this.publicKey)
  }

  get serviceTokenAccount() {
    return MarketplaceHelpers.service_token(this.publicKey, this.serviceMint)
  }

  serviceTicketAccount(serviceTicketMint: PublicKey) {
    return MarketplaceHelpers.serviceTicketToken(this.publicKey, serviceTicketMint)
  }

  
}

export class MarketplaceHelpers {
	static wsol = (owner: PublicKey) =>
		getAssociatedTokenAddressSync(NATIVE_MINT, owner, true, TOKEN_PROGRAM_ID)

	static service_token = (owner: PublicKey, serviceMint: PublicKey) =>
		getAssociatedTokenAddressSync(
			serviceMint,
			owner,
			true,
			TOKEN_2022_PROGRAM_ID
		)

	static serviceTicketToken = (owner: PublicKey, serviceTicketMint: PublicKey) =>
		getAssociatedTokenAddressSync(
			serviceTicketMint,
			owner,
			true,
			TOKEN_2022_PROGRAM_ID
		)

	static servicePda = (mint: PublicKey, programId: PublicKey) =>
		PublicKey.findProgramAddressSync(
			[Buffer.from(SERVICE_ACCOUNT_SEEDS), mint.toBuffer()],
			programId
		)[0]

	static extraMetasList(mint: PublicKey, transferControlPublicKey: PublicKey) {
		return PublicKey.findProgramAddressSync(
			[Buffer.from("extra-account-metas"), mint.toBuffer()],
			transferControlPublicKey
		)
	}

	static mintRoyaltyConfig(mint: PublicKey, transferHookPublickey: PublicKey) {
		return PublicKey.findProgramAddressSync(
			[mint.toBuffer()],
			transferHookPublickey
		)
	}
}
