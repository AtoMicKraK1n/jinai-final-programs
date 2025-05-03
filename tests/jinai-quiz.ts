import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { QuizProgram } from "../target/types/quiz_program";

describe("jinai-quiz", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.JinaiQuiz as Program<QuizProgram>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.appointQuiz().rpc();
    console.log("Your transaction signature", tx);
  });
});
