# whos home

Haven't you always wanted to know which of your closest friends are at home? Now you can!

## Setup

Install Rust however you feel like doing that. We recommend using [asdf](https://github.com/asdf-vm/asdf)

Install Cargo: https://doc.rust-lang.org/cargo/getting-started/installation.html

Install Terraform, also using asdf: https://github.com/asdf-community/asdf-hashicorp

Install the AWS CLI: https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html

Install [Just](https://github.com/casey/just#installation)

Request access to the AWS project

Request access to the Terraform Cloud repo

## Publishing updates to the lambda function

Within the arrivals_handler directory, run the `just publish` command:

```bash
cd arrivals_handler
just publish
```

The `publish` command does a few things:

- compiles the rust code with Cargo, and builds it for the macOS or linux system
- zips the compiled code
- publishes the zipped contents to AWS as a lambda function

## Resources

- [AWS Console](https://us-east-2.console.aws.amazon.com/console/home?region=us-east-2)
- [Terraform Cloud](https://app.terraform.io/app/whos-home/workspaces/whos-home)
