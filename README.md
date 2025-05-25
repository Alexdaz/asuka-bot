![alt text](https://raw.githubusercontent.com/Alexdaz/asuka-bot/main/Images/AsukaLogo.png)

<p align="center">
  <a href="https://opensource.org/licenses/MIT/">
      <img src="https://img.shields.io/badge/License-MIT-orange.svg" alt="Asuka-bot is released under the MIT license." />
  </a>
  <img src="https://img.shields.io/badge/Release-1.2.0-blue" />
</p>

### 💗 Asuka-bot

Asuka-bot is a free bot for Discord that was created using Rust. I made this bot for my friend's Discord server, but it might be useful for anyone else. This bot is focused on privacy as you have complete control over the code.

### 🕹️ Commands

- `help` -> Lists all commands.
- `animedex` -> Enter the anime you want to know more about.
- `yugi` -> Enter the card's name to get the most relevant info.
- `felizjueves` -> This command wish you a happy Thursday (obviously it only works on Thursdays).
- `poll` -> Make a poll where people can vote.

### ⚙️ Config File

The configuration file is contained within the `Settings.toml` file.

- `prefix`: Prefix for commands.
- `activity`: Define an activity for the bot.
- `timezone`: Time zone for the felizjueves command.
- `debug`: The most important events of the bot should be logged into the console and in a file, for troubleshooting purposes. To start the log, enter `1` and `0` to stop it.

Example

```
[settings]
prefix = '$'
activity = "Napping..."
timezone = "America/New_York"
debug = 0
```
If the file does not exist, it will be generated automatically.

### 🔒 Token

When you launch the bot for the first time, it will prompt you to enter the token generated from the Discord developer portal. After you’ve entered the token, it will be saved in a hidden and encrypted file that cannot be viewed or edited by anyone. The encryption algorithm used is AES-256-GCM.