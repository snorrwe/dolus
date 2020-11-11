# -*- coding: utf-8 -*-

# Define here the models for your scraped items
#
# See documentation in:
# https://docs.scrapy.org/en/latest/topics/items.html

import scrapy
import os
import sys
import json

WORDS = os.getenv("WORDS")

try:
    with open("words.json") as f:
        WORDS =json.load(f)
except FileNotFoundError:
    print("No `words.json` file was provided", file=sys.stderr)

assert WORDS, "Can't start collection without a list of words"


class IndexItem(scrapy.Item):
    """
    Items scraped from the index page of a news site
    """

    count = scrapy.Field()
    top_words = scrapy.Field()
    url = scrapy.Field()
    page_hash = scrapy.Field()
