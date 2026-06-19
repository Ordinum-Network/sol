import * as anchor from "@coral-xyz/anchor";
import { Ordinum } from "../target/types/ordinum";
import { assert } from "chai";
import { getProgramPDA } from "./helpers/getSponsor";
import { BN } from "bn.js";
import { COORDINATOR_SEED, ESCROW_SEED, PATIENT_SEED, PHASE, SPONSOR_SEED, TRIAL_SEED, USDC_ADDR, VISIT_RECORD } from "./utils/constants";
import { getAssociatedTokenAddress } from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";

describe("visit record", () => {
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
    let CRC: anchor.web3.Keypair
    let CRCPubkey: anchor.web3.PublicKey
    let sponsorAccount: any
    let patientPubkey: anchor.web3.PublicKey
    let patientPDA: anchor.web3.PublicKey
    let phasePda: anchor.web3.PublicKey
    let trialAcc: any

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
      
        CRC = coordinatorKeypair
        CRCPubkey = coordinatorPubkey
        
        const [derivedSponsorPDA] = PublicKey.findProgramAddressSync(
         [Buffer.from(SPONSOR_SEED), signer.publicKey.toBuffer(), Buffer.from(sponsor)],
           program.programId
         )


        const [crcCoordinatorPDA] = anchor.web3.PublicKey.findProgramAddressSync(
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
        
        const coordinatorAcc = await program.account.coordinator.fetch(crcCoordinatorPDA);
        assert.isTrue(coordinatorAcc.sponsor.equals(sponsorPDA));
        assert.isTrue(coordinatorAcc.trialId.equals(trialPDA));
    })

    it("prefund signer (crc)", async() => {
        console.log(await connection.getBalance(CRCPubkey), "CRC Balance before transfer")

        await program.methods.prefundSignerAsCrc(
            trialId,
            sponsor
        ).accounts({
            signer: CRCPubkey,
            sponsorAuthority: signer.publicKey
        }).signers([CRC]).rpc()
        
        
        console.log(await connection.getBalance(CRCPubkey), "CRC Balance after transfer")
    })

  
    it ("init patient", async() => {
          const contHash = Array.from(anchor.web3.Keypair.generate().publicKey.toBytes())
          const patient = anchor.web3.Keypair.generate()
          const _patientPubkey = patient.publicKey;
      
          patientPubkey = _patientPubkey
          await program.methods.initPatient(
             trialId,
             sponsor, 
             contHash
          ).accounts({
             signer: CRCPubkey,
             sponsorAuthority: sponsorAccount.authority, 
             patientWallet: patientPubkey
          }).signers([CRC]).rpc()
  
          const [patientpda] = anchor.web3.PublicKey.findProgramAddressSync(
           [
               Buffer.from(PATIENT_SEED),
               trialPDA.toBuffer(),
               patientPubkey.toBuffer()
           ],
           program.programId
          );
          patientPDA = patientpda
  
          const patientAcc = await program.account.patient.fetch(patientPDA);
          assert.isTrue(patientAcc.wallet.equals(patientPubkey))
          
    })

    it("prefund signer (crc)", async() => {
        console.log(await connection.getBalance(CRCPubkey), "CRC Balance before transfer")

        await program.methods.prefundSignerAsCrcForVisit(
            trialId,
            sponsor
        ).accounts({
            signer: CRCPubkey,
            sponsorAuthority: signer.publicKey
        }).signers([CRC]).rpc()
        
        
        console.log(await connection.getBalance(CRCPubkey), "CRC Balance after transfer")
    })

        it("prefund signer (crc)", async() => {
        console.log(await connection.getBalance(CRCPubkey), "CRC Balance before transfer")

        await program.methods.prefundSignerAsCrcForPhase(
            trialId,
            sponsor
        ).accounts({
            signer: CRCPubkey,
            sponsorAuthority: signer.publicKey
        }).signers([CRC]).rpc()
        console.log(await connection.getBalance(CRCPubkey), "CRC Balance After transfer")
    })

        it("init phase", async() => {
           const hash = Array.from(anchor.web3.Keypair.generate().publicKey.toBytes())
           await program.methods.initPhase(
            trialId,
            sponsor, 
            hash,
            19
           ).accounts({
            signer: CRCPubkey,
            sponsorAuthority: sponsorAccount.authority
           }).signers([CRC]).rpc()
    
           const [phasePDA] = anchor.web3.PublicKey.findProgramAddressSync([
              Buffer.from(PHASE),
              trialPDA.toBuffer(), 
              new BN(trialAcc.currentPhase).toArrayLike(Buffer, "le", 1),
           ], program.programId)

           phasePda = phasePDA
    
           const phaseAcc = await program.account.phase.fetch(phasePDA)
           assert.isTrue(phaseAcc.trialId.equals(trialPDA))
           assert.isTrue(phaseAcc.sponsor.equals(sponsorPDA))
        })

    it ("init visit record", async() => {
        const contHash = Array.from(anchor.web3.Keypair.generate().publicKey.toBytes());
       
  
        const patientAcc = await program.account.patient.fetch(patientPDA);
        // const count = patientAcc.numberOfVisits;

        const acc = await program.methods.visitRecord(
            trialId,
            sponsor,
            1,
            contHash
        ).accounts({
            signer:CRCPubkey,
            sponsorAuthority: sponsorAccount.authority,
            patientWallet: patientPubkey,
            phaseAccount: phasePda
        }).signers([CRC]).rpc()

        const updatedPatientAcc = await program.account.patient.fetch(patientPDA);

        const [visitRecordPDA] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from(VISIT_RECORD),
                trialPDA.toBuffer(),
                patientPDA.toBuffer(),
                new BN(1).toArrayLike(Buffer, "le", 1),
                new BN(patientAcc.numberOfVisits).toArrayLike(Buffer, "le", 1),
            ],
            program.programId
        )
       assert(updatedPatientAcc.numberOfVisits === patientAcc.numberOfVisits + 1);
       
       const visitRecordAcc = await program.account.visitRecord.fetch(visitRecordPDA)
       assert.isTrue(visitRecordAcc.patient.equals(patientPDA))
       assert.isTrue(visitRecordAcc.trial.equals(trialPDA))
    })

})