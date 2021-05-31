# asuro-timer
A simple timer that uses windows notifications. Currently only works on windows.

## Installation
`cargo install asuro-timer` will install the binary on your computer.

## Usage
The timer only supports inputs in minutes.

Examples: `timer 3` or `timer 3.5`

An additional repeat duration can be entered, after the timer has run out the notification will be repeated at that interval.

Example: `timer 3+1`

## TODO
- Fix output with old windows terminal
- Show different progress bar for repeat timer
- Linux?