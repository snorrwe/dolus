# -*- coding: utf-8 -*-
import scrapy
from scrapy.loader import ItemLoader
from news_indices.items import IndexItem, WORDS

from collections import Counter
import re
import hashlib
import os
import json

import psycopg2
from psycopg2.extras import Json


class NewsflashSpider(scrapy.Spider):
    name = "newsflash"
    allowed_domains = ["telex.hu", "index.hu", "origo.hu", "portfolio.hu"]
    start_urls = [
        "https://telex.hu/",
        "https://index.hu",
        "https://origo.hu",
        "https://www.portfolio.hu/",
        "https://www.szeretlekmagyarorszag.hu/",
    ]
    conn = psycopg2.connect(os.environ.get("DATABASE_URL", None))

    def parse(self, response):
        cur = self.conn.cursor()

        n_orban = 0
        count = {w["word"]: 0 for w in WORDS}

        page_hash = hashlib.sha256()

        page_hash.update(response.url.encode())

        word_count = Counter()

        for item in response.css("::text").getall():
            page_hash.update(item.encode())
            sentence = re.split("\s+", item)
            word_count.update((w for w in sentence if len(w) > 3))
            for word in WORDS:
                ic = word.get("ignoreCase", False)
                suffix = word.get("allowSuffix", False)
                word = word["word"].strip()
                if ic:
                    word = word.lower()
                for sw in sentence:
                    sw = sw.strip()
                    if ic:
                        sw = sw.lower()
                    sw = sw.strip()
                    if (suffix and sw.startswith(word)) or sw == word:
                        count[word] += 1

        page_hash = str(page_hash.hexdigest())
        top_words = dict(word_count.most_common(20))

        il = ItemLoader(item=IndexItem())
        il.add_value("count", dict(count))
        il.add_value("top_words", top_words)
        il.add_value("url", response.url)
        il.add_value("page_hash", page_hash)

        cur.execute(
            """
            INSERT INTO crawled(counts, top_words, page_hash, url)
            VALUES (%s, %s, %s, %s)
            ON CONFLICT (page_hash) DO NOTHING
            """,
            (Json(count), Json(top_words), page_hash, response.url),
        )
        self.conn.commit()

        return il.load_item()
