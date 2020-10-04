# -*- coding: utf-8 -*-
import scrapy
from scrapy.loader import ItemLoader
from onrab.items import OnrabItem, WORDS

from collections import defaultdict, Counter
import re
import hashlib
import os
import json

import psycopg2
from psycopg2.extras import Json


class OnrabSpider(scrapy.Spider):
    name = "onrab"
    allowed_domains = ["telex.hu", "index.hu", "origo.hu", "portfolio.hu"]
    start_urls = [
        "https://telex.hu/",
        "https://index.hu",
        "https://origo.hu",
        "https://www.portfolio.hu/",
    ]
    conn = psycopg2.connect(os.environ.get("DATABASE_URL", None))

    def parse(self, response):
        cur = self.conn.cursor()

        n_orban = 0
        count = defaultdict(lambda: 0)

        page_hash = hashlib.sha256()

        page_hash.update(response.url.encode())

        word_count = Counter()

        for item in response.css("::text").getall():
            item = item.lower()
            word_count.update((w for w in re.split("\s+", item) if len(w) > 3))
            for word in WORDS:
                if word in item.lower():
                    page_hash.update(item.encode())
                    count[word] += 1

        page_hash = str(page_hash.hexdigest())

        il = ItemLoader(item=OnrabItem())
        il.add_value("count", dict(count))
        il.add_value("top_words", word_count.most_common(20))
        il.add_value("url", response.url)
        il.add_value("page_hash", page_hash)

        cur.execute(
            """
            INSERT INTO crawled(counts, top_words, page_hash, url)
            VALUES (%s, %s, %s)
            ON CONFLICT (page_hash) DO NOTHING
            """,
            (Json(count), Json(top_words), page_hash, response.url),
        )
        self.conn.commit()

        return il.load_item()
