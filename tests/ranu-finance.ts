import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { getAssociatedTokenAddress } from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import { BN } from "bn.js";
import { RanuFinance } from "../target/types/ranu_finance";

const RANU_CONFIG_SEED = "RanuConfig";
const POOL_SEED = "VaultPool";
const SOL_POOL_SEED = "SolVaultPool";

describe("ranu-finance", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.RanuFinance as Program<RanuFinance>;

  it("Is initialized!", async () => {
    let fee = new BN(1);

    let [ranuConfig] = PublicKey.findProgramAddressSync(
      [Buffer.from(RANU_CONFIG_SEED)],
      program.programId,
    );

    const tx = await program.methods.initialize(fee).accounts(ranuConfig).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Pool created!", async () => {
    let maxCap = new BN(1);

    let [ranuConfig] = PublicKey.findProgramAddressSync(
      [Buffer.from(RANU_CONFIG_SEED)],
      program.programId,
    );

    let tokenMint = anchor.web3.Keypair.generate();

    let [pool] = PublicKey.findProgramAddressSync(
      [Buffer.from(POOL_SEED), tokenMint.publicKey.toBuffer()],
      program.programId,
    );

    let [poolSolVault] = PublicKey.findProgramAddressSync(
      [Buffer.from(SOL_POOL_SEED), tokenMint.publicKey.toBuffer()],
      program.programId,
    );

    let poolTokenAccount = await getAssociatedTokenAddress(
      tokenMint.publicKey,
      pool,
      true,
    );

    const tx = await program.methods
      .createPool(maxCap)
      .accounts({
        ranuConfig,
        pool,
        tokenMint: tokenMint.publicKey,
        poolSolVault,
        poolTokenAccount,
      })
      .signers([tokenMint])
      .rpc();

    console.log("Your transaction signature", tx);
  });
});
