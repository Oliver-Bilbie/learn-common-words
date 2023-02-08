# Learn Common Words

![Build-Status](https://s3.eu-west-1.amazonaws.com/learn-common-words.net/build-status.svg?)
[![Last-Commit](https://img.shields.io/github/last-commit/Oliver-Bilbie/learn-common-words)](https://github.com/Oliver-Bilbie/learn-common-words/blob/main/CHANGELOG.md)

---

## Overview

An endless quiz for learning the most common German words. Additional languages may follow in the future.

### [Hosted here](https://learn-common-words.net)

---

## Roadmap

-   Support for other languages.
-   Support for anonymously collecting incorrect answers and using this data to increase the likelihood of the most commonly picked decoy answers for a given word to appear alongside it (data collection will be opt-in because I respect user privacy).

---

## Architecture

### Frontend

The frontend is written using [React](https://reactjs.org/) with the code written in TypeScript. I have used the [Grommet](https://v2.grommet.io/) component library for UI elements as well as icons, and [React-Confetti](https://www.npmjs.com/package/react-confetti) to provide a confetti effect.
The frontend is hosted from an [AWS S3](https://aws.amazon.com/s3/) bucket using the static website hosting functionality.

### Backend

The backend is written in Python 3.9 and is deployed as a set of [AWS Lambda](https://aws.amazon.com/lambda/) functions. The deployment is handled using [Serverless Framework](https://www.serverless.com/framework/) along with the [Serverless Python Requirements](https://www.serverless.com/plugins/serverless-python-requirements) and [Serverless IAM Roles Per Function](https://www.serverless.com/plugins/serverless-iam-roles-per-function) plugins. The Serverless Python Requirements plugin is used to create a Lambda layer with the required Pip dependencies which is driven from the backend Pipfile, and the Serverless IAM Roles Per Function plugin is used to conveniently control IAM permissions on a per-function basis.
The backend uses a [DynamoDB](https://aws.amazon.com/dynamodb/) table to store the word list.

### Pipelines

When a PR is created or updated, a [CodeBuild](https://aws.amazon.com/codebuild/) event is triggered which will deploy the project into a development environment, and a second event which will run the unit and integration tests. A merge into the main branch will trigger an event which will deploy the project into the production environment.

For this project I have decided to use one of AWS's default build images, since I do not have the time to consistently maintain my own. As a result, the dependencies are installed each time a build is run which is not ideal, but this works well enough for now.

The pipelines will build the required DynamoDB table using [Terraform](https://www.terraform.io/), deploy the backend using [Serverless Framework](https://www.serverless.com/framework/), and then finally build the frontend and copy the files to [AWS S3](https://aws.amazon.com/s3/).
