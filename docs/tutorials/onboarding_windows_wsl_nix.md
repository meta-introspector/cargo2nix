# Onboarding Guide: Setting up the Development Environment on Windows with WSL2 and Nix

This guide provides step-by-step instructions to set up your development environment on Windows using Windows Subsystem for Linux 2 (WSL2) and Nix, allowing you to contribute to the project seamlessly.

## Prerequisites

*   A Windows 10 or 11 machine.
*   Administrative privileges on your Windows machine.
*   Internet connection.

## Step 1: Install Windows Subsystem for Linux 2 (WSL2)

WSL2 allows you to run a full Linux environment directly within Windows.

1.  **Open PowerShell as Administrator:**
    *   Right-click the Start button and select "Windows PowerShell (Admin)" or "Windows Terminal (Admin)".

2.  **Enable WSL and Virtual Machine Platform:**
    *   Run the following command:
        ```powershell
        wsl --install
        ```
    *   If you've previously installed WSL and want to update to WSL2, use:
        ```powershell
        wsl --update
        wsl --set-default-version 2
        ```
    *   Reboot your system if prompted.

3.  **Install a Linux Distribution:**
    *   By default, `wsl --install` installs Ubuntu. If you want a different distribution, you can list available distributions:
        ```powershell
        wsl --list --online
        ```
    *   And install a specific one (e.g., Debian):
        ```powershell
        wsl --install -d Debian
        ```
    *   Once installed, launch the Linux distribution from the Start Menu. You will be prompted to create a username and password for your Linux environment.

## Step 2: Install Nix in WSL2

Nix is a powerful package manager that helps ensure a reproducible and consistent development environment.

1.  **Open your WSL2 Linux Terminal.**

2.  **Install Nix:**
    *   Run the official recommended installation command:
        ```bash
        sh <(curl -L https://nixos.org/nix/install) --daemon
        ```
    *   Follow the prompts. Choose "y" when asked to install Nix to the multi-user profile. Enter your password if prompted.
    *   **Important:** After the installation, you'll be instructed to either open a new shell or source a file to add Nix to your PATH. For example, it might say `source /etc/profile.d/nix.sh`. **Do this step.**

3.  **Verify Nix Installation:**
    *   Close and reopen your WSL2 terminal.
    *   Run:
        ```bash
        nix --version
        ```
    *   You should see the Nix version number.

## Step 3: Clone the Project Repository

1.  **Navigate to your desired directory in WSL2:**
    *   It's recommended to clone repositories into your Linux home directory or a subdirectory within it, not directly into mounted Windows drives (`/mnt/c/...`) to avoid performance issues.
    *   Example:
        ```bash
        cd ~
        mkdir projects
        cd projects
        ```

2.  **Clone the repository:**
    ```bash
    git clone <repository_url>
    cd <repository_name>
    ```
    *   Replace `<repository_url>` and `<repository_name>` with the actual project details.

## Step 4: Initialize the Development Environment with `nix develop`

Our project uses `nix develop` to set up the Rust environment and all necessary dependencies.

1.  **Navigate to the project root directory** (where `flake.nix` and `Cargo.toml` are located).

2.  **Enter the development shell:**
    ```bash
    nix develop
    ```
    *   This command will read the `flake.nix` file, fetch all specified dependencies, and set up the development environment. This may take some time on the first run.
    *   You will notice your shell prompt changes, indicating you are inside the Nix development shell.

3.  **Verify Rust environment:**
    *   Once inside the `nix develop` shell, verify `cargo` and `rustc` are available:
        ```bash
        cargo --version
        rustc --version
        ```

## Step 5: Build and Run the Project

With the environment set up by `nix develop`, you can now build and run the project.

1.  **Build the project:**
    ```bash
    cargo build
    ```

2.  **Run tests:**
    ```bash
    cargo test
    ```

3.  **Run specific executables (if any):**
    *   Refer to the project's `README.md` or specific documentation for running executables. For example:
        ```bash
        cargo run -p <crate_name> -- <args>
        ```

## Troubleshooting

*   **"command not found: nix"**: Ensure you have sourced the Nix profile script or opened a new terminal after installing Nix.
*   **WSL issues**: Check Microsoft's official WSL documentation for common troubleshooting steps.
*   **Slow performance on mounted drives**: Avoid working directly on `/mnt/c/` drives; clone repositories into your Linux home directory (`~`) for optimal performance.

This guide should get you up and running with the project on Windows using WSL2 and Nix. If you encounter further issues, please refer to the project's main documentation or contact the development team.
