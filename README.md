# klarna_mail_parser

## What it does
It takes post-purchase emails sent by Klarna (and stored in .eml format) and parses them all into a csv file for easy importing into financial software and storage.

## How to use
`Tipp: use -h when running for extra help`

### Dependencies
Cargo must be installed, check out https://rustup.rs/ in case you are missing it.

### Step 1
Download this repo and build with `cargo build` in order to compile and run from source

Binaries will be supplied at a later time

### Step 2
The program will automatically create the folder structure for you in the folder you run it in.
Put your .eml files into the `klarna_mails/input/` folder

### Step 3
Run it again and check the results in `klarna_mails/input/` folder

## Limitations
```diff
- This currently only works with the German versions of the email
```
I can't adapt it since I don't know how the English version looks but you are free to submit a pull request if you do.

## Adding support for additional languages
In order to add support for additional languages you should only need to edit src/settings.rs 

- The clutter map defines all the content in the email subject that is not the payees name (including spaces)
    - `Informationen zu deinem Kauf auf Rechnung bei `Name of the payee
    - `Information regarding your purchase with `Name of the payee
- The currency regex is used to match the amount of the payment and you will probably only need to add additional currency symbols in order to make it work

