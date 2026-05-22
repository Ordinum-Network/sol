import * as anchor from "@coral-xyz/anchor";
import { Ordinum } from "../target/types/ordinum";
import { assert } from "chai";
import { getProgramPDA } from "./helpers/getSponsor";
import { BN } from "bn.js";
import { COORDINATOR_SEED, ESCROW_SEED, PATIENT_SEED, SPONSOR_SEED, TRIAL_SEED, USDC_ADDR } from "./utils/constants";
import { getAssociatedTokenAddress } from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";

describe("patient", () => {
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

     it("prefund signer (sponsor)", async() => {
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
        const patientPubkey = patient.publicKey;

        await program.methods.initPatient(
           trialId,
           sponsor, 
           contHash
        ).accounts({
           signer: CRCPubkey,
           sponsorAuthority: sponsorAccount.authority, 
           patientWallet: patientPubkey
        }).signers([CRC]).rpc()


        const [patientPDA] = anchor.web3.PublicKey.findProgramAddressSync([
            Buffer.from(PATIENT_SEED),
            trialPDA.toBuffer(),
            patientPubkey.toBuffer()
         ],
          program.programId
        )
        const patientAcc = await program.account.patient.fetch(patientPDA)

        assert(patientAcc.numberOfVisits === 0)

    })

})