# telegram_bot_geolocation_rust

[Python version](https://github.com/antonovmike/telegram_bot_geolocation_python) of this telegram bot

Postgres
```bash
sudo su postgres
psql
CREATE USER tg_bot WITH password 'qwerty';
CREATE DATABASE telegram_db OWNER tg_bot;
\connect telegram_db;
```
