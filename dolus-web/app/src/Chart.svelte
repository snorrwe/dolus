<script>
	let selected;
	let wordList = [];
	let loading = false;
	let chartValues = {};

	import {loadWords, draw, closestValues} from "./charts.js";

	$: chartCalc = draw("dolus-chart", selected);

	loadWords()
		.then(w => {
			wordList = w.sort();
			selected = wordList[0]
		});

	const onSelect = (w) => () => {
		selected = w;
	};

	const onMoved = (w) => (event) => {
		let actualRect = event.target.getBoundingClientRect();
		let logicX = event.offsetX/ actualRect.width;
			/* let logicY = event.offsetY / actualRect.height; */
		try{
			let res = closestValues({word:w, x:logicX});
			chartValues = res;
		} catch(e) {
			console.error(e);
		}
	};

</script>

<div class="container">
	<ul>
		{#each wordList as w}
			<li on:click={onSelect(w)} class="wordListItem">
				{w}
			</li>
		{/each}
	</ul>

	<div>

		<h2> {selected || ""} </h2>

		<table>
			<tbody>
				{#each Object.keys(chartValues).sort() as key}
					<tr>
						<td>
							{key}
						</td>
						<td>
							{chartValues[key][0]}
						</td>
						<td>
							{chartValues[key][1]}
						</td>
					</tr>
				{/each}
			</tbody>
		</table>

		<canvas id="dolus-chart" width=1350 height=700 on:mousemove={onMoved(selected)} />

	</div>

</div>

<style>
	.container {
		display: flex;
		align-items: flex-start;
		flex-wrap: wrap;
		justify-content: space-between;
	}

	.wordListItem {
		cursor: pointer;
	}

	canvas {
	}
</style>
