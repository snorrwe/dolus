# -*- coding: utf-8 -*-

# Define here the models for your scraped items
#
# See documentation in:
# https://docs.scrapy.org/en/latest/topics/items.html

import scrapy


with open ("words.txt") as f:
    WORDS = list(f.readlines())


class OnrabItem(scrapy.Item):
    count = scrapy.Field()
    top_words = scrapy.Field()
    url = scrapy.Field()
    page_hash = scrapy.Field()
