"""Handlers for AWS Lambda functions"""

import os
import json
import traceback
import numpy as np
import boto3

german_table_name = os.getenv("GERMAN_TABLE", None)
dynamoResource = boto3.resource("dynamodb")
german_table = dynamoResource.Table(german_table_name) if german_table_name else None


class ValidationException(Exception):
    """
    This class is used to handle exceptions raised due to input validaiton.
    It behaves identially to the Exception class.
    """


def handle_exceptions(function):
    """
    Decorator function to handle runtime errors.
    Any errors handled within the code should be raised as "ValidationException"
    types, in which case the error message will be passed to the API.
    Any other error class will return a generic error message.

    Args:
        function [Python function]: The function to be executed

    Returns:
        [json]: Json-encoded API response
    """

    def inner(*args):
        try:
            response = function(*args)

        except ValidationException as exc:
            print(traceback.format_exc())
            response = json.dumps({"response": str(exc), "status": 400})

        except Exception:
            print(traceback.format_exc())
            response = json.dumps(
                {
                    "response": "The server was unable to process your request",
                    "status": 500,
                }
            )

        return response

    return inner


@handle_exceptions
def get_word(event, context):
    """
    Handler for the get_word Lambda function, which returns a
    random German word from the most common {word_limit} words
    along with three English words, one of which corresponds to
    the German word.

    Args:
        event: AWS Lambda event
        context: AWS Lambda context

    Returns a json object with:
        "response": response body
        "status": request status code
    """

    word_limit = int(event.get("pathParameters").get("word_limit"))

    # Validate word limit
    if word_limit < 10 or word_limit > 1000:
        raise ValidationException(f"Invalid word limit ({word_limit})")

    # Generate three random numbers
    random_numbers = np.random.randint(1, word_limit + 1, 3)
    # Check that there are no duplicates
    for index in [1, 2]:
        while random_numbers[index] in random_numbers[:index]:
            # We do not need completely (pseudo-)random numbers
            # so we will just add 1 to any duplicated to cut down
            # on computation
            random_numbers[index] = (
                random_numbers[index] + 1 if random_numbers[index] < word_limit else 1
            )

    # Get words from the database
    words = []
    for index in range(0, 3):
        words.append(
            german_table.get_item(Key={"frequency": int(random_numbers[index])}).get(
                "Item"
            )
        )

    body = {
        "word": words[0].get("german"),
        "correct": words[0].get("english"),
        "decoy_1": words[1].get("english"),
        "decoy_2": words[2].get("english"),
    }

    response = json.dumps({"response": body, "status": 200})

    return response
