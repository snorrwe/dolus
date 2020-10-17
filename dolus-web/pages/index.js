import { useEffect, useState, useMemo, useCallback } from "react";
import Head from "next/head";
import styles from "../styles/Home.module.css";
import * as moment from "moment";
import * as d3 from "d3";

const COLORS = [
  "#ff0000",
  "#00ff00",
  "#0000ff",
  "#f0f000",
  "#00f0f0",
  "#000f0f",
];

const WORDS = [
  "orbán",
  "baloldal",
  "fidesz",
  "gyurcsány",
  "migráns",
  "koronavírus",
  "covid",
  "soros györgy",
  "színművészeti",
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
        a[w] = { series: [], dates: new Set() };
        return a;
      }, {});
      for (const word of WORDS) {
        const dates = new Set();
        for (const url in data) {
          chartDataByWord[word].series.push({
            name: url,
            values: data[url].map(({ created, counts }) => {
              created = moment.utc(created);
              dates.add(created);
              return {
                x: created,
                y: counts[word] || 0.0,
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

      <main className={styles.main}>
        {data
          ? WORDS.map((w) => <WordChart key={w} word={w} data={data[w]} />)
          : null}
      </main>
    </div>
  );
}

function WordChart({ word, data }) {
  const height = 300;
  const width = 800;
  const margin = {
    bottom: 20,
    top: 20,
    left: 20,
    right: 20,
  };
  const { line, x, y } = getLineFn({
    dates: data.dates,
    width,
    series: data.series,
    height,
    width,
    margin,
  });
  const { xAxis, yAxis } = getAxisFns({
    height,
    width,
    x,
    y,
    yTitle: word,
    margin,
  });
  const chart = getChart({
    x,
    y,
    line,
    height,
    width,
    data,
    xAxis,
    yAxis,
  });
  return (
    <>
      <h2>{word}</h2>
      <div
        className={styles.chart}
        ref={(e) => {
          if (e) {
            e.innerHtml = "";
            e.appendChild(chart);
          }
        }}
      ></div>
    </>
  );
}

function getAxisFns({ height, margin, width, x, y, yTitle }) {
  return {
    xAxis: (g) =>
      g.attr("transform", `translate(0,${height - margin.bottom})`).call(
        d3
          .axisBottom(x)
          .ticks(width / 80)
          .tickSizeOuter(0)
      ),
    yAxis: (g) =>
      g
        .attr("transform", `translate(${margin.left},0)`)
        .call(d3.axisLeft(y))
        .call((g) => g.select(".domain").remove())
        .call((g) =>
          g
            .select(".tick:last-of-type text")
            .clone()
            .attr("x", 3)
            .attr("text-anchor", "start")
            .attr("font-weight", "bold")
            .text(yTitle)
        ),
  };
}

function getLineFn({ dates, series, height, margin, width }) {
  const x = d3
    .scaleUtc()
    .domain(d3.extent(dates))
    .range([margin.left, width - margin.right]);
  const y = d3
    .scaleLinear()
    .domain([0, d3.max(series, (d) => d3.max(d.values.map((d) => d.y)))])
    .nice()
    .range([height - margin.bottom, margin.top]);

  return {
    x,
    y,
    line: d3
      .line()
      .defined((d) => d)
      .x((d, i) => x(d.x))
      .y((d) => y(d.y)),
  };
}

function hover({ x, y, data }) {
  return (svg, path) => {
    function moved(event) {
      event.preventDefault();
      const pointer = d3.pointer(event, this);
      const xm = x.invert(pointer[0]);
      const ym = y.invert(pointer[1]);
      const i = d3.bisectCenter(data.dates, xm);
      const dt = data.dates[i];
      const s = d3.least(data.series, (d) => {
        const closest = d3.bisectCenter(
          d.values.map(({ x }) => x),
          dt
        );
        d.closestInd = closest;
        return Math.abs(d.values[closest].y - ym);
      });
      path
        .attr("stroke", (d) => (d === s ? null : "#ddd"))
        .filter((d) => d === s)
        .raise();
      dot.attr(
        "transform",
        `translate(${x(s.values[s.closestInd].x)},${y(
          s.values[s.closestInd].y
        )})`
      );
      dot
        .select("text")
        .text(
          `${s.name} : ${s.values[s.closestInd].x.format(
            "YYYY/M/DD HH:mm"
          )} = ${s.values[s.closestInd].y}`
        );
    }

    function entered() {
      path.style("mix-blend-mode", null).attr("stroke", "#ddd");
      dot.attr("display", null);
    }

    function left() {
      path.style("mix-blend-mode", "multiply").attr("stroke", null);
      dot.attr("display", "none");
    }
    if ("ontouchstart" in document)
      svg
        .style("-webkit-tap-highlight-color", "transparent")
        .on("touchmove", moved)
        .on("touchstart", entered)
        .on("touchend", left);
    else
      svg
        .on("mousemove", moved)
        .on("mouseenter", entered)
        .on("mouseleave", left);

    const dot = svg.append("g").attr("display", "none");

    dot.append("circle").attr("r", 2.5);

    dot
      .append("text")
      .attr("font-family", "sans-serif")
      .attr("font-size", 10)
      .attr("text-anchor", "middle")
      .attr("y", -8);
  };
}

function getChart({ line, width, height, xAxis, yAxis, data, x, y }) {
  const svg = d3
    .create("svg")
    .attr("viewBox", [0, 0, width, height])
    .style("overflow", "visible");

  svg.append("g").call(xAxis);

  svg.append("g").call(yAxis);

  const path = svg
    .append("g")
    .attr("fill", "none")
    .attr("stroke", "steelblue")
    .attr("stroke-width", 2.0)
    .attr("stroke-linejoin", "round")
    .attr("stroke-linecap", "round")
    .selectAll("path")
    .data(data.series)
    .join("path")
    .style("mix-blend-mode", "multiply")
    .attr("d", (d) => line(d.values));

  svg.call(hover({ x, y, data }), path);

  return svg.node();
}
