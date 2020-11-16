import { useEffect, useState, useMemo, useCallback } from "react";
import Head from "next/head";
import styles from "../styles/Home.module.css";
import * as moment from "moment";

import WordChart from "../components/WordChart"

const COLORS = ["#F34A53", "#AAB384", "steelblue", "#437356", "#1E4147"];

var WORDS = [];

const COLOR_BY_URL = {};
var nextInd = 0;

function getColor(url) {
  if (COLOR_BY_URL[url]) return COLOR_BY_URL[url];

  const res = (COLOR_BY_URL[url] = COLORS[nextInd]);
  nextInd = (nextInd + 1) % COLORS.length;
  return res;
}

async function fetchWords() {
  if (WORDS && WORDS.length) return WORDS;
  let words = await fetch("https://dolus.herokuapp.com/api/words");
  words = await words.json();
  WORDS = words.sort();
  return WORDS;
}

/**
 * Table of Contents
 */
function Toc() {
  return (
    <div className={styles.tocContainer}>
      <ul>
        {WORDS.map((w) => (
          <li className={styles.tocItem} key={w}>
            {" "}
            <a href={`#${w}`}>{w}</a>
          </li>
        ))}
      </ul>
    </div>
  );
}

export default function Home() {
  const [data, setData] = useState(null);

  useEffect(() => {
    if (data) {
      return;
    }
    (async () => {
      let response = fetch("https://dolus.herokuapp.com/api/counts");
      response = await response;
      const body = await response.json();
      let words = await fetchWords();
      const data = body
        .map((o) => [o.url, o])
        .reduce((a, [url, body]) => {
          a[url] = a[url] ? [...a[url], body] : [body];
          return a;
        }, {});

      const chartDataByWord = words.reduce((a, w) => {
        a[w] = { series: [], dates: new Set() };
        return a;
      }, {});
      for (const word of words) {
        const dates = new Set();
        for (const url in data) {
          chartDataByWord[word].series.push({
            name: url,
            color: getColor(url),
            values: data[url].map(({ created, counts }) => {
              created = moment.utc(created);
              dates.add(created);
              return {
                x: created,
                y: counts[word] || null,
              };
            }),
          });
        }
        chartDataByWord[word].dates = [...dates];
        chartDataByWord[word].dates.sort((a, b) => b - a);
      }
      setData(chartDataByWord);
    })();
  }, [data, setData]);

  return (
    <div className={styles.container}>
      <Head>
        <title>Dolus</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <Toc />
      <main className={styles.main}>
        {data
          ? WORDS.map((w) => <WordChart key={w} word={w} data={data[w]} />)
          : null}
      </main>
    </div>
  );
}

