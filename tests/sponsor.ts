import * as anchor from "@coral-xyz/anchor";
import { Ordinum } from "../target/types/ordinum";
import { SPONSOR_SEED } from "./utils/constants";
import { assert } from "chai";
import { BN } from "bn.js";
import { getProgramPDA } from "./helpers/getSponsor";

describe("sponsor", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.ordinum as anchor.Program<Ordinum>
    const signer = provider.wallet;
    const sponsor: string = "pFizer2"
    let sponsorPDA: any

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
})