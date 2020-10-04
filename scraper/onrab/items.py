# -*- coding: utf-8 -*-

# Define here the models for your scraped items
#
# See documentation in:
# https://docs.scrapy.org/en/latest/topics/items.html

import scrapy


WORDS = ["orbán", "baloldal", "fidesz", "gyurcsány", "migráns", "koronavírus", "vírus", "soros"]


class OnrabItem(scrapy.Item):
    count = scrapy.Field()
    top_words = scrapy.Field()
    url = scrapy.Field()
    page_hash = scrapy.Field()
