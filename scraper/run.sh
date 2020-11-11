#/usr/bin/bash

set -e

timer="60"
for (( ; ; )) do
  scrapy crawl news_indices
  echo "Sleeping for $timer seconds"
  sleep $timer
done
