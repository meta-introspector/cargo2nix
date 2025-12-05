# Rust-based Secret Manager for Oracle OCI

## Goal
Develop a Rust application to automate the generation, secure storage, and `sops`-based encryption of GPG and SSH keys for Oracle OCI access. This application aims to provide a more robust and auditable secret management workflow, integrating with external secret stores like AWS Parameter Store.

## Core Functionalities

1.  **GPG Key Generation:**
    *   Generate a new GPG key pair (public and private keys).
    *   Allow configuration of key parameters (e.g., name, email, expiration).
    *   Output the GPG key fingerprint.

2.  **SSH Key Generation:**
    *   Generate a new SSH key pair (public and private keys).
    *   Allow configuration of key type (e.g., ED25519) and output path.

3.  **AWS Parameter Store Integration:**
    *   Securely store the generated GPG private key in AWS Parameter Store as a `SecureString`.
    *   Retrieve the GPG private key from AWS Parameter Store when needed for `sops` operations.
    *   Handle AWS authentication (e.g., via environment variables, IAM roles).

4.  **`sops.yaml` Management:**
    *   Read and parse the existing `oci/sops.yaml` configuration file.
    *   Update `oci/sops.yaml` to include the fingerprint of the newly generated GPG key in the `creation_rules` for `oci/secrets/.*\.age$`.
    *   Ensure idempotency (i.e., not adding duplicate fingerprints).

5.  **`sops` Encryption Orchestration:**
    *   Use the GPG key (retrieved from AWS Parameter Store) to encrypt the generated SSH private key using `sops`.
    *   Store the encrypted SSH key in `oci/secrets/oci-ssh-key.age`.

## Technical Approach

*   **Language:** Rust
*   **CLI Framework:** `clap` for command-line argument parsing.
*   **AWS Interaction:** `aws-sdk-ssm` for interacting with AWS Parameter Store.
*   **Key Generation:** Shell out to `gpg` and `ssh-keygen` command-line tools for key generation to leverage existing, well-tested cryptographic implementations.
*   **YAML Management:** `serde` and `serde_yaml` for parsing and serializing `sops.yaml`.
*   **Process Execution:** `std::process::Command` for shelling out to `gpg`, `ssh-keygen`, and `sops`.
*   **Error Handling:** `anyhow` for simplified error management.
*   **Logging:** `tracing` and `tracing-subscriber` for structured logging.

## Workflow Steps

The Rust application will expose subcommands to perform the following sequence of operations:

1.  `secret-manager-rs generate-gpg-key --name "OCI Nix Flake" --email "oci-nix-flake@example.com" --output-dir "./gpg_keys"`
2.  `secret-manager-rs store-gpg-key --key-path "./gpg_keys/oci-gpg-secret.asc" --param-name "/oci/gpg/private-key" --region "us-east-1"`
3.  `secret-manager-rs generate-ssh-key --output-path "./ssh_keys/id_ed25519"`
4.  `secret-manager-rs update-sops-config --gpg-fingerprint "0x..." --sops-config-path "./oci/sops.yaml"` (The fingerprint will be obtained from the `generate-gpg-key` output or a separate `list-gpg-keys` command).
5.  `secret-manager-rs encrypt-ssh-key --ssh-key-path "./ssh_keys/id_ed25519" --sops-encrypted-path "./oci/secrets/oci-ssh-key.age"`

## Challenges and Considerations

*   **GPG Key Passphrase:** The current plan generates GPG keys without a passphrase for simplicity in automation. If a passphrase is required, it would need to be managed securely (e.g., another secret in Parameter Store, or interactive input). The user's request for a "one-time PIN" is not directly addressed by this automated approach and would require a more complex interactive component.
*   **AWS Credentials:** The Rust application will need appropriate AWS credentials configured in its environment to access Parameter Store.
*   **Temporary Files:** Ensure that sensitive temporary files (e.g., unencrypted GPG private key before storing in Parameter Store, unencrypted SSH private key before `sops` encryption) are handled securely and promptly deleted.
*   **Error Handling and Retries:** Implement robust error handling and potentially retry mechanisms for AWS API calls and external command executions.
*   **Security Best Practices:** Adhere to security best practices throughout the development, especially concerning secret handling and process execution.
*   **Nix Integration:** How this Rust tool will be integrated into the Nix flake system (e.g., as a `nix-shell` dependency, or a `nix build` output) needs to be defined.
