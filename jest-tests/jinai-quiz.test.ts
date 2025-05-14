import * as anchor from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Program } from "@coral-xyz/anchor";
import { QuizProgram } from "../target/types/quiz_program";
import { BankrunProvider, startAnchor } from "anchor-bankrun";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { createMint , createAssociatedTokenAccount , mintTo } from "spl-token-bankrun";
import { describe, test, beforeAll, expect } from "@jest/globals";

describe("JinAI betting system!", () => {
  // Setup variables at the top level
  let provider: BankrunProvider;
  let program: Program<QuizProgram>;
  let host: Keypair;
  let player: Keypair;
  let quizAccountPDA: PublicKey;
  let quizAccountBump: number;
  let quizTokenAccountPDA: PublicKey;
  let quizTokenAccountBump: number;
  let quizMint: Keypair;
  let context: any;

  // Use beforeAll to set up your environment
  beforeAll(async () => {
    // Setting up stuff to use through out the test!
    host = Keypair.generate();
    player = Keypair.generate();

    context = await startAnchor("", [], [
      {
        address: host.publicKey,
        info: {
          lamports: 1_000_000_000, // Fund host with 1 SOL
          data: Buffer.alloc(0),
          owner: SystemProgram.programId,
          executable: false,
        },
      },
      {
        address: player.publicKey,
        info: {
          lamports: 1_000_000_000, // Fund player with 1 SOL
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

     // Log PDAs for debugging
    // console.log("Quiz Account PDA:", quizAccountPDA.toString());
    // console.log("Quiz Token Account PDA:", quizTokenAccountPDA.toString());
    // console.log("Quiz Mint:", quizMint.publicKey.toString());
  });

  it("Check whether quiz is appointed or not!", async () => {
    try {
      const tx = await program.methods.appointQuiz(
        new anchor.BN(1000), 
        10, 
        "AAA Titles", 
        60,
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
  
      console.log("Your transaction signature for appointed quiz", tx);
    }
    catch (error) {
      console.error("Error appointing quiz:", error);
      throw error;
    }
  });

  it("Check whether players are getting connected or not!", async () => {
    try {
      const playerATA = await createAssociatedTokenAccount(
        context.banksClient,  
        player,               
        quizMint.publicKey,   
        player.publicKey      
      );

      await mintTo(
        context.banksClient,
        host,                
        quizMint.publicKey,  
        playerATA,          
        host.publicKey,     
        100000                 
        );

      const tx = await program.methods.connectPlayers().accountsPartial({
        player: player.publicKey,
        quizAccount: quizAccountPDA,
        playerTokenAccount: playerATA,
        quizTokenAccount: quizTokenAccountPDA,
        quizMint: quizMint.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([player])
      .rpc();
      console.log("Your transaction signature for connecting players", tx);

    } catch (error) {
      console.error("Error connecting players:", error);
      throw error;
    }
  });
});