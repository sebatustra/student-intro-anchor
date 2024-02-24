import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { StudentIntro } from "../target/types/student_intro";
import { expect } from "chai";

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

    it("Initialized the student intro", async () => {
        const tx = await program.methods
            .initializeIntro(studentIntro.name, studentIntro.message)
            .rpc();
        
        const intro = await program.account
            .studentIntroState
            .fetch(introPda);

        expect(intro.name === studentIntro.name);
        expect(intro.message === studentIntro.message);
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
