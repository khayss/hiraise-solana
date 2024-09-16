import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Hiraise } from "../target/types/hiraise";
import { BN } from "bn.js";
import { assert, expect } from "chai";
import { Wallet } from "@coral-xyz/anchor/dist/cjs/provider";

describe("hiraise", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Hiraise as Program<Hiraise>;
  let creator: Wallet;
  let campaignKeyPair: anchor.web3.Keypair;

  beforeEach(async () => {
    creator = (program.provider as anchor.AnchorProvider).wallet;

    campaignKeyPair = anchor.web3.Keypair.generate();

    await program.methods
      .initialize(new BN(1000000000), new BN(100000))
      .accounts({
        campaign: campaignKeyPair.publicKey,
        creator: creator.publicKey,
      })
      .signers([campaignKeyPair])
      .rpc();
  });
  it("it initializes", async () => {
    const campaignData = await program.account.campaign.fetch(
      campaignKeyPair.publicKey
    );

    assert(campaignData.amountRaised.eq(new BN(0)), "amountRaised is not 0");
    assert(
      campaignData.creator.equals(creator.publicKey),
      "public Key does not match"
    );
  });

  it("allows donations", async () => {
    try {
      const donorKeyPair = anchor.web3.Keypair.generate();
      const donationAmount = new BN(1 * 10 ** 8);

      const airdropTxSig = await program.provider.connection.requestAirdrop(
        donorKeyPair.publicKey,
        2 * 10 ** 9
      );

      await program.provider.connection.confirmTransaction(airdropTxSig);

      const campaignBalBeforeDonation =
        await program.provider.connection.getBalance(campaignKeyPair.publicKey);

      await program.methods
        .donate(donationAmount)
        .accounts({
          campaign: campaignKeyPair.publicKey,
          donor: donorKeyPair.publicKey,
        })
        .signers([donorKeyPair])
        .rpc();

      const campaignData = await program.account.campaign.fetch(
        campaignKeyPair.publicKey
      );
      const campaignBalance = await program.provider.connection.getBalance(
        campaignKeyPair.publicKey
      );

      assert(
        campaignData.amountRaised.eq(new BN(1 * 10 ** 8)),
        "amountRaised is not 1 * 10 ** 8"
      );
      assert(
        donationAmount
          .add(new BN(campaignBalBeforeDonation))
          .eq(new BN(campaignBalance)),
        `donation amount ${donationAmount} and campaign balance ${campaignBalance} did not match`
      );

      assert(
        campaignData.amountRaised
          .add(new BN(campaignBalBeforeDonation))
          .eq(new BN(campaignBalance)),
        "amount raised and campaign balance did not match"
      );
    } catch (error) {
      console.error(error);
    }
  });

  it("allows campaign can end", async () => {
    const donorKeyPair = anchor.web3.Keypair.generate();

    const airdropTxSig = await program.provider.connection.requestAirdrop(
      donorKeyPair.publicKey,
      2 * 10 ** 9
    );

    await program.provider.connection.confirmTransaction(airdropTxSig);

    await program.methods
      .donate(new BN(1 * 10 ** 8))
      .accounts({
        campaign: campaignKeyPair.publicKey,
        donor: donorKeyPair.publicKey,
      })
      .signers([donorKeyPair])
      .rpc();

    const creatorBalBefore = await program.provider.connection.getBalance(
      creator.publicKey
    );
    const campaignBalBefore = await program.provider.connection.getBalance(
      creator.publicKey
    );

    await program.methods
      .endCampaign()
      .accounts({
        creator: creator.publicKey,
        campaign: campaignKeyPair.publicKey,
      })
      .signers([])
      .rpc();

    const creatorBalAfter = await program.provider.connection.getBalance(
      creator.publicKey
    );
    const campaignBalAfter = await program.provider.connection.getBalance(
      campaignKeyPair.publicKey
    );

    assert(
      creatorBalAfter > creatorBalBefore,
      `creator balance after ${creatorBalAfter} is less than creator balance after ${creatorBalBefore}`
    );
    assert(
      campaignBalBefore > campaignBalAfter,
      `campaign balance after ${campaignBalAfter} is not less than campaign balance before ${campaignBalBefore}`
    );
    // assert(campaignBalAfter === 0, "campaign balance after is not zero");

    it("does not allow donations after campaign ends", async () => {});

    it("does not allow campaign to end twice", async () => {});

    it("does not allow campaign to end before duration", async () => {});
  });
});
