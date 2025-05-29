# Intro to Rust Programming

This repository contains materials, code, and exercises for the **Intro to Rust Programming** course in the **Computer Science** program at **AGH University of Kraków**. 

The development environment is set up using Rust inside a Docker container.

# CLI-Weather

A CLI app that allows user to check the current and forecast weather (up to 7 days) for chosen location.
The app saves all quary data to a `MongoDB` collection and allows user to access it.

### Quick Start Guide

The app requires a `MongoDB` connection. Using `Docker` is recomended. 

### With Docker

1. Create a `.env` file in the main directory (not src). Inside there should be:

WEATHER_API_KEY=:your weatherapi API key:
MONGO_URL=mongodb://db:27017

2. Open the terminal in this directory (.../AGH_Rust_Intro) in the treminal:

```bash
docker compose up -d

docker exec -it rust-compiler bash
```

3. Compile the project using:

```bash
cargo run
```


### Without Docker

1. Create a `.env` file in the main directory (not src). Inside there should be:

WEATHER_API_KEY=:your weatherapi API key:
MONGO_URL=:your mongodb URL:

2. Compile the project using:

```bash
cargo run
```