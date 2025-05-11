import * as anchor from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID, createMint } from "@solana/spl-token";
import { Program, type Provider } from "@coral-xyz/anchor";
import { QuizProgram } from "../target/types/quiz_program";
import { BankrunProvider, startAnchor } from "anchor-bankrun";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { Keypair, SystemProgram } from "@solana/web3.js";


describe("Check whether the quiz is initialized!", () => {
  // Configure the client to use the local cluster.

  let secondKeypair: Keypair = new anchor.web3.Keypair();

  const wallet = Keypair.generate();
  
  // console.log(wallet);
  
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

    const quizAccount = Keypair.generate();
    const quizTokenAccount = Keypair.generate();
    const quizMint = Keypair.generate();

    await createMint(
      provider.connection,
      host,
      host.publicKey,
      null, 
      9, 
      quizMint
    );
  
    const tx = await program.methods.appointQuiz
    (
      new anchor.BN(1000), 
      10, 
      "AAA Titles", 
      60
    )
    .accountsPartial
    ({
      host: host.publicKey,
      quizAccount: quizAccount.publicKey,
      quizTokenAccount: quizTokenAccount.publicKey,
      quizMint: quizMint.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    })
    .signers([host])
    .rpc();

    console.log(JSON.stringify(tx));
    console.log("Your transaction signature", tx);
  });
});
