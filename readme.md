# Alyssa Discord Bot

The Alyssa discord bot that is purpose built for the Molten Aether FFRP discord server.

## Dependencies

* **Twilight:** The core library for interacting with Discord's API ([https://twilight.rs/](https://twilight.rs/)).
* **Serde:** For working with JSON files for bot configuration ([https://serde.rs/](https://serde.rs/)).
* **Reqwest:** To make HTTP requests to the MediaWiki API ([[https://docs.rs/reqwest/latest/reqwest/](https://docs.rs/reqwest/latest/reqwest/)]).

## Commands

TBD

## Command Line

TBD

## Roadmap - 2024-03-31

The initial tasks will be to build out the necessary modules and then put them all into the pot.

### Modules
1. **Logging:**
    * Use the `env_logger` crate to configure console logging for debug messages and errors.
2. **Configuration:**
    * Use `serde` to create a struct representing your bot's configuration options (token, wiki URL, command prefix, etc.).
    * Load the configuration from a JSON file using `serde_json`.
3. **Discord Integration:**
    * Create a struct to manage the Discord client using `twilight_http`.
    * Implement functions to:
        * Connect to Discord using the bot token from config.
        * Listen for incoming events (mainly messages).
        * Send messages back to Discord channels.
4. **MediaWiki API:**
    * Use `reqwest` to make HTTP requests to the MediaWiki API.
    * Define functions to:
        * Search the Wiki based on a query string.
        * Parse the API response and extract relevant information (titles, URLs).
5. **Command Handler:**
    * Use Twilight's command framework to define commands like `!wiki-search`.
    * Parse incoming messages to check for commands and arguments.
    * Call the appropriate module functions (e.g., `MediaWiki API`) based on the command.
    * Format the search results into a user-friendly Discord embed message.
6. **Putting it Together**
   *  The `main` function will:
       * Load configuration.
       * Initialize the Discord client and event handler.
       * Register the command handler with the client.
       * Start the event loop to listen for incoming messages and handle commands.