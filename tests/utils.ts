export const print_address = (label, address) => {
    console.log(`${label}: https://explorer.solana.com/address/${address}?cluster=localnet`);
}
  
  
export const print_thread = async (clockworkProvider, address) => {
    const threadAccount = await clockworkProvider.getThreadAccount(address);
    console.log("\nThread: ", threadAccount, "\n");
    print_address("🧵 Thread", address);
    console.log("\n")
}