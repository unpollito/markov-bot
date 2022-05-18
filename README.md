# markov-bot

Telegram bot that reads messages, generates a first-order
[Markov chain](https://en.wikipedia.org/wiki/Markov_chain) from these, and then
generates sentences based on the read messages, often to hilarious results.

Markov chain data is stored in a MongoDB database. The Telegram bot is implemented
using [simple-telegram-bot](https://github.com/unpollito/simple-telegram-bot).
