import * as anchor from "@coral-xyz/anchor";
import { Ordinum } from "../target/types/ordinum";
import { SPONSOR_SEED } from "./utils/constants";
import { assert } from "chai";
import { BN } from "bn.js";

describe("init sponsor", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.ordinum as anchor.Program<Ordinum>
    const signer = provider.wallet;

    it("initialise sponsor acc", async() => {
        const name = "test_sponsor";
        
        //derviving PDA
        const [sponsorPDA, bump] = anchor.web3.PublicKey.findProgramAddressSync(
            [
                Buffer.from(SPONSOR_SEED),
                signer.publicKey.toBuffer(),
                Buffer.from(name)
            ],
            program.programId
        );

        await program.methods
              .initSponsor(name)
              .accounts({
                 signer: signer.publicKey,
        }).rpc();
        
        //fetching account
        const account = await program.account.sponsor.fetch(sponsorPDA);

        // assertionn tests
        assert.equal(account.authority.toBase58(), signer.publicKey.toBase58());
        assert.equal(account.sponsorTitle, name)
        assert.equal(account.verified, false);
        assert.ok(account.trialCount.eq(new BN(0)));
        assert.equal(account.bump, bump);
        assert.ok(account.createdAt.toNumber() > 0);

    })
})