# Early Bird Movie Ticket Alert System

This project is a Rust-based application designed to notify you as soon as tickets for a movie -from https://www.paribucineverse.com website- you are eagerly awaiting become available.
By using this, you can ensure you book your preferred showtime and seats before they sell out. You will get alerts through your Telegram channel.
## Features

- Automated checks for ticket availability
- Customizable notifications for specific movies and showtimes
- Fast and efficient ticket alerts to maximize your chances of securing the best seats.

## Installation

1. Built image from Dockerfile

```
Docker build -t earlybird .
```

2. Run the image
```
Docker run -e BOT_TOKEN=YOUR_TOKEN -e CHAT_ID=YOUR_CHAT_ID earlybird
```
3. Set a cron job to run the image every 30 minutes
```
*/30 * * * * Docker run -e BOT_TOKEN=YOUR_TOKEN -e CHAT_ID=YOUR_CHAT_ID -d earlybird
```