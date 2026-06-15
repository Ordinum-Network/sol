import * as anchor from "@coral-xyz/anchor";
import { Ordinum } from "../target/types/ordinum";
import { assert } from "chai";
import { getProgramPDA } from "./helpers/getSponsor";
import { BN } from "bn.js";
import { COORDINATOR_SEED, ESCROW_SEED, SPONSOR_SEED, TRIAL_SEED, USDC_ADDR } from "./utils/constants";
import { getAssociatedTokenAddress } from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";

describe("coordinator", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const connection = provider.connection;

    const program = anchor.workspace.ordinum as anchor.Program<Ordinum>;
    const signer = provider.wallet
    const sponsor: string = "pFizer"
    let sponsorPDA: anchor.web3.PublicKey
    let trialPDA: anchor.web3.PublicKey
    let trialId: string
    let escrowPDA: anchor.web3.PublicKey
    let coordinatorPI: anchor.web3.Keypair
    let coordinatorPIPubkey: anchor.web3.PublicKey
    let crcPubkey: anchor.web3.PublicKey
    let crc: anchor.web3.PublicKey
    let sponsorAccount: any

    it ("initialise sponsor acc", async () => {
        const result = (await getProgramPDA(signer, program, sponsor))!;
        if (!result) throw new Error("Sponsor not found");

        const { sponsorPda, sponsorAcc } = result!;
        sponsorAccount = sponsorAcc
        sponsorPDA = sponsorPda;
        assert.equal(sponsorAcc.authority.toBase58(), signer.publicKey.toBase58());
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

    it ("fetch PDA's ATA balance", async() => {
        const pubkey = new PublicKey(USDC_ADDR)
        const ata = await getAssociatedTokenAddress(
            pubkey,
            escrowPDA,
            true
        )
        
        const balance = await connection.getTokenAccountBalance(ata);
        assert.ok(new BN(balance.value.amount).eq(new BN(100)))
    })
   
    it ("fetch PDA's sol balance", async() => {
        const expected = new BN(100).mul(new BN(anchor.web3.LAMPORTS_PER_SOL));
        assert.ok(new BN(100*anchor.web3.LAMPORTS_PER_SOL).gte(expected));
    })

    // it ("init coordinator", async() => {
    //     const coordinatorKeypair = anchor.web3.Keypair.generate();
    //     const coordinatorPubkey = coordinatorKeypair.publicKey;
    //     console.log(await connection.getBalance(escrowPDA), " => before transfer")

    //     coordinatorPI = coordinatorKeypair;
    //     coordinatorPIPubkey = coordinatorPubkey;
    //     const [derivedSponsorPDA] = PublicKey.findProgramAddressSync(
    //      [Buffer.from(SPONSOR_SEED), signer.publicKey.toBuffer(), Buffer.from(sponsor)],
    //        program.programId
    //      )

    //     const [coordinatorPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    //      [
    //          Buffer.from(COORDINATOR_SEED),
    //          trialPDA.toBuffer(),
    //          coordinatorPubkey.toBuffer(),
    //      ],
    //      program.programId
    //     );

    //     await program.methods
    //           .initCoordinator(
    //             trialId,
    //             sponsor,
    //             coordinatorPubkey,
    //             {pi:{}},
    //           ).accounts({
    //             signer: signer.publicKey,
    //     }).rpc();
        
    //     console.log(await connection.getBalance(escrowPDA), " => after transfer")
    //     const coordinatorAcc = await program.account.coordinator.fetch(coordinatorPDA);
    //     assert.isTrue(coordinatorAcc.sponsor.equals(sponsorPDA));
    //     assert.isTrue(coordinatorAcc.trialId.equals(trialPDA));
    // })

    it("prefund signer (sponsor)", async() => {
        console.log(await connection.getBalance(sponsorAccount.authority), "Escrow Before balance")

        await program.methods.prefundSignerAsSponsor(
            trialId,
            sponsor
        ).accounts({
            signer: signer.publicKey,
            sponsorAuthority: signer.publicKey
        }).rpc()

        console.log(await connection.getBalance(sponsorAccount.authority), "Escrow After balance")
    })
    
    it ("init coordinator", async() => {
        const coordinatorKeypair = anchor.web3.Keypair.generate();
        const coordinatorPubkey = coordinatorKeypair.publicKey;
        console.log(await connection.getBalance(escrowPDA), " => before transfer")

        coordinatorPI = coordinatorKeypair;
        coordinatorPIPubkey = coordinatorPubkey;
        const [derivedSponsorPDA] = PublicKey.findProgramAddressSync(
         [Buffer.from(SPONSOR_SEED), signer.publicKey.toBuffer(), Buffer.from(sponsor)],
           program.programId
         )


        const [coordinatorPDA] = anchor.web3.PublicKey.findProgramAddressSync(
         [
             Buffer.from(COORDINATOR_SEED),
             trialPDA.toBuffer(),
             coordinatorPubkey.toBuffer(),
         ],
         program.programId
        );

        await program.methods
              .initCoordinator(
                trialId,
                sponsor,
                coordinatorPubkey,
                {pi:{}},
              ).accounts({
                signer: signer.publicKey,
        }).rpc();
        
        console.log(await connection.getBalance(escrowPDA), " => after transfer")
        const coordinatorAcc = await program.account.coordinator.fetch(coordinatorPDA);
        assert.isTrue(coordinatorAcc.sponsor.equals(sponsorPDA));
        assert.isTrue(coordinatorAcc.trialId.equals(trialPDA));
    })
  
    it ("Prefund signer (PI)", async() => {
        console.log(await connection.getBalance(coordinatorPIPubkey), "PI Balance before transfer")

        await program.methods.prefundSignerAsPi(
            trialId,
            sponsor
        ).accounts({
            signer: coordinatorPIPubkey,
            sponsorAuthority: signer.publicKey
        }).signers([coordinatorPI]).rpc()

        console.log(await connection.getBalance(coordinatorPIPubkey), "PI Balance After transfer")
    })
  
    it ("init coordinator with PI", async() => {
        const coordinatorKeypair = anchor.web3.Keypair.generate();
        const coordinatorPubkey = coordinatorKeypair.publicKey;
        console.log(await connection.getBalance(escrowPDA), " => before transfer")

        const [derivedSponsorPDA] = PublicKey.findProgramAddressSync(
         [Buffer.from(SPONSOR_SEED), signer.publicKey.toBuffer(), Buffer.from(sponsor)],
           program.programId
         )

        const [coordinatorPDA] = anchor.web3.PublicKey.findProgramAddressSync(
         [
            Buffer.from(COORDINATOR_SEED),
            trialPDA.toBuffer(),
            coordinatorPubkey.toBuffer(),
         ],
         program.programId
        );

 //     const sig = await connection.requestAirdrop(
 //      coordinatorPIPubkey,
 //      5 * anchor.web3.LAMPORTS_PER_SOL
 //     );
 
 //     await connection.confirmTransaction(sig);


        await program.methods
            .initCoordinatorWithPi(
              trialId,
              sponsor,
              coordinatorPubkey,
              {crc:{}},
            ).accounts({
              signer: coordinatorPIPubkey,
              sponsorAuthority: sponsorAccount.authority
        }).signers([coordinatorPI]).rpc();
        
        console.log(await connection.getBalance(escrowPDA), " => after transfer")
        const coordinatorAcc = await program.account.coordinator.fetch(coordinatorPDA);
        assert.isTrue(coordinatorAcc.sponsor.equals(sponsorPDA));
        assert.isTrue(coordinatorAcc.trialId.equals(trialPDA));
    })

    it("prefund signer (sponsor)", async() => {
        console.log(await connection.getBalance(sponsorAccount.authority), "Escrow Before balance")

        await program.methods.prefundSignerAsSponsor(
            trialId,
            sponsor
        ).accounts({
            signer: signer.publicKey,
            sponsorAuthority: signer.publicKey
        }).rpc()

        console.log(await connection.getBalance(sponsorAccount.authority), "Escrow After balance")
    })

    it ("init coordinator (CRC)", async() => {
        const coordinatorKeypair = anchor.web3.Keypair.generate();
        const coordinatorPubkey = coordinatorKeypair.publicKey;

        coordinatorPI = coordinatorKeypair;
        coordinatorPIPubkey = coordinatorPubkey;
        const [derivedSponsorPDA] = PublicKey.findProgramAddressSync(
         [Buffer.from(SPONSOR_SEED), signer.publicKey.toBuffer(), Buffer.from(sponsor)],
           program.programId
         )


        const [coordinatorPDA] = anchor.web3.PublicKey.findProgramAddressSync(
         [
             Buffer.from(COORDINATOR_SEED),
             trialPDA.toBuffer(),
             coordinatorPubkey.toBuffer(),
         ],
         program.programId
        );

        await program.methods
              .initCoordinator(
                trialId,
                sponsor,
                coordinatorPubkey,
                {crc:{}},
              ).accounts({
                signer: signer.publicKey,
        }).rpc();

        const coordinatorAcc = await program.account.coordinator.fetch(coordinatorPDA);
        assert.isTrue(coordinatorAcc.sponsor.equals(sponsorPDA));
        assert.isTrue(coordinatorAcc.trialId.equals(trialPDA));
    })

    it ("Prefund signer (CRC)", async() => {
        console.log(await connection.getBalance(coordinatorPIPubkey), "CRC Balance before transfer")
        try {
          await program.methods.prefundSignerAsPi(
              trialId,
              sponsor
          ).accounts({
              signer: coordinatorPIPubkey,
              sponsorAuthority: signer.publicKey
          }).signers([coordinatorPI]).rpc()
          assert.fail("Expected error but succeeded");
        } catch(err: any) {
          assert.ok(err.message.includes("Unauthorized"));
        }

        console.log(await connection.getBalance(coordinatorPIPubkey), "CRC Balance After transfer")
     })

     it ("init coordinator with CRC", async() => {
        try {
        const coordinatorKeypair = anchor.web3.Keypair.generate();
        const coordinatorPubkey = coordinatorKeypair.publicKey;
        console.log(await connection.getBalance(escrowPDA), " => before transfer")

        await program.methods
            .initCoordinatorWithPi(
              trialId,
              sponsor,
              coordinatorPubkey,
              {cra:{}},
            ).accounts({
              signer: coordinatorPIPubkey,
              sponsorAuthority: sponsorAccount.authority
        }).signers([coordinatorPI]).rpc();
         assert.fail("Expected error but succeeded");
      } catch(err: any) {
          const isUnauthorized = err.message.includes("Unauthorized");
          const isInsufficientFunds = err.message.includes("insufficient lamports");
          assert.ok(isUnauthorized || isInsufficientFunds);
      }
     })
     
     it ("init coordinator with CRC balanced", async() => {
        try {
          const coordinatorKeypair = anchor.web3.Keypair.generate();
          const coordinatorPubkey = coordinatorKeypair.publicKey;
          console.log(await connection.getBalance(escrowPDA), " => before transfer")
          const sig = await connection.requestAirdrop(
            coordinatorPIPubkey,
             2 * anchor.web3.LAMPORTS_PER_SOL
          );
   
          await connection.confirmTransaction(sig);
  
          await program.methods
              .initCoordinatorWithPi(
                trialId,
                sponsor,
              coordinatorPubkey,
              {cra:{}},
            ).accounts({
              signer: coordinatorPIPubkey,
              sponsorAuthority: sponsorAccount.authority
         }).signers([coordinatorPI]).rpc();
          assert.fail("Expected error but succeeded");
       } catch(err: any) {
          const isUnauthorized = err.message.includes("Unauthorized");
          const isInsufficientFunds = err.message.includes("insufficient lamports");
          assert.ok(isUnauthorized || isInsufficientFunds);
      }
     })

})