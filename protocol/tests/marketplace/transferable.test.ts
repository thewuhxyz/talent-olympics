import * as anchor from "@coral-xyz/anchor"
import { AnchorProvider, Program } from "@coral-xyz/anchor"
import { Marketplace, MarketplaceTransferController } from "../../src"
import { sleep, explorer } from "../helpers"
import { assert } from "chai"
import {
	ASSOCIATED_TOKEN_PROGRAM_ID,
	NATIVE_MINT,
	TOKEN_2022_PROGRAM_ID,
	TOKEN_PROGRAM_ID,
	getAssociatedTokenAddressSync,
	transferCheckedWithTransferHook,
} from "@solana/spl-token"
import {
	PublicKey,
	Keypair,
	LAMPORTS_PER_SOL,
	SystemProgram,
	ComputeBudgetProgram,
} from "@solana/web3.js"

describe("Marketplace: Transferable", () => {
	const provider = AnchorProvider.env()
	const connection = provider.connection

	const program = anchor.workspace.Marketplace as Program<Marketplace>
	const transfer_controller = anchor.workspace
		.MarketplaceTransferController as Program<MarketplaceTransferController>

	const transfer_hook_program_id = transfer_controller.programId

	const serviceProvider = Keypair.generate()
	const serviceReseller = Keypair.generate()
	const serviceReceiver = Keypair.generate()

	const service_mint = Keypair.generate()
	const service_ticket_mint = Keypair.generate()

	console.log("👉 program id:", program.programId.toBase58())

	console.log("👉 service provider:", serviceProvider.publicKey.toBase58())

	console.log("👉 service reseller:", serviceReseller.publicKey.toBase58())

	console.log("👉 service receiver:", serviceReceiver.publicKey.toBase58())

	const wsol = (owner: PublicKey) =>
		getAssociatedTokenAddressSync(NATIVE_MINT, owner, true, TOKEN_PROGRAM_ID)

	const service_token = (owner: PublicKey) =>
		getAssociatedTokenAddressSync(
			service_mint.publicKey,
			owner,
			true,
			TOKEN_2022_PROGRAM_ID
		)

	const service_ticket_token = (owner: PublicKey) =>
		getAssociatedTokenAddressSync(
			service_ticket_mint.publicKey,
			owner,
			true,
			TOKEN_2022_PROGRAM_ID
		)

	const servicePda = (mint: Keypair) =>
		PublicKey.findProgramAddressSync(
			[mint.publicKey.toBuffer()],
			program.programId
		)[0]

	const [extraAccountMetaListPDA] = PublicKey.findProgramAddressSync(
		[
			Buffer.from("extra-account-metas"),
			service_ticket_mint.publicKey.toBuffer(),
		],
		transfer_hook_program_id
	)

	const mintRoyaltyConfig = (mint: Keypair) =>
		PublicKey.findProgramAddressSync(
			[mint.publicKey.toBuffer()],
			transfer_hook_program_id
		)[0]

	const [delegateSigner] = PublicKey.findProgramAddressSync(
		[Buffer.from("signer"), service_ticket_mint.publicKey.toBuffer()],
		transfer_hook_program_id
	)

	console.log("👉 delegate key:", delegateSigner.toBase58())

	before(async () => {
		console.log("---- airdroping token ----")

		let tx = await connection.requestAirdrop(
			serviceProvider.publicKey,
			1 * anchor.web3.LAMPORTS_PER_SOL
		)
		console.log("✅ Transaction successful", tx)

		tx = await connection.requestAirdrop(
			serviceReseller.publicKey,
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

		let tx = await program.methods
			.listService({
				name: "The Wuh",
				description: "At your service",
				feeBasisPoints: 1000,
				maximumFee: new anchor.BN(LAMPORTS_PER_SOL * 1000),
				price: new anchor.BN(LAMPORTS_PER_SOL * 0.01),
				symbol: "SERV",
				transferable: true,
				uri: "https://dev.thewuh.xyz/avatar.jpg",
			})
			.accountsStrict({
				associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
				payer: serviceProvider.publicKey,
				provider: serviceProvider.publicKey,
				serviceAccount: servicePda(service_mint),
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
			.rpc({ skipPreflight: false })

		console.log("✅ Transaction successful", explorer(tx, "tx", "custom"))

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
			servicePda(service_mint)
		)

		console.log("👉 service:", JSON.parse(JSON.stringify(service)))

		assert(
			service.holder.toBase58() === serviceProvider.publicKey.toBase58(),
			"counter address does not match"
		)

		assert(
			service.mint.toBase58() === service_mint.publicKey.toBase58(),
			"authority does not match"
		)
	})

	it("creates a service ticket", async () => {
		console.log("---- creating a service ticket ----")

		const ticket = service_ticket_token(serviceReseller.publicKey)

		let tx = await program.methods
			.buyService()
			.accountsStrict({
				associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
				buyer: serviceReseller.publicKey,
				provider: serviceProvider.publicKey,
				serviceTicketTokenAccount: ticket,
				serviceTicketMint: service_ticket_mint.publicKey,
				providerServiceAccount: servicePda(service_mint),
				serviceMint: service_mint.publicKey,
				tokenProgram: TOKEN_2022_PROGRAM_ID,
				systemProgram: SystemProgram.programId,
				buyerServiceAccount: servicePda(service_ticket_mint),
				transferHookProgram: transfer_hook_program_id,
				transferHookProgramAccount: transfer_hook_program_id,
				extraAccountMetasList: extraAccountMetaListPDA,
			})
			.signers([serviceReseller, service_ticket_mint])
			.preInstructions([
				ComputeBudgetProgram.setComputeUnitLimit({
					units: 400000,
				}),
			])
			.rpc({ skipPreflight: true })

		console.log("✅ Transaction successful", explorer(tx, "tx", "custom"))

		console.log(
			"👉 ticket mint Account",
			explorer(service_ticket_mint.publicKey.toBase58(), "address", "custom")
		)

		console.log(
			"👉 ticket token Account",
			explorer(ticket.toBase58(), "address", "custom")
		)

		await sleep(3)

		const service = await program.account.serviceAccount.fetch(
			servicePda(service_ticket_mint)
		)

		console.log("👉 service:", JSON.parse(JSON.stringify(service)))

		// assert(
		// 	service.holder.toBase58() === serviceProvider.publicKey.toBase58(),
		// 	"counter address does not match"
		// )

		// assert(
		// 	service.mint.toBase58() === service_mint.publicKey.toBase58(),
		// 	"authority does not match"
		// )
	})

	it("initialize royalties", async () => {
		console.log("---- initializing royalties ----")

		const ticket = service_ticket_token(serviceReseller.publicKey)

		console.log("token account:", ticket)

		let tx = await program.methods
			.royaltiesInit()
			.accountsStrict({
				provider: serviceProvider.publicKey,
				serviceTicketMint: service_ticket_mint.publicKey,
				systemProgram: SystemProgram.programId,
				transferHookProgram: transfer_hook_program_id,
				mintRoyaltyConfig: mintRoyaltyConfig(service_ticket_mint),
				holder: serviceReseller.publicKey,
				serviceAccount: servicePda(service_ticket_mint),
				serviceTicketToken: service_ticket_token(serviceReseller.publicKey),
				tokenProgram: TOKEN_2022_PROGRAM_ID
			})
			.signers([serviceReseller])
			.preInstructions([
				ComputeBudgetProgram.setComputeUnitLimit({
					units: 400000,
				}),
			])
			.rpc({ skipPreflight: false })

		console.log("✅ Transaction successful", explorer(tx, "tx", "custom"))

		// console.log(
		// 	"👉 ticket mint Account",
		// 	explorer(service_ticket_mint.publicKey.toBase58(), "address", "custom")
		// )

		// console.log(
		// 	"👉 ticket token Account",
		// 	explorer(ticket.toBase58(), "address", "custom")
		// )

		await sleep(3)
	})

	it("fails attempt to transfer outside marketplace", async () => {
		console.log("---- attempting to transfer outside marketplace ----")

		const ticket = service_ticket_token(serviceReseller.publicKey)

		console.log("token account:", ticket)

		await transferCheckedWithTransferHook(
			connection,
			serviceReseller,
			service_token(serviceReseller.publicKey),
			service_ticket_mint.publicKey,
			service_token(serviceReceiver.publicKey),
			serviceReseller,
			BigInt(1),
			0,
			undefined,
			{ skipPreflight: false },
			TOKEN_2022_PROGRAM_ID
		)
			.then((tx) => {
				console.log("❌ Transaction successful", explorer(tx, "tx", "custom"))
				throw "transfer succeeded. should have failed. Hook did not block transfer"
			})
			.catch(() =>
				console.log("✅ transfer failed successfully. Hook blocked transfer!!!")
			)
	})

	it("lists a service ticket for resale", async () => {
		console.log("---- lists a service ticket for resale ----")

		const ticket = service_ticket_token(serviceReseller.publicKey)

		console.log("token account:", ticket)

		let tx = await program.methods
			.relistService()
			.accountsStrict({
				associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
				holder: serviceReseller.publicKey,
				serviceTicketMint: service_ticket_mint.publicKey,
				tokenProgram: TOKEN_2022_PROGRAM_ID,
				systemProgram: SystemProgram.programId,
				serviceAccount: servicePda(service_ticket_mint),
				serviceTicketToken: service_ticket_token(serviceReseller.publicKey),
			})
			.signers([serviceReseller])
			.preInstructions([
				ComputeBudgetProgram.setComputeUnitLimit({
					units: 400000,
				}),
			])
			.rpc({ skipPreflight: true })

		console.log("✅ Transaction successful", explorer(tx, "tx", "custom"))

		console.log(
			"👉 ticket mint Account",
			explorer(service_ticket_mint.publicKey.toBase58(), "address", "custom")
		)

		console.log(
			"👉 ticket token Account",
			explorer(ticket.toBase58(), "address", "custom")
		)

		await sleep(3)

		const service = await program.account.serviceAccount.fetch(
			servicePda(service_ticket_mint)
		)

		console.log("👉 service:", JSON.parse(JSON.stringify(service)))

		// assert(
		// 	service.holder.toBase58() === serviceProvider.publicKey.toBase58(),
		// 	"counter address does not match"
		// )

		// assert(
		// 	service.mint.toBase58() === service_mint.publicKey.toBase58(),
		// 	"authority does not match"
		// )
	})

	it("resells a service ticket and pay royalties", async () => {
		console.log("---- creating a service ----")

		const ticket = service_ticket_token(serviceReseller.publicKey)

		console.log("token account:", ticket)

		let tx = await program.methods
			.resellService()
			.accountsStrict({
				associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
				reseller: serviceReseller.publicKey,
				serviceTicketMint: service_ticket_mint.publicKey,
				tokenProgram: TOKEN_2022_PROGRAM_ID,
				systemProgram: SystemProgram.programId,
				buyer: serviceReceiver.publicKey,
				provider: serviceProvider.publicKey,
				buyerServiceTicketToken: service_ticket_token(
					serviceReceiver.publicKey
				),
				resellerServiceTicketToken: service_ticket_token(
					serviceReseller.publicKey
				),
				serviceAccount: servicePda(service_ticket_mint),
				transferHookProgram: transfer_hook_program_id,
				mintRoyaltyConfig: mintRoyaltyConfig(service_ticket_mint),
			})
			.remainingAccounts([
				{
					pubkey: TOKEN_PROGRAM_ID, // token program
					isSigner: false,
					isWritable: false,
				},
				{
					pubkey: NATIVE_MINT, // token mint
					isSigner: false,
					isWritable: true,
				},
				{
					pubkey: wsol(serviceProvider.publicKey), // provider token account
					isSigner: false,
					isWritable: true,
				},
				{
					pubkey: wsol(serviceReseller.publicKey), // reseller token accunt
					isSigner: false,
					isWritable: true,
				},
				{
					pubkey: wsol(serviceReceiver.publicKey), // receiver token account
					isSigner: false,
					isWritable: true,
				},
				{
					pubkey: extraAccountMetaListPDA,
					isSigner: false,
					isWritable: true,
				},
			])
			.signers([serviceReceiver])
			.preInstructions([
				ComputeBudgetProgram.setComputeUnitLimit({
					units: 400000,
				}),
			])
			.rpc({ skipPreflight: true })

		console.log("✅ Transaction successful", explorer(tx, "tx", "custom"))

		console.log(
			"👉 ticket mint Account",
			explorer(service_ticket_mint.publicKey.toBase58(), "address", "custom")
		)

		console.log(
			"👉 ticket token Account",
			explorer(ticket.toBase58(), "address", "custom")
		)

		await sleep(3)

		const service = await program.account.serviceAccount.fetch(
			servicePda(service_ticket_mint)
		)

		console.log("👉 service:", JSON.parse(JSON.stringify(service)))

		// assert(
		// 	service.holder.toBase58() === serviceProvider.publicKey.toBase58(),
		// 	"counter address does not match"
		// )

		// assert(
		// 	service.mint.toBase58() === service_mint.publicKey.toBase58(),
		// 	"authority does not match"
		// )
	})
})
