import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { QuizProgram } from "../target/types/quiz_program";
import { BankrunProvider, startAnchor } from "anchor-bankrun";

describe("Check whether the quiz is initialized!", () => {
  // Configure the client to use the local cluster.

  it("Is initialized!", async () => {
    const context = await startAnchor("", [], []);
    const program = anchor.workspace.JinaiQuiz as Program<QuizProgram>;

    const provider = new BankrunProvider(context);
    // Add your test here.
    const tx = await program.methods.appointQuiz().rpc();
    console.log("Your transaction signature", tx);
  });
});
