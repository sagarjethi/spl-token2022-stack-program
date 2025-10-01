import { expect } from 'chai';
import * as anchor from "@project-serum/anchor";
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { TOKEN_2022_PROGRAM_ID } from "@solana/spl-token-2022";
import {
  stakeMintAddress,
  beefMintAddress,
  program,
  findStakeMintAuthorityPDA
} from "../scripts/programHelper/config"
import { User } from "./user";
import { createMints } from "../scripts/programHelper/create-mints";
import { airdropBeef } from "../scripts/programHelper/airdrop-beef";
import { TokenHelper } from "./token_helper";

// Test cases for both Token and Token 2022
const testCases = [
  { name: "SPL Token", tokenProgram: TOKEN_PROGRAM_ID },
  { name: "Token 2022", tokenProgram: TOKEN_2022_PROGRAM_ID }
];

describe("staker with Token 2022 support", () => {

  before(async () => {
    await createMints();
    await airdropBeef();
  });

  testCases.forEach(({ name, tokenProgram }) => {
    describe(`${name} tests`, () => {
      
      it(`It creates the program 🐮💰 beef token bag with ${name}`, async () => {
        const user = new User();
        const [beefPDA, _] = await getProgramBeefTokenBagPDA();

        await program.rpc.createMysplAta({
          accounts: {
            mysplMint: beefMintAddress,
            mysplAtaForProgram: beefPDA,
            payer: user.wallet.publicKey,
            systemProgram: SystemProgram.programId,
            tokenProgram: tokenProgram,
            rent: SYSVAR_RENT_PUBKEY,
          }
        });

        const tokenHelper = new TokenHelper(beefMintAddress);
        expect(await tokenHelper.balance(beefPDA)).to.be.eql(0);
      });

      it(`It swaps $🐮 for $🥩 with ${name}`, async () => {
        // 0. Prepare Token Bags
        const user = new User();
        await user.getOrCreateStakeTokenBag();
        await user.getOrCreateBeefTokenBag()

        // 1. Get current stake amount
        const userStakes = await user.stakeBalance();
        const userBeefs = await user.beefBalance();

        // For the MINT
        const [stakePDA, stakePDABump] = await findStakeMintAuthorityPDA();
        // For the TRANSFER
        const [beefBagPDA, beefBagBump] = await getProgramBeefTokenBagPDA();

        // 2. Execute our stuff
        await program.rpc.stake(
          stakePDABump,
          beefBagBump,
          new anchor.BN(5_000),
          {
            accounts: {
              tokenProgram: tokenProgram,

              // **************
              // MINTING 🥩 TO USERS
              // **************
              rewardMint: stakeMintAddress,
              authorityOfRewardMint: stakePDA,
              rewardAtaForUser: user.stakeTokenBag,

              // **************
              // TRANSFERING 🐮 FROM USERS
              // **************
              mysplAtaForUser: user.beefTokenBag,
              authorityOfMysplAtaForUser: user.wallet.publicKey,
              mysplAtaForProgram: beefBagPDA,
              mysplMint: beefMintAddress,
            },
          },
        );

        // 3. Tests
        // We expect the user to have received 5_000 $🥩
        expect(await user.stakeBalance()).to.be.eql(userStakes + 5_000);

        // We expect the user to have paid 5_000 $🐮 to the program.
        expect(await user.beefBalance()).to.be.eql(userBeefs - 5_000);
        const tokenHelper = new TokenHelper(beefMintAddress);
        expect(await tokenHelper.balance(beefBagPDA)).to.be.eql(5_000)
      });

      it(`It redeems 🥩 for 🐮 with ${name}`, async () => {
        // 0. Prepare Token Bags
        const user = new User();
        await user.getOrCreateStakeTokenBag();
        await user.getOrCreateBeefTokenBag()
        // For the TRANSFER
        const [beefBagPDA, beefBagBump] = await getProgramBeefTokenBagPDA();

        // 1. Get current stake amount
        const userStakes = await user.stakeBalance();
        const userBeefs = await user.beefBalance();

        // 2. Execute our stuff
        await program.rpc.unstake(
          beefBagBump,
          new anchor.BN(5_000),
          {
            accounts: {
              tokenProgram: tokenProgram,

              // **************
              // BURNING USER'S 🥩
              // **************
              rewardMint: stakeMintAddress,
              rewardAtaForUser: user.stakeTokenBag,
              authorityOfRewardAtaForUser: user.wallet.publicKey,

              // **************
              // TRANSFER 🐮 TO USERS
              // **************
              mysplAtaForProgram: beefBagPDA,
              mysplAtaForUser: user.beefTokenBag,
              mysplMint: beefMintAddress,
            },
          }
        );

        // 3. Tests
        // We expect the user to have redeem $🥩 to the program.
        expect(await user.stakeBalance()).to.be.eql(userStakes - 5_000);

        // We expect the user to have received 5_000 beef $🐮 back.
        expect(await user.beefBalance()).to.be.eql(userBeefs + 5_000);
      });
    });
  });
});

const getProgramBeefTokenBagPDA = async (): Promise<[PublicKey, number]> => {
  const seed = beefMintAddress;

  return await PublicKey.findProgramAddress(
    [seed.toBuffer()],
    program.programId
  );
}
