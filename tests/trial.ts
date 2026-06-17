import * as anchor from "@coral-xyz/anchor"
import { Ordinum } from "../target/types/ordinum";
import { TRIAL_SEED } from "./utils/constants";
import { assert } from "chai";
import { BN } from "bn.js";
import { getProgramPDA } from "./helpers/getSponsor";

describe("trial", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.ordinum as anchor.Program<Ordinum>;
    const signer = provider.wallet;
    const sponsor: string = "pFizer"
    let sponsorPDA: any
    let trialId: string;
    
    
    it ("initialise sponsor acc in trial", async () => {
       const result = (await getProgramPDA(signer, program, sponsor))!;
       if (!result) throw new Error("Sponsor not found");

       const { sponsorPda, sponsorAcc } = result!;
       sponsorPDA = sponsorPda;
       assert.equal(sponsorAcc.authority.toBase58(), signer.publicKey.toBase58());
    })

    it ("create trial with associated sponsor", async() => {
        const now = Math.floor(Date.now() / 1000);
        const trial = {
            trialId: "NOVA-Resp Trial"+Date.now(),
            sponsorTitle: sponsor,
            totalPhases: 23,
            startDate: new BN(now),
            endDate: new BN(now + 30 * 24 * 60 * 60) 
        }
        trialId = trial.trialId

        const [trialPDA, bump] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            Buffer.from(TRIAL_SEED),
            signer.publicKey.toBuffer(),
            Buffer.from(trial.trialId),
            sponsorPDA.toBuffer()
          ],
          program.programId
        )

        await program.methods
              .initTrial(
                trial.trialId,
                trial.sponsorTitle,
                trial.totalPhases,
                trial.startDate,
                trial.endDate
              )
              .accounts({
                signer: signer.publicKey,
        }).rpc()

        const trialAccount = await program.account.trial.fetch(trialPDA);
        assert.isTrue(trialAccount.sponsor.equals(sponsorPDA));
        assert.equal(trialAccount.title, trial.trialId);
    })

    it ("update total phases in trial", async() => {
      
      await program.methods.updateTotalPhasesInTrial(
        24,
        sponsor,
        trialId
      ).accounts({
        signer: signer.publicKey
      }).rpc()

       const [trialPDA, bump] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            Buffer.from(TRIAL_SEED),
            signer.publicKey.toBuffer(),
            Buffer.from(trialId),
            sponsorPDA.toBuffer()
          ],
          program.programId
        )

        const trialAcc = await program.account.trial.fetch(trialPDA)

        assert.ok(new BN(trialAcc.totalPhases).eq(new BN(24)))
    })
   
    it ("update status in trial", async() => {
      
      await program.methods.updateStatusInTrial(
        TrialStatus.InProgress,
        sponsor,
        trialId,
      ).accounts({
        signer: signer.publicKey
      }).rpc()

       const [trialPDA, bump] = anchor.web3.PublicKey.findProgramAddressSync(
          [
            Buffer.from(TRIAL_SEED),
            signer.publicKey.toBuffer(),
            Buffer.from(trialId),
            sponsorPDA.toBuffer()
          ],
          program.programId
        )

        const trialAcc = await program.account.trial.fetch(trialPDA)

        assert.ok(new BN(trialAcc.totalPhases).eq(new BN(24)))
    })
})

