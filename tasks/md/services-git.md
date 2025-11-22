# Git Services Deployment Plan

## Introduction
This document outlines a plan to package the `git-wrapper-lib` functionality as a deployable service, enabling its use in various forms on a remote server. The goal is to establish a robust and automated deployment pipeline for Git-related operations, leveraging the "eigenform" concept for flexible service instantiation.

## Service Forms (Eigenforms) of `git-wrapper-lib`

The `git-wrapper-lib` will be adaptable to different operational contexts, representing various "eigenforms" of its core functionality:

1.  **Native Crate (Binary/Library)**:
    *   **Description**: The `git-wrapper-lib` compiled directly into a standalone executable or a dynamically linked library.
    *   **Use Case**: Direct execution on the server for high performance and minimal overhead. This is the primary target for initial deployment.

2.  **RPC Call (Remote Procedure Call)**:
    *   **Description**: `git-wrapper-lib` functionality exposed via an RPC interface (e.g., gRPC, Thrift, or a custom protocol).
    *   **Use Case**: Allows other services or applications to remotely invoke Git operations without direct binary access, promoting microservices architecture.

3.  **eBPF Program (Later)**:
    *   **Description**: A future consideration to implement highly optimized, kernel-level Git operations using eBPF. This would involve a specialized "eigenform" of `git-wrapper-lib` that interacts directly with kernel events.
    *   **Use Case**: Extremely high-performance, low-latency Git operations, potentially for monitoring or specialized Git hooks. (To be explored in a later phase).

4.  **Mock (Local Development/Testing)**:
    *   **Description**: A mock implementation of the `git-wrapper-lib` API, as already developed, for local testing and development environments.
    *   **Use Case**: Facilitates rapid iteration and testing of dependent components without requiring a full Git environment or network access.

## Deployment Strategy: Terraform and SSH

The deployment to a new server will be automated using Terraform for infrastructure provisioning and secure SSH for access and binary deployment.

### 1. Server Provisioning via Terraform
*   **Action**: Use Terraform to define and provision a new virtual machine or cloud instance.
*   **Configuration**: The Terraform configuration will specify the server's operating system, resources, and initial setup.

### 2. SSH Key Generation
*   **Action**: Generate a new SSH key pair (private and public keys) on the local machine.
*   **Purpose**: To establish secure, passwordless access to the new server.

### 3. Public Key Deployment via Terraform
*   **Action**: Integrate the generated SSH public key into the Terraform configuration.
*   **Mechanism**: Terraform will ensure that the public key is automatically added to the `authorized_keys` file of the default user on the newly provisioned server.

### 4. Login to Server
*   **Action**: Once the server is provisioned, use the generated private key to establish an SSH connection to the server.
*   **Command**: `ssh -i /path/to/your/private_key user@server_ip`

### 5. Deploy Binary
*   **Action**: Transfer the compiled `git-wrapper-lib` native binary (or the RPC service executable) to the server.
*   **Mechanism**: Use `scp` or `rsync` over SSH.
*   **Example**: `scp -i /path/to/your/private_key /path/to/local/git-wrapper-lib-binary user@server_ip:/opt/git-service/git-wrapper-lib`

### 6. Use Binary to Push Git Code
*   **Action**: Execute the deployed `git-wrapper-lib` binary on the server to perform Git operations, such as pushing code to a remote repository.
*   **Example (Conceptual)**:
    ```bash
    ssh -i /path/to/your/private_key user@server_ip << 'EOF'
    /opt/git-service/git-wrapper-lib push --repo-path /path/to/repo --remote origin --branch main
    EOF
    ```
    (Note: The exact command will depend on the CLI interface of the `git-wrapper-lib` binary).

### 7. Build Code on Server
*   **Action**: After pushing the code, trigger a build process on the server for the newly updated repository.
*   **Mechanism**: This could involve invoking `cargo build`, `make`, or a custom build script.
*   **Example (Conceptual)**:
    ```bash
    ssh -i /path/to/your/private_key user@server_ip << 'EOF'
    cd /path/to/repo
    cargo build --release
    EOF
    ```

### 8. Bootstrap
*   **Action**: Perform any necessary post-deployment setup or initialization on the server.
*   **Mechanism**: This could include setting up environment variables, configuring system services, or running database migrations.
*   **Example**: Installing Rust toolchains, configuring Git credentials, setting up cron jobs.

This comprehensive plan ensures that the `git-wrapper-lib` can be effectively deployed and utilized in a secure and automated manner, supporting the project's modular and configurable architecture.