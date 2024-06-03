# Crab Bot

## How to set up config file

place config.toml in the same directory as the executable
> config.toml
```toml
[TOKEN]
discord_token ="DISCORD TOKEN HERE"

[GUILD.MAIN]
GUILD_ID = 1 # the home server of the bot
PRESIDENT = 1 # who ever is in change of the bot

[GUILD.PARTNERS.a name or something]
id = 1
name = "Name of partner"
SEND_NEWS = true # whether the "partner" want's to receive the news posted or not
NEWS_CHANNEL = 1

[ROLES.PUBLIC]
# The supported roles id's that can be given to users
ROLE1 = 1
ROLE2 = 1
other = 1

[ROLES.PRIVATE]
# the default role given when a user accepts the rules
REMOVE_ROLE_ID = 1
# the role who should be notified if something goes wrong in registration (caused by user having higher permission than the bot or already existing in enrollments.json)
ADMIN_ROLE_ID = 1

[CHANNELS]
# Channel where user will enroll
ENROLL_CHANNEL_ID = 1
# A voice channel where the meetings happen
MEETING_CHANNEL_ID = 1
# where the news urls are posted to be shared 
READING_CHANNEL_ID = 1

# currently not used and can be removed
[FEATURES]
SEND_NEWS = true

```

## How to run

### Using the Rust installed locally
for development purposes you can remove the --release flag
```shell
cargo build --release
./target/release/crab-bot
```

### Docker installed locally
```shell
docker build --tag 'crab-bot-v2' .
docker run --detach 'crab-bot-v2'
```