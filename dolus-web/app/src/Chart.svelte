<script>
	let selected;
	let wordList = [];
	let loading = false;
	let mouseX = null;
	let chartValues = {};
	let isDrawing = false;

	import {loadWords, draw, closestValues} from "./charts.js";

	$: chartCalc = (() => {
		isDrawing = true;
		try {
			draw({chartId:"dolus-chart", word:selected, mouseX});
		}catch (err){
			console.error("Failed to draw", err);
		}
		isDrawing = false;
	})();

	loadWords()
		.then(w => {
			let c = document.getElementById("dolus-chart");
			c.width  =  window.innerWidth  / 2;
			c.height =  window.innerHeight / 2;

			wordList = w.sort();
			selected = wordList[0]
		});

	const onSelect = (w) => () => {
		selected = w;
	};

	const onMoved = (w) => (event) => {
		if (isDrawing) {
			return;
		}
		let actualRect = event.target.getBoundingClientRect();
		let logicX = event.offsetX / actualRect.width;
		mouseX = logicX;
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

		<div>
		<h2> {selected || ""} </h2>

		<table>
			<thead>
				<tr>
					<td>
						URL
					</td>
					<td>
						Date-time
					</td>
					<td>
						Count
					</td>
				</tr>
			</thead>
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
		</div>

		<div class="data-explorer">

			<canvas id="dolus-chart" on:mousemove={onMoved(selected)} />

		</div>
	</div>

</div>

<style>
	.container {
		display: flex;
		align-items: flex-start;
		flex-wrap: wrap;
	}

	.wordListItem {
		cursor: pointer;
	}

	.data-explorer {
		flex-grow: 1;
		width: 66vw;
		height: 50vh;
		padding-left: 20px;
	}

	table { border-collapse: collapse; }
	tr { border: none; }
	td {
	  border-right: solid 1px #f00; 
	  border-left: solid 1px #f00;
	}
</style>
