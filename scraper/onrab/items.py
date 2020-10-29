# -*- coding: utf-8 -*-

# Define here the models for your scraped items
#
# See documentation in:
# https://docs.scrapy.org/en/latest/topics/items.html

import scrapy
import os
import sys

WORDS = os.getenv("WORDS")

if WORDS:
    WORDS = WORDS.split(";")
else:
    try:
        with open("words.txt") as f:
            WORDS = list(f.readlines())
    except FileNotFoundError:
        print("No `words.txt` file was provided", file=sys.stderr)

assert WORDS, "Can't start collection without a list of words"


class OnrabItem(scrapy.Item):
    count = scrapy.Field()
    top_words = scrapy.Field()
    url = scrapy.Field()
    page_hash = scrapy.Field()
