# Update `secp256k1` Source Code

The document provides the steps to update the `secp256k1` source code used in this project.

## Updating the Source Code

1. Update the `COMMIT_SHA` in [src/main.rs](src/main.rs#L9) to the desired commit hash.
2. Execute the following command to download and update the [p256k1/_secp256k1](p256k1/_secp256k1) directory:
   ```shell
   cargo run --bin update
   ```
3. Review the changes, then commit and push them to a new branch.
4. Make a Pull Request.
