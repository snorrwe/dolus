# -*- coding: utf-8 -*-
import scrapy
from scrapy.loader import ItemLoader
from onrab.items import OnrabItem, WORDS

from collections import defaultdict
import hashlib


class OnrabSpider(scrapy.Spider):
    name = "onrab"
    allowed_domains = ["telex.hu", "index.hu", "origo.hu"]
    start_urls = ["https://telex.hu/", "https://index.hu", "https://origo.hu"]

    def parse(self, response):
        n_orban = 0
        count = defaultdict(lambda: 0)

        page_hash = hashlib.sha256()

        for item in response.css("::text").getall():
            page_hash.update(item.encode())
            for word in WORDS:
                if word in item.lower():
                    count[word] += 1

        il = ItemLoader(item=OnrabItem())
        il.add_value("count", dict(count))
        il.add_value("url", response.url)
        il.add_value("page_hash", str(page_hash.hexdigest()))

        return il.load_item()
