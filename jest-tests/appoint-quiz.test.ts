import * as anchor from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Program } from "@coral-xyz/anchor";
import { QuizProgram } from "../target/types/quiz_program";
import { BankrunProvider, startAnchor } from "anchor-bankrun";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { createMint } from "spl-token-bankrun";
// import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

describe("Check whether the quiz is initialized or appointed!", () => {
  // Configure the client to use the local cluster.
  // let secondKeypair: Keypair = new anchor.web3.Keypair();
  // const wallet = Keypair.generate();
  
  it("Is initialized!", async () => {
    const host = Keypair.generate();
    
    const context = await startAnchor("", [], [
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
    
    const provider = new BankrunProvider(context); 
    anchor.setProvider(provider);
    
    const program = anchor.workspace.QuizProgram as Program<QuizProgram>;

    const [quizAccountPDA, quizAccountBump] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("jinai-quiz"),
        host.publicKey.toBuffer()
      ],
      program.programId
    );

    const [quizTokenAccountPDA, quizTokenAccountBump] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("quiz-token-account"),
        quizAccountPDA.toBuffer()
      ],
      program.programId
    );

    const quizMint = Keypair.generate();
  
    await createMint(
      context.banksClient,
      host,              
      host.publicKey,      
      host.publicKey,       
      9,
      quizMint
    );
  
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
});