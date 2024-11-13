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

The configuration file is contained within the `.env` file.

- `PREFIX`: Prefix for commands.
- `ACTIVITY`: Define an activity for the bot.
- `TIME_ZONE`: Time zone for the felizjueves command.
- `DISCORD_TOKEN`: The token used by the Discord bot.
- `DEBUG`: The most important events of the bot should be logged into the console and in a file, for troubleshooting purposes. To start the log, enter `1` and `0` to stop it.

Example

```
PREFIX='$'
ACTIVITY=Napping...
TIME_ZONE=America/New_York
DISCORD_TOKEN=YOURTOKEN-123456789
DEBUG=0
```
