# klarna_mail_parser

## What it does
It takes post-purchase emails sent by Klarna (and stored in .eml format) and parses them all into a csv file for easy importing into financial software and storage.

## How to use
### Step 1
Download this repo and run with `cargo run` in order to compile and run from source
OR
Download prebuilt binary matching your platform and run it

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
