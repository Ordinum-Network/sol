import * as anchor from "@coral-xyz/anchor";

export interface sponsorAcc {
    authority: anchor.web3.PublicKey;
    sponsorTitle: string;
    trialCount: anchor.BN;
    verified: boolean;
    createdAt: anchor.BN;
    bump: number;
}