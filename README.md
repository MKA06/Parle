# Parle

Parle is a simple Discord bot written in Rust that responds to certain phrases with pre-initialized responses. 

## Set up phrases and responses

Under the root directory, create a `data.json` file. Inside it, insert your content in the format:

```json
{
    "commands": {
        "ping!" : "pong!",
        "help!" : "hello! I am Parle bot. How can I be of assistance?"
    },

    "phrases" : {
        "bored" : "are you bored?",
        "star wars" : "Star Wars is cool!"
    }
}
```

Commands are the hard-coded responses and will be only executed if the message contains that command alone or at the end of the message. 

Responses to phrases will be sent if the message contains the phrase. If the message contains multiple phrases, responses will be printed under each other. 

## Usage

To use Parle, first create a `.env` file under the root directory and enter the Discord token you obtained from the Development Portal:

```env
DISCORD_TOKEN=
```

Then, set up a Rust environment and run:

```sh
cargo run
```

If you see the message "<your bot's name> is connected!", it works!
