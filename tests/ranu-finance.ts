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
  const connection = program.provider.connection;

  let tokenMint = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    let fee = new BN(1);

    let [ranuConfig] = PublicKey.findProgramAddressSync(
      [Buffer.from(RANU_CONFIG_SEED)],
      program.programId,
    );

    const tx = await program.methods
      .initialize(fee)
      .accounts({ ranuConfig })
      .rpc();

    console.log("Your transaction signature", tx);
  });

  it("Pool created!", async () => {
    let maxCap = new BN("100000000000"); // 100 SOL

    let [ranuConfig] = PublicKey.findProgramAddressSync(
      [Buffer.from(RANU_CONFIG_SEED)],
      program.programId,
    );

    let [pool] = PublicKey.findProgramAddressSync(
      [Buffer.from(POOL_SEED), tokenMint.publicKey.toBuffer()],
      program.programId,
    );

    let [poolSolVault] = PublicKey.findProgramAddressSync(
      [Buffer.from(SOL_POOL_SEED), tokenMint.publicKey.toBuffer()],
      program.programId,
    );

    const tx = await program.methods
      .createPool(maxCap)
      .accounts({
        ranuConfig,
        pool,
        tokenMint: tokenMint.publicKey,
        poolSolVault,
      })
      .signers([tokenMint])
      .rpc();

    console.log("Your transaction signature", tx);
  });

  it("Deposit", async () => {
    let solAmount = new BN("100000000000"); // 100 SOL

    let [ranuConfig] = PublicKey.findProgramAddressSync(
      [Buffer.from(RANU_CONFIG_SEED)],
      program.programId,
    );

    let [pool] = PublicKey.findProgramAddressSync(
      [Buffer.from(POOL_SEED), tokenMint.publicKey.toBuffer()],
      program.programId,
    );

    let [poolSolVault] = PublicKey.findProgramAddressSync(
      [Buffer.from(SOL_POOL_SEED), tokenMint.publicKey.toBuffer()],
      program.programId,
    );

    let userTokenAccount = await getAssociatedTokenAddress(
      tokenMint.publicKey,
      program.provider.publicKey,
      true,
    );

    const tx = await program.methods
      .deposit(solAmount)
      .accounts({
        ranuConfig,
        pool,
        poolSolVault,
        userTokenAccount,
        tokenMint: tokenMint.publicKey,
      })
      .rpc();

    console.log("Your transaction signature", tx);

    const tokenBalance = (
      await connection.getTokenAccountBalance(userTokenAccount)
    ).value.uiAmount;
    console.log("Token balance after deposit: ", tokenBalance);

    const solBalance = await connection.getBalance(program.provider.publicKey);
    console.log("Sol balance after deposit: ", solBalance / 10 ** 9, "SOL");
  });

  it("Withdraw", async () => {
    let solAmount = new BN("10000000000"); // 100 SOL

    let [ranuConfig] = PublicKey.findProgramAddressSync(
      [Buffer.from(RANU_CONFIG_SEED)],
      program.programId,
    );

    let [pool] = PublicKey.findProgramAddressSync(
      [Buffer.from(POOL_SEED), tokenMint.publicKey.toBuffer()],
      program.programId,
    );

    let [poolSolVault] = PublicKey.findProgramAddressSync(
      [Buffer.from(SOL_POOL_SEED), tokenMint.publicKey.toBuffer()],
      program.programId,
    );

    const tx = await program.methods
      .withdraw(solAmount)
      .accounts({
        ranuConfig,
        pool,
        poolSolVault,
        tokenMint: tokenMint.publicKey,
      })
      .rpc();

    console.log("Your transaction signature", tx);

    const solBalance = await connection.getBalance(program.provider.publicKey);
    console.log("Sol balance after withdraw: ", solBalance / 10 ** 9, "SOL");
  });
});
