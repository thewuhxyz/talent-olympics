import * as anchor from "@coral-xyz/anchor"
import { AnchorProvider, Program } from "@coral-xyz/anchor"
import { Marketplace } from "../src"
import { sleep, explorer } from "./helpers"
import { assert } from "chai"
import {
	ASSOCIATED_TOKEN_PROGRAM_ID,
	TOKEN_2022_PROGRAM_ID,
	getAssociatedTokenAddressSync,
} from "@solana/spl-token"
import {
	PublicKey,
	Keypair,
	LAMPORTS_PER_SOL,
	SystemProgram,
	SYSVAR_RENT_PUBKEY,
	ComputeBudgetProgram,
} from "@solana/web3.js"

describe("demo_program", () => {
	const provider = AnchorProvider.env()
	const connection = provider.connection

	const program = anchor.workspace.Marketplace as Program<Marketplace>

	const serviceProvider = Keypair.generate()
	const serviceReceiver = Keypair.generate()
	const service_mint = Keypair.generate()
	const service_ticket_mint = Keypair.generate()

	console.log("👉 program id:", program.programId.toBase58())

	console.log("👉 service provider:", serviceProvider.publicKey.toBase58())
	console.log("👉 service receiver:", serviceReceiver.publicKey.toBase58())

	const SERVICE_ACCOUNT_SEEDS = "service-account"

	const service_token = (owner: PublicKey) =>
		getAssociatedTokenAddressSync(service_mint.publicKey, owner, true, TOKEN_2022_PROGRAM_ID)

	const service_ticket_token = (owner: PublicKey) =>
		getAssociatedTokenAddressSync(
			service_ticket_mint.publicKey,
			owner,
			true,
			TOKEN_2022_PROGRAM_ID
		)

	const servicePda = (mint: PublicKey) =>
		PublicKey.findProgramAddressSync(
			[Buffer.from(SERVICE_ACCOUNT_SEEDS), mint.toBuffer()],
			program.programId
		)[0]

	before(async () => {
		console.log("---- airdroping token ----")

		let tx = await connection.requestAirdrop(
			serviceProvider.publicKey,
			1 * anchor.web3.LAMPORTS_PER_SOL
		)
		console.log("✅ Transaction successful", tx)

		tx = await connection.requestAirdrop(
			serviceReceiver.publicKey,
			1 * anchor.web3.LAMPORTS_PER_SOL
		)
		console.log("✅ Transaction successful", tx)

		await sleep(3)
	})

	it("creates a service", async () => {
		console.log("---- creating a service ----")

		const token = service_token(serviceProvider.publicKey)

		console.log("token account:", token)

		let tx = await program.methods
			.listService({
				name: "my service",
				description: "description of my service",
				feeBasisPoints: 1000,
				maximumFee: new anchor.BN(LAMPORTS_PER_SOL * 1000),
				price: new anchor.BN(LAMPORTS_PER_SOL * 0.1),
				symbol: "SERV",
				transferable: true,
				uri: "some_url.json",
			})
			.accountsStrict({
				associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
				payer: serviceProvider.publicKey,
				provider: serviceProvider.publicKey,
				// rent: SYSVAR_RENT_PUBKEY,
				serviceAccount: servicePda(service_mint.publicKey),
				serviceMint: service_mint.publicKey,
				serviceTokenAccount: service_token(serviceProvider.publicKey),
				tokenProgram: TOKEN_2022_PROGRAM_ID,
				systemProgram: SystemProgram.programId,
			})
			.signers([serviceProvider, service_mint])
			.preInstructions([
				ComputeBudgetProgram.setComputeUnitLimit({
					units: 400000, // specify the number of compute units you want
				}),
			])
			.rpc({ skipPreflight: true })

		console.log(
			"✅ Transaction successful",
			explorer(tx, "tx", "custom")
		)

		console.log(
			"👉 Mint Account",
			explorer(service_mint.publicKey.toBase58(), "address", "custom")
		)

		console.log(
			"👉 Token Account",
			explorer(token.toBase58(), "address", "custom")
		)

		await sleep(3)

		const service = await program.account.serviceAccount.fetch(
			servicePda(service_mint.publicKey)
		)

		console.log("👉 service:", JSON.parse(JSON.stringify(service)))

		assert(
			service.provider.toBase58() === serviceProvider.publicKey.toBase58(),
			"counter address does not match"
		)

		assert(
			service.serviceMint.toBase58() === service.serviceMint.toBase58(),
			"authority does not match"
		)
	})
})
