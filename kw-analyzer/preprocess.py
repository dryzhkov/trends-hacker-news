import pandas as pd
import re

RECORD_NUM = 500000


def pre_process(text):
    text = text.lower()

    # remove tags
    text = re.sub("</?.*?>", " <> ", text)

    # remove special characters and digits
    text = re.sub("(\\d|\\W)+", " ", text)

    return text


def combine_text(table):
    # combine title and text into a single field
    if isinstance(table['text'], str):
        table['text'] = table['title'] + " " + table['text']
    else:
        table['text'] = table['title']

    table = table.drop(columns=['title'])
    return table


def filter(table):
    # filter table to only whats needed
    table = table.drop(columns=['id', 'time', 'time_ts', 'by', 'score',
                                'deleted', 'dead', 'url', 'author', 'descendants'])

    table = table.dropna(subset=['title'], axis='rows')

    table = combine_text(table)
    # take top N records
    table = table.head(RECORD_NUM)

    return table


if __name__ == "__main__":
    # read json into a dataframe and filter it
    data = filter(pd.read_json("~/Downloads/stories_full.json", lines=True))
    # pre-process text column
    data['text'] = data['text'].apply(lambda x: pre_process(x))
    # print schema
    print("Schema:\n\n", data.dtypes)
    print("Number of questions,columns=", data.shape)

    # save results to file
    data.to_json('data/hn-history.json')
    print("Results saved to data/hn-history.json")
