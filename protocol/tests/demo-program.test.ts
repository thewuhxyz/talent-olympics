import * as anchor from "@coral-xyz/anchor"
import { DemoProgram, COUNTER_SEEDS } from "../src"
import { sleep } from "./helpers"
import { assert } from "chai"

describe("demo_program", () => {
	const provider = anchor.AnchorProvider.env()

	const program = anchor.workspace.DemoProgram as anchor.Program<DemoProgram>

	const user = anchor.web3.Keypair.generate()

	console.log("👉 publickey:", user.publicKey.toBase58())
	console.log("👉 program id:", program.programId.toBase58())

	const counterPda = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from(COUNTER_SEEDS), user.publicKey.toBuffer()],
		program.programId
	)[0]

	before(async () => {
		console.log("---- airdroping token ----")

		let tx = await provider.connection.requestAirdrop(
			user.publicKey,
			0.1 * anchor.web3.LAMPORTS_PER_SOL
		)
		console.log("✅ Transaction successful", tx)
		await sleep(3)
	})

	it("creates a counter", async () => {
		console.log("---- creating a counter ----")

		const tx = await program.methods
			.createCounter()
			.accountsStrict({
				authority: user.publicKey,
				counter: counterPda,
				systemProgram: anchor.web3.SystemProgram.programId,
			})
			.signers([user])
			.rpc()

		console.log("✅ Transaction successful", tx)

		await sleep(3)

		const counter = await program.account.counter.fetch(counterPda)

		console.log("👉 counter:", JSON.parse(JSON.stringify(counter)))

		assert(
			counter.pubkey.toBase58() === counterPda.toBase58(),
			"counter address does not match"
		)

		assert(
			counter.authority.toBase58() === user.publicKey.toBase58(),
			"authority does not match"
		)

		assert(
			counter.count.toNumber() === 0,
			"counter did not initialize with zero"
		)
	})

	it("increments count", async () => {
		console.log("---- incrementing count ----")

		const tx = await program.methods
			.incrementCount()
			.accountsStrict({
				authority: user.publicKey,
				counter: counterPda,
			})
			.signers([user])
			.rpc()
		console.log("✅ Transaction successful", tx)

		await sleep(3)

		const counter = await program.account.counter.fetch(counterPda)

		console.log("👉 counter:", JSON.parse(JSON.stringify(counter)))

		assert(
			counter.count.toNumber() === 1,
			"counter does not increment count from 0"
		)
	})
})
