import { useEffect, useState, useMemo, useCallback } from "react";
import Head from "next/head";
import { Chart } from "react-charts";
import styles from "../styles/Home.module.css";

const WORDS = [
  "orbán",
  "baloldal",
  "fidesz",
  "gyurcsány",
  "migráns",
  "vírus",
  "soros",
];

export default function Home() {
  const [data, setData] = useState(null);

  useEffect(() => {
    if (data) {
      return;
    }
    (async () => {
      const response = await fetch("https://dolus.herokuapp.com/api/counts");
      const body = await response.json();
      const data = body
        .map((o) => [o.url, o])
        .reduce((a, [url, body]) => {
          a[url] = a[url] ? [...a[url], body] : [body];
          return a;
        }, {});

      const chartDataByWord = WORDS.reduce((a, w) => {
        a[w] = [];
        return a;
      }, {});
      for (const word of WORDS) {
        for (const key in data) {
          chartDataByWord[word].push({
            label: key,
            data: data[key]
              .map(({ created, counts }) => ({
                primary: new Date(created),
                secondary: counts[word] || 0.0,
              })),
          });
        }
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

      <main className={styles.main}>
        {data
          ? WORDS.map((w) => <WordChart key={w} word={w} data={data[w]} />)
          : null}
      </main>
    </div>
  );
}

function WordChart({ word, data }) {
  const series = React.useMemo(
    () => ({
      showPoints: true,
    }),
    []
  );
  const axes = useMemo(
    () => [
      {
        primary: true,
        type: "utc",
        position: "bottom",
      },
      { type: "linear", position: "left", hardMin: 0.0, hardMax: 10.0 },
    ],
    []
  );

  return (
    <>
      <h2>{word}</h2>
      <div className={styles.chart}>
        {data ? (
          <Chart
            data={data}
            series={series}
            axes={axes}
            tooltip
            primaryCursor
            secondaryCursor
            focusClosest
          />
        ) : null}
      </div>
    </>
  );
}
