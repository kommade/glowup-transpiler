# Glowup Transpiler

Glowup Transpiler is a tool for transpiling code between JavaScript/TypeScript and a custom language called Glowup.
This was inspired by [Glowup vibes](https://github.com/christina-de-martinez/babel-plugin-glowup-vibes) and [FaceDev](https://www.youtube.com/watch?v=pgeSGBwtHW8&ab_channel=FaceDev).

Just a stupid random project idea to occupy my Saturday afternoon.

## Glowup

This just transpiles genz slang into valid Javascript/Typescript. Make sure to put in valid code or the transpiler will error.

| Your Code        | Standard JS Equivalent    |
|------------------|---------------------------|
| noCap            | true                      |
| cap              | false                     |
| lowkey.stan      | console.log               |
| lowkey.sus       | console.warn              |
| lowkey.crowed    | console.debug             |
| lowkey.cringe    | console.error             |
| lowkey.tea       | console.info              |
| yeet             | throw                     |
| L                | reject                    |
| W                | resolve                   |
| fuckup           | Error                     |
| ghosted          | return null               |
| itsGiving        | return                    |
| skrt             | break                     |
| holdUp           | async                     |
| letItCook        | await                     |
| grab             | require                   |
| finesse          | import                    |
| ships            | exports                   |
| ship             | export                    |
| fr               | assert                    |
| outOfPocket      | Infinity                  |
| dis              | this                      |
| clapback         | yield                     |
| finna            | confirm                   |
| vibeOnEvent      | addEventListener          |
| highkey          | alert                     |
| Bet              | Promise                   |
| chill            | setTimeout                |
| sussy            | ?                         |
| sussier          | ??                        |
| fuckAround       | try                       |
| findOut          | catch                     |
| lit              | let                       |
| be               | =                         |
| vibeCheck        | ==                        |
| vibeCheckFr      | ===                       |
| ate              | =>                        |

## Installation

Install the latest release from the releases tab or

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/custom-js-transpiler.git
   ```

2. Build the Rust transpiler:

   ```bash
   cd custom-js-transpiler
   cargo build --release
   ```

## Usage

To transpile a file, run the following command:

```sh
glowup-transpiler <path> [--out=<lang>] [--reverse]
```

Where:

- path is the path to the file or directory you want to transpile.
- --out=lang specifies the output language. The default is js. Other options are ts for TypeScript.
- --reverse reverses the transpilation direction. By default, the transpiler converts from Glowup to the output language. If --reverse is specified, it converts from the output language to Glowup.
