import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { StudentIntro } from "../target/types/student_intro";
import { expect } from "chai";
import { getAssociatedTokenAddress, getAccount } from "@solana/spl-token";

describe("student-intro", () => {
    // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.StudentIntro as Program<StudentIntro>;

    const studentIntro = {
        name: "seba",
        message: "hola!"
    };

    const [introPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [provider.publicKey.toBuffer()],
        program.programId
    );

    const [mint] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("mint")],
        program.programId
    );

    it("Initialized the token mint", async () => {
        const tx = await program.methods
            .initializeMint()
            .rpc();
    })


    it("Initialized the student intro", async () => {
        const tokenAccount = await getAssociatedTokenAddress(mint, provider.wallet.publicKey);

        const tx = await program.methods
            .initializeIntro(studentIntro.name, studentIntro.message)
            .accounts({tokenAccount})
            .rpc();
        
        const intro = await program.account
            .studentIntroState
            .fetch(introPda);

        expect(intro.name === studentIntro.name);
        expect(intro.message === studentIntro.message);

        const userAta = await getAccount(provider.connection, tokenAccount);

        expect(Number(userAta.amount)).to.equal((10* 10) ^ 6);
    })

    it("Updated the student intro", async () => {
        const newIntro = {
            name: "Sebastián",
            message: "Buenos días"
        };

        const tx = await program.methods
            .updateIntro(newIntro.name, newIntro.message)
            .rpc();

        const intro = await program.account
            .studentIntroState
            .fetch(introPda)

        expect(intro.name === newIntro.name);
        expect(intro.message === newIntro.message);
    })

    it("Closed the student intro", async () => {
        const tx = await program.methods
            .deleteIntro()
            .rpc()
    })

});
