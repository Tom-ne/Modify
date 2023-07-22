# Modify

Modify is a command-line tool written in Rust that provides an easy and efficient way to manage mods for the Minecraft game. It simplifies the process of installing, removing, and updating mods, allowing Minecraft players to enhance their gameplay experience with ease.

## Current Features

- Mod installation: Easily install mods from Modrinth.
- Mod removal: Uninstall mods that are not desired.
- Mod search: Search for mods based on names.
- Dependency management: Automatically handle mod dependency installation to ensure compatibility.
- Minecraft version compatibility: Check for mod compatibility with different Minecraft versions.
- Configuration management: Customize mod settings and options using configuration files.
- User-friendly interface: Intuitive command-line interface with clear instructions and feedback.

## Future Features
- Mod backup and restore: Create backups of your Minecraft installations and restore them as needed.
- Mod updating: Keep your installed mods up to date with the latest versions available.
- Mod search: Search for mods based on keywords, categories, or tags.
- Graphical user interface: The same features, with a graphical user interface

## Installation

### Prerequisites

- Rust programming language installed on your system. You can install Rust by following the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
- Minecraft game installed on your computer.

### Steps

1. Clone or download Modify repository from [GitHub](https://github.com/TomNe/mod-manager).
2. Open a terminal or command prompt and navigate to the project's directory.
3. Build the project using the following command:

   ```shell
   cargo build --release
   ```

4. After the build process completes, you will find the executable file in the `target/release` directory.
5. Copy the executable file to a location of your choice or add it to your system's `PATH` environment variable for easy access.

## Usage

1. Open a terminal or command prompt.
2. Navigate to the directory where the Modify executable is located.
3. Run the following command to start Modify:

   ```shell
   ./mod-manager
   ```

4. Modify will launch, and you can begin managing your mods using the available commands.
5. Follow the instructions provided by Modify to install, remove, or update mods.
6. Enjoy playing Minecraft with your favorite mods!

## Contributing

Contributions to the Modify project are welcome! If you would like to contribute, please follow these steps:

1. Fork the repository and clone it to your local machine.
2. Create a new branch for your feature or bug fix.
3. Make your changes and ensure that the code builds successfully.
4. Commit your changes and push them to your forked repository.
5. Submit a pull request with a detailed description of your changes.

Please review the [Contributing Guidelines](CONTRIBUTING.md) for more information.

## License

This project is licensed under the [MIT License](LICENSE). Feel free to use, modify, and distribute it according to the terms of the license.

## Acknowledgements

- The Rust programming language and its community for providing a powerful and reliable development environment.
- The Minecraft modding community for their incredible creations and continuous inspiration.
- Modrinth for creating an easy to use and powerfull API

Enjoy modding your Minecraft game with Modify!
