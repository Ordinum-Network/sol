import * as anchor from "@coral-xyz/anchor";
import { Ordinum } from "../target/types/ordinum";
import { ESCROW_SEED, SPONSOR_SEED, TRIAL_SEED } from "./utils/constants";
import { assert } from "chai";
import { BN } from "bn.js";
import { getProgramPDA } from "./helpers/getSponsor";

describe("sponsor", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const connection = provider.connection;

    const program = anchor.workspace.ordinum as anchor.Program<Ordinum>
    const signer = provider.wallet;
    const sponsor: string = "pFizer2"
    let sponsorPDA: any
    let trialId:any;
    let trialPDA: anchor.web3.PublicKey
    let escrowPDA: anchor.web3.PublicKey
    let trialAcc;

    // it("initialise sponsor acc", async() => {
    //     const name = "test_sponsor";
        
    //     //derviving PDA
    //     const [sponsorPDA, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    //         [
    //             Buffer.from(SPONSOR_SEED),
    //             signer.publicKey.toBuffer(),
    //             Buffer.from(name)
    //         ],
    //         program.programId
    //     );

    //     await program.methods
    //           .initSponsor(name)
    //           .accounts({
    //              signer: signer.publicKey,
    //     }).rpc();
        
    //     //fetching account
    //     const account = await program.account.sponsor.fetch(sponsorPDA);

    //     // assertionn tests
    //     assert.equal(account.authority.toBase58(), signer.publicKey.toBase58());
    //     assert.equal(account.sponsorTitle, name)
    //     assert.equal(account.verified, false);
    //     assert.ok(account.trialCount.eq(new BN(0)));
    //     assert.equal(account.bump, bump);
    //     assert.ok(account.createdAt.toNumber() > 0);

    // })
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

     it ("create trial with associated sponsor", async() => {
            const now = Math.floor(Date.now() / 1000);
            const trial = {
                trialId: String(Date.now()),
                sponsorTitle: sponsor,
                totalPhases: 23,
                startDate: new BN(now),
                endDate: new BN(now + 30 * 24 * 60 * 60) 
            }    
            const [trialPda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
              [
                Buffer.from(TRIAL_SEED),
                signer.publicKey.toBuffer(),
                Buffer.from(trial.trialId),
                sponsorPDA.toBuffer()
              ],
              program.programId
            )
            trialPDA = trialPda
        
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
            
            trialId = trial.trialId
            const trialAccount = await program.account.trial.fetch(trialPDA);
            trialAcc = trialAccount;
    
            assert.isTrue(trialAccount.sponsor.equals(sponsorPDA));
            assert.equal(trialAccount.title, trial.trialId);
     }) 
    
     it ("init escrow account with ATA", async() => {
            const [escrowPda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from(ESCROW_SEED),
                    Buffer.from(trialId),
                    sponsorPDA.toBuffer()
                ],
                program.programId
            )
            escrowPDA = escrowPda
    
    
            await program.methods
                  .initEscrow(
                     trialId,
                     sponsor,
                     new BN(100),
                     new BN(100 * anchor.web3.LAMPORTS_PER_SOL)
                  )
                  .accounts({
                    signer: signer.publicKey,
                  }).rpc()
           
            const escrowAcc = await program.account.escrow.fetch(escrowPDA);
            assert.isTrue(escrowAcc.trial.equals(trialPDA));      
                  
     })

    it("prefund signer for update", async() => {
        console.log(await connection.getBalance(signer.publicKey), "sssssssseeeeee11111111")
        await program.methods.prefundSignerForUpdate(
            trialId,
            sponsor
        ).accounts({
            signer: signer.publicKey,
            sponsorAuthority: signer.publicKey
        }).rpc()

        console.log(await connection.getBalance(signer.publicKey), "sssssssseeeeee22222222")
    })

    it ("update verified", async () => {
        await program.methods.updateSponsorVerified(
          sponsor,
          true
        ).accounts({
            signer: signer.publicKey
        }).rpc()

        const result = (await getProgramPDA(signer, program, sponsor))!;

        const { sponsorAcc } = result!;
        assert.equal(sponsorAcc.verified, true)
    })
})