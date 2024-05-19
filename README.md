# zk ID Prover with Hylé and RISC Zero - powered by dRPC

When you have a private group or portal, for example a portal that wants to verify that the person is part of that group (gender, employee, former provider or just a person of interest) then we will have to register them first so they can prove the relation to the group and later be able to be verified without having to reveal any further information about them.

I thought of using Worldcoin ID since the nullifier hash is linked only to my project and even if it's revealed it can´t be related to the person, so only that nullifier hash exists for that person on my app. This would bring a double factor of privacy protection for both the user and the data.

You can test the program with

cargo run [id]

the id´s that are already in the system are 1234,4567,7890

This will give you the output of the group they belong to or if the ID doesn´t belong to any groups

Thank you!