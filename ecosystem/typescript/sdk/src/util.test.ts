// export const NODE_URL = process.env.APTOS_NODE_URL || 'https://fullnode.devnet.aptoslabs.com';
// export const FAUCET_URL = process.env.APTOS_FAUCET_URL || 'https://faucet.devnet.aptoslabs.com';
export const NODE_URL = process.env.APTOS_NODE_URL || 'http://127.0.0.1:8080';
export const FAUCET_URL = process.env.APTOS_FAUCET_URL || 'http://0.0.0.0:8081';

test("noop", () => {
  // All TS files are compiled by default into the npm package
  // Adding this empty test allows us to:
  // 1. Guarantee that this test library won't get compiled
  // 2. Prevent jest from exploding when it finds a file with no tests in it
});
