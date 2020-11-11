#/usr/bin/bash

set -e

timer="60"
for (( ; ; )) do
  scrapy crawl newsflash
  echo "Sleeping for $timer seconds"
  sleep $timer
done
