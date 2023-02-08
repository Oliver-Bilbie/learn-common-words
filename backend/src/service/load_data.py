"""
Script to write data to the database from a csv file.
This will eventually be run in the pipelines.
"""

import pandas as pd
import boto3

# german_table_name = os.getenv("GERMAN_TABLE", None)
german_table_name = "learn-common-words-german-dev"
dynamoResource = boto3.resource("dynamodb")
german_table = dynamoResource.Table(german_table_name) if german_table_name else None


df = pd.read_csv("backend/resources/german_words.csv", index_col=0)

with german_table.batch_writer() as batch:
    for index in range(len(df.index)):
        batch.put_item(
            Item={
                "frequency": index,
                "german": df.iloc[index]["german"],
                "english": df.iloc[index]["english"],
            }
        )
