# Learn Common Words

![Build-Status](https://s3.eu-west-1.amazonaws.com/learn-common-words.net/build-status.svg?)
[![Last-Commit](https://img.shields.io/github/last-commit/Oliver-Bilbie/learn-common-words)](https://github.com/Oliver-Bilbie/learn-common-words/blob/main/CHANGELOG.md)

---

## Overview

An endless quiz for learning the most common German words. Additional languages may follow in the future.

### [Hosted here](https://learn-common-words.net)

---

## To Do:

- Replace all *unwrap* statements with *expect* statements, and improve error handling.
- Show the correct answer after selecting an option.
- Build a HTML/CSS confetti effect to play when the score reaches a multiple of ten.
- Add more languages.

---

## Architecture

### Frontend

The frontend is written using [Yew](https://yew.rs/) with the code written in Rust, and is hosted from an [S3](https://aws.amazon.com/s3/) bucket using the static website hosting functionality. This service does not support HTTPS connections, so I have used an [Cloudfront](https://aws.amazon.com/cloudfront/) distribution along with AWS's certificate manager to provide this functionality.

### Pipelines

When a PR is created or updated, a [CodeBuild](https://aws.amazon.com/codebuild/) event is triggered which will deploy the project into a development environment, and a second event which will run the unit and integration tests. A merge into the main branch will trigger an event which will deploy the project into the production environment.

For this project I have decided to use one of AWS's default build images, since I do not have the time to consistently maintain my own. As a result, the dependencies are installed each time a build is run which is not ideal, but this works well enough for now.

The pipelines will build the required S3 bucket using [Terraform](https://www.terraform.io/) then build the frontend and copy the files to [S3](https://aws.amazon.com/s3/).
