<script lang="ts">
	export let board: boolean[][];
    export let initialPosition: {x: number, y: number}
	export let aggregatedBoardStats: {
		[key: string]: number;
	};
    export let total_losses: number;
	import { onDestroy, onMount } from 'svelte';
	let windowSize = { width: window.innerWidth, height: window.innerHeight };
	let verticalDisplay = window.innerWidth / window.innerHeight < 30 / 16;
	let cellSize = 0;

    const heatmapColors = ["#05050f", "#03071e","#370617","#6a040f","#9d0208","#d00000","#dc2f02","#f77f31","#ffa32b","#ebd87c"].reverse();

	function getCellSize(vertical = false): number {
		if (vertical) {
			let cellSize = 0.0213 * windowSize.height;
			cellSize = Math.max(cellSize, 12);
			cellSize = Math.min(cellSize, 32);
			return cellSize;
		}
		let cellSize = 0.0267 * windowSize.width;
		cellSize = Math.max(cellSize, 16);
		cellSize = Math.min(cellSize, 32);
		return cellSize;
	}

	const numberedBoard = Array.from({ length: board.length }, () => Array.from({ length: board[0].length }, () => ({ starter: false, value: -1 })));
	const countNeighbours = (x: number, y: number) => {
		let count = 0;
		for (let i = -1; i < 2; i++) {
			for (let j = -1; j < 2; j++) {
				if (i == 0 && j == 0) continue;
				if (board[x + i] && board[x + i][y + j]) count++;
			}
		}
		return count;
	};

	const calculateCellSize = () => {
		windowSize = { width: window.innerWidth, height: window.innerHeight };
		verticalDisplay = windowSize.width < windowSize.height;
		console.log('calculating cell size', verticalDisplay, windowSize);
		cellSize = getCellSize(verticalDisplay);
	};

    const discover = (x: number, y: number) => {
        numberedBoard[x][y].starter = true;
		if (numberedBoard[x][y].value == 0) {
			if (numberedBoard[x] && numberedBoard[x][y - 1] && !numberedBoard[x][y - 1].starter)
				discover(x, y - 1);
			if (numberedBoard[x] && numberedBoard[x][y + 1] && !numberedBoard[x][y + 1].starter)
				discover(x, y + 1);
			if (numberedBoard[x - 1] && numberedBoard[x - 1][y] && !numberedBoard[x - 1][y].starter)
				discover(x - 1, y);
			if (numberedBoard[x + 1] && numberedBoard[x + 1][y] && !numberedBoard[x + 1][y].starter)
				discover(x + 1, y);
			if (numberedBoard[x - 1] && numberedBoard[x - 1][y - 1] && !numberedBoard[x - 1][y - 1].starter)
				discover(x - 1, y - 1);
			if (numberedBoard[x + 1] && numberedBoard[x + 1][y + 1] && !numberedBoard[x + 1][y + 1].starter)
				discover(x + 1, y + 1);
			if (numberedBoard[x - 1] && numberedBoard[x - 1][y + 1] && !numberedBoard[x - 1][y + 1].starter)
				discover(x - 1, y + 1);
			if (numberedBoard[x + 1] && numberedBoard[x + 1][y - 1] && !numberedBoard[x + 1][y - 1].starter)
				discover(x + 1, y - 1);
			return;
		}
	};


	onDestroy(() => {
		window.removeEventListener('resize', calculateCellSize);
	});

	onMount(() => {
		window.addEventListener('resize', calculateCellSize);
		calculateCellSize();
		for (let i = 0; i < board.length; i++) {
			for (let j = 0; j < board[i].length; j++) {
				if (board[i][j]) {
					numberedBoard[i][j] = {
						starter: false,
						value: -1
					};
				} else {
					numberedBoard[i][j] = {
						starter: false,
						value: countNeighbours(i, j)
					};
				}
			}
		}
        discover(initialPosition.x, initialPosition.y);
	});

    const getBombColor =   (i: number, j: number) => {
        if(!aggregatedBoardStats[`${i}:${j}`]) return heatmapColors[0]
        const percentage = Math.floor(aggregatedBoardStats[`${i}:${j}`] / total_losses * 10) - 1;
        console.log(percentage)
        return heatmapColors[percentage]
    }

    const getBombColorText =   (i: number, j: number) => {
        
        if(!aggregatedBoardStats[`${i}:${j}`]) "var(--shade)"
        const percentage = Math.round(aggregatedBoardStats[`${i}:${j}`] / total_losses * 10);
        if(percentage > 5) return "white"
        return heatmapColors[percentage]
    }

</script>

<div class="board" class:vertical={verticalDisplay}>
	{#each numberedBoard as row, i}
		{#each row as cell, j}
			<div
				class={cell.value == -1 ? 'bomb' : `cell-${cell.value}`}
				class:cell={cell.value != -1}
                class:starter={cell.starter}
				on:contextmenu|preventDefault={() => {}}
				style={`width: ${cellSize}px; height: ${cellSize}px; ${cell.value == -1 ? "background-color:" + getBombColor(i, j) : ""}`}
			>
				{#if cell.value != -1 && cell.value != 0}
					{cell.value}
				{/if}
				{#if cell.value == -1}
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"
						style={`${cell.value == -1 ? "fill:" + getBombColorText(i, j) : ""}`}
                    ><path
                    d="M459.1 52.4L442.6 6.5C440.7 2.6 436.5 0 432.1 0s-8.5 2.6-10.4 6.5L405.2 52.4l-46 16.8c-4.3 1.6-7.3 5.9-7.2 10.4c0 4.5 3 8.7 7.2 10.2l45.7 16.8 16.8 45.8c1.5 4.4 5.8 7.5 10.4 7.5s8.9-3.1 10.4-7.5l16.5-45.8 45.7-16.8c4.2-1.5 7.2-5.7 7.2-10.2c0-4.6-3-8.9-7.2-10.4L459.1 52.4zm-132.4 53c-12.5-12.5-32.8-12.5-45.3 0l-2.9 2.9C256.5 100.3 232.7 96 208 96C93.1 96 0 189.1 0 304S93.1 512 208 512s208-93.1 208-208c0-24.7-4.3-48.5-12.2-70.5l2.9-2.9c12.5-12.5 12.5-32.8 0-45.3l-80-80zM200 192c-57.4 0-104 46.6-104 104v8c0 8.8-7.2 16-16 16s-16-7.2-16-16v-8c0-75.1 60.9-136 136-136h8c8.8 0 16 7.2 16 16s-7.2 16-16 16h-8z"
                    /></svg
                    >
				{/if}
			</div>
		{/each}
	{/each}
</div>

<style lang="scss">
	.board {
		display: grid;
		gap: 1px;
		grid-template-columns: repeat(30, 1fr);
		grid-template-rows: repeat(16, 1fr);
        width: fit-content;
		&.vertical {
            width: unset;
			rotate: 90deg;
			> div {
				transform: rotate(-90deg);
			}
		}
	}
	.cell {
		display: grid;
		place-items: center;
		background-color: lightgrey;
		box-shadow: 1px 1px 1px 0px black;
		font-size: clamp(0.6rem, 1.5vw, 1rem);
	}

	.bomb {
		display: grid;
		place-items: center;
		box-shadow: 1px 1px 1px 0px black;
		svg {
			scale: 0.5;
			fill: var(--shade);
		}
	}

	.hidden-cell {
		background-color: grey;
		box-shadow: 1px 1px 1px 0px black;
		cursor: pointer;
		&:hover {
			background-color: lightgrey;
		}
	}

    .starter{
        background-color: var(--starter)
    }

	* {
		user-select: none;
	}
</style>
