import { getWalletVersion, getToolchain } from '../utils.mjs';

await $`echo "WALLET_VERSION=${getSolanaVersion()}" >> $GITHUB_ENV`;
await $`echo "TOOLCHAIN_FORMAT=${getToolchain('format')}" >> $GITHUB_ENV`;
await $`echo "TOOLCHAIN_LINT=${getToolchain('lint')}" >> $GITHUB_ENV`;
