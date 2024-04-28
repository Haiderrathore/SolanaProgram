import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { UserProfileBackend } from "../target/types/user_profile_backend";
import { expect } from "chai";

describe("user-profile-backend", () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.UserProfileBackend as Program<UserProfileBackend>;

  it("Initialize User", async () =>
  {
    const [userProfilePDA] = web3.PublicKey.findProgramAddressSync([Buffer.from("User"), provider.publicKey.toBuffer()], program.programId);

    const name = "Prasanta";
    const username = "Memxor";
    const email = "test@gmail.com";

    await program.methods.initUserprofile(name, username, email).rpc();

    const userInfoAccount = await program.account.userProfile.fetch(userProfilePDA);

    expect(userInfoAccount.name).to.equal(name);
    expect(userInfoAccount.username).to.equal(username);
    expect(userInfoAccount.email).to.equal(email);
  });
});