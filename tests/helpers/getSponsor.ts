import { Wallet } from "@coral-xyz/anchor/dist/cjs/provider";
import { SPONSOR_SEED } from "../utils/constants";
import { Ordinum } from "../../target/types/ordinum";
import * as anchor from "@coral-xyz/anchor";

export const getProgramPDA = async(signer: Wallet, program: anchor.Program<Ordinum>, name: string) => {
    const [sponsorPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
        Buffer.from(SPONSOR_SEED),
        signer.publicKey.toBuffer(),
        Buffer.from(name)
    ],
    program.programId
   );

    try {
    const sponsorAcc = await program.account.sponsor.fetch(sponsorPda);
    return { sponsorPda, sponsorAcc };
  } catch (e) {
    await program.methods
      .initSponsor(name)
      .accounts({
        signer: signer.publicKey,
      })
      .rpc();

    const sponsorAcc = await program.account.sponsor.fetch(sponsorPda);
    return { sponsorPda, sponsorAcc };
  }
}