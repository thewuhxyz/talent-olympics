export const sleep = (sec: number) =>
	new Promise((resolve) => setTimeout(resolve, sec * 1000))

export const explorer = (
	data: string,
	type: "address" | "tx",
	cluster: "devnet" | "custom" | "mainnet"
) => {
	const clusterUri = cluster === "mainnet" ? "" : `?cluster=${cluster}`
	return `https://explorer.solana.com/${type}/${data}${clusterUri}`
}
