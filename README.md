# SQL CHATBOT

This project uses the OpenAI API to generate SQL code for a given statement. It takes user input, combines it with a preamble, and sends a request to the OpenAI API to generate SQL code. The response is then displayed to the user.

## Getting Started

To get started with this project, follow these steps:

1. Clone the repository to your local machine.
2. Install the required dependencies using `cargo build`.
3. Set up your `.env` file with your OpenAI access token.
4. Run the program using `cargo run`.

## Dependencies

This project uses the following dependencies:

- dotenv
- hyper
- hyper-tls
- serde
- spinners

## Usage

Once the program is running, enter a question or a statement for which you want to generate SQL code. The program will then send a request to the OpenAI API and display the generated SQL code.

## Example

```sh
$ cargo run
> Generate SQL code for listing all employees.
