# Rustaceans - Poker Project (Dealer/Server)

## Contributors

The Rustaceans team: James Fu, Cedric Boucher, Katharine Zhong

## What is This?

This repository contains our poker project's server, which acts as the dealer and manages poker games, for the ECE 421 project 1+3.
Three poker variants are supported: five card draw, seven card stud, and texas hold'em.
The game must be played with multiple people, as no bots have been implemented to play the game automatically.
The server can be run standalone and played in the command line, but all players must use the same terminal, and cards
can obviously not be hidden from other players due to the limitations of a single terminal.
The client part of this project is intended to be used alongside this server, which allows each player
to run their own client independently, on separate machines, which eliminates the single terminal limitations and
provides a more user-friendly interface.

## Quick Start Guide

### Requirements

- MongoDB must be installed on the machine that the server will run on.
- Rust must be installed on the machine that the server will run on, if the source code is to be compiled locally.

### Installation

If all of the requirements for installation have been met, then installation is simple:
clone this repository, and run `cargo run --release` to build and run an optimized/release binary.
No configuration is required.

### Usage

If a pre-complied executable binary is used, simply run the executable to run the server.
