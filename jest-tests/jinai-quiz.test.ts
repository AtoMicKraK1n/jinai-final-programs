import * as anchor from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Program } from "@coral-xyz/anchor";
import { QuizProgram } from "../target/types/quiz_program";
import { BankrunProvider, startAnchor } from "anchor-bankrun";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { createMint } from "spl-token-bankrun";
import { describe, test, beforeAll, expect } from "@jest/globals";

describe("JinAI betting system!", () => {
  // Setup variables at the top level
  let provider: BankrunProvider;
  let program: Program<QuizProgram>;
  let host: Keypair;
  let quizAccountPDA: PublicKey;
  let quizAccountBump: number;
  let quizTokenAccountPDA: PublicKey;
  let quizTokenAccountBump: number;
  let quizMint: Keypair;
  let context: any;

  // Use beforeAll to set up your environment
  beforeAll(async () => {
    // Initialize keypairs and variables
    host = Keypair.generate();

    context = await startAnchor("", [], [
      {
        address: host.publicKey,
        info: {
          lamports: 1_000_000_000, // Fund host with 1 SOL
          data: Buffer.alloc(0),
          owner: SystemProgram.programId,
          executable: false,
        },
      }
    ]);

    provider = new BankrunProvider(context); 
    anchor.setProvider(provider);

    program = anchor.workspace.QuizProgram as Program<QuizProgram>;

    [quizAccountPDA, quizAccountBump] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("jinai-quiz"),
        host.publicKey.toBuffer()
      ],
      program.programId
    );

    [quizTokenAccountPDA, quizTokenAccountBump] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("quiz-token-account"),
        quizAccountPDA.toBuffer()
      ],
      program.programId
    );

    quizMint = Keypair.generate();
  
    await createMint(
      context.banksClient,
      host,              
      host.publicKey,      
      host.publicKey,       
      9,
      quizMint
    );
  });

  it("Check whether quiz is appointed or not!", async () => {
    const tx = await program.methods.appointQuiz(
      new anchor.BN(1000), 
      10, 
      "AAA Titles", 
      60
    )
    .accountsPartial({
      host: host.publicKey,
      quizAccount: quizAccountPDA,
      quizTokenAccount: quizTokenAccountPDA,
      quizMint: quizMint.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    })
    .signers([host])
    .rpc();

    console.log("Your transaction signature", tx);
  });

  // Additional tests can be added here
  // it("Check whether players are getting connected or not!", async () => {
  //   // Test implementation here
  // });
});