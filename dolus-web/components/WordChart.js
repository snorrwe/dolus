import { useState, useCallback } from "react";

import * as d3 from "d3";
import styles from "../styles/Charts.module.css";

export function Swatch({ data, props }) {
  return data.series.map(({ name, color }) => (
    <div
      {...props}
      style={{ backgroundColor: color }}
      key={name}
      id={"swatch-" + name}
    >
      {name}
    </div>
  ));
}

export default function WordChart({ word, data }) {
  const height = 450;
  const width = 1000;
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
    <div id={word}>
      <h2>{word}</h2>
      <div>
        <Swatch data={data} props={{}} />
      </div>
      <div
        className={styles.chart}
        ref={(e) => {
          if (e) {
            e.innerHtml = "";
            e.appendChild(chart);
          }
        }}
      ></div>
    </div>
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
      .defined((d) => d != null && d.y != null)
      .x((d, i) => x(d.x))
      .y((d) => y(d.y)),
  };
}

function closest(arr, val) {
  let mind = Math.abs(arr[0] - val);
  let mini = 0;
  for (let i = 1; i < arr.length; ++i) {
    const d = Math.abs(arr[i] - val);
    if (d < mind) {
      mind = d;
      mini = i;
    }
  }
  return mini;
}

function hover({ x, y, data }) {
  return (svg, path) => {
    function moved(event) {
      event.preventDefault();
      const pointer = d3.pointer(event, this);
      const xm = x.invert(pointer[0]);
      const ym = y.invert(pointer[1]);
      const i = closest(data.dates, xm);
      const dt = data.dates[i];
      const s = d3.least(data.series, (d) => {
        const cl = closest(
          d.values.map(({ x }) => x),
          dt
        );
        d.closestInd = cl;
        return Math.abs(d.values[cl].y - ym);
      });
      if (s == null) return;
      path
        .attr("stroke", (d) => (d === s ? d.color : "#ddd"))
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
      path.style("mix-blend-mode", "multiply").attr("stroke", (d) => d.color);
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

    dot.append("circle").attr("r", 4);

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
    .attr("stroke-width", 3.0)
    .attr("stroke-linejoin", "round")
    .attr("stroke-linecap", "round")
    .selectAll("path")
    .data(data.series)
    .enter()
    .append("path")
    .attr("stroke", (d) => d.color)
    .style("mix-blend-mode", "multiply")
    .attr("d", (d) => line(d.values));

  svg.call(hover({ x, y, data }), path);

  return svg.node();
}
