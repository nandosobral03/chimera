<script lang="ts">
	import { createEventDispatcher, onMount } from "svelte";
    export let board: boolean[][];
    export let initialPosition: string;

    let dispatch = createEventDispatcher();
	let moves:{x:number, y:number}[] = [];

	let lastClicked = { x: -1, y: -1 };
	const numberedBoard = JSON.parse(JSON.stringify(board));
	const flagMask = Array.from({ length: board.length }, () =>
		Array.from({ length: board[0].length }, () => false)
	);
	let gameStatus = 0;
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

	onMount(() => {
		for (let i = 0; i < board.length; i++) {
			for (let j = 0; j < board[i].length; j++) {
				if (board[i][j]) {
					numberedBoard[i][j] = {
						shown: false,
						value: -1
					};
				} else {
					numberedBoard[i][j] = {
						shown: false,
						value: countNeighbours(i, j)
					};
				}
			}
		}

		let [initialX, initialY] = initialPosition.split(":").map((x:string) => parseInt(x));
		discover(initialX, initialY);

	});

	const discover = (x: number, y: number, userEmitted=false) => {
		numberedBoard[x][y].shown = true;
		if (userEmitted) moves.push({x, y});
		if (
			numberedBoard.every(
				(
					row: [
						{
							shown: boolean;
							value: number;
						}
					]
				) => row.every((cell) => cell.shown || cell.value == -1)
			)
		) {
			dispatch("gameover", { win: true, moves });
			return;
		}

		if (numberedBoard[x][y].value == 0) {
			if (numberedBoard[x] && numberedBoard[x][y - 1] && !numberedBoard[x][y - 1].shown)
				discover(x, y - 1);
			if (numberedBoard[x] && numberedBoard[x][y + 1] && !numberedBoard[x][y + 1].shown)
				discover(x, y + 1);
			if (numberedBoard[x - 1] && numberedBoard[x - 1][y] && !numberedBoard[x - 1][y].shown)
				discover(x - 1, y);
			if (numberedBoard[x + 1] && numberedBoard[x + 1][y] && !numberedBoard[x + 1][y].shown)
				discover(x + 1, y);
			if (numberedBoard[x - 1] && numberedBoard[x - 1][y - 1] && !numberedBoard[x - 1][y - 1].shown)
				discover(x - 1, y - 1);
			if (numberedBoard[x + 1] && numberedBoard[x + 1][y + 1] && !numberedBoard[x + 1][y + 1].shown)
				discover(x + 1, y + 1);
			if (numberedBoard[x - 1] && numberedBoard[x - 1][y + 1] && !numberedBoard[x - 1][y + 1].shown)
				discover(x - 1, y + 1);
			if (numberedBoard[x + 1] && numberedBoard[x + 1][y - 1] && !numberedBoard[x + 1][y - 1].shown)
				discover(x + 1, y - 1);
			return;
		}
		if (numberedBoard[x][y].value == -1) {
			gameStatus = -1;
            lastClicked = {x, y};
			dispatch("gameover", { win: false, moves, last_move: {x, y} });
			for (let i = 0; i < numberedBoard.length; i++) {
				for (let j = 0; j < numberedBoard[i].length; j++) {
					if (numberedBoard[i][j].value == -1) {
						numberedBoard[i][j].shown = true;
					}
				}
			}
			return;
		}
	};

	const countSurroundingFlags = (x: number, y: number) => {
		let count = 0;
		for (let i = -1; i < 2; i++) {
			for (let j = -1; j < 2; j++) {
				if (i == 0 && j == 0) continue;
				if (numberedBoard[x + i] && numberedBoard[x + i][y + j] && flagMask[x + i][y + j]) {
					count++;
				}
			}
		}
		return count;
	};

	const showSurronding = (x: number, y: number) => {
		let surroundingFlags = countSurroundingFlags(x, y);
		const cell = numberedBoard[x][y];
		if (surroundingFlags == cell.value) {
			for (let i = -1; i < 2; i++) {
				for (let j = -1; j < 2; j++) {
					if (numberedBoard[x + i] && numberedBoard[x + i][y + j] && !flagMask[x + i][y + j]) {
						discover(x + i, y + j);
					}
				}
			}
		}
	};

	const flagCell = (x: number, y: number) => {
		flagMask[x][y] = !flagMask[x][y];
        if(flagMask[x][y]) {
            dispatch("flag", {x, y});
        } else {
            dispatch("unflag", {x, y});
        }
	};

	const gameStatusWrapper = (fun: (x: number, y: number, b?:boolean) => void, x: number, y: number) => {
		if (gameStatus == 0) {
			fun(x, y, true);
		}
	};
</script>


<div class="board" aria-hidden="true">
    {#each numberedBoard as row, i}
        {#each row as cell, j}
            {#if cell.shown}
                <div
                    class={cell.value == -1 ? 'bomb' : `cell-${cell.value}`}
                    class:mine={cell.value == -1}
                    class:cell={cell.value != -1}
                    on:click={() => gameStatusWrapper(showSurronding, i, j)}
                    on:keydown={(e) => {}}
                    on:contextmenu={(e) => {
                        e.preventDefault();
                    }}
                >
                    {#if i == lastClicked.x && j == lastClicked.y}
                        <svg xmlns="http://www.w3.org/2000/svg" data-name="Layer 1" viewBox="0 0 64 64" id="firework"><path d="M32,57a.806.806,0,0,1-.085,0A1,1,0,0,1,31,56c0-.052-.078-5.481-4.515-8.143-4.369-2.622-10.839,1.911-10.9,1.957a1,1,0,0,1-1.287-1.521c.037-.038,3.85-3.96,2.6-8.979C15.65,34.366,7.912,33,7.834,32.986A1,1,0,0,1,8,31H8c.055,0,5.483-.079,8.146-4.515,2.624-4.373-1.912-10.839-1.957-10.9a1,1,0,0,1,1.521-1.288c.037.037,3.958,3.85,8.98,2.6C29.635,15.651,31,7.912,31.014,7.834A1,1,0,0,1,33,8c0,.052.078,5.481,4.515,8.143,4.372,2.625,10.839-1.911,10.905-1.957a1,1,0,0,1,1.287,1.521c-.037.038-3.85,3.96-2.595,8.979,1.238,4.948,8.976,6.314,9.054,6.328A1,1,0,0,1,56,33c-.052,0-5.48.079-8.143,4.515-2.624,4.373,1.912,10.839,1.957,10.9a1,1,0,0,1-1.521,1.288c-.038-.037-3.961-3.852-8.98-2.595-4.948,1.237-6.314,8.976-6.327,9.054A1,1,0,0,1,32,57ZM23.326,45.038a8.013,8.013,0,0,1,4.189,1.105A11.007,11.007,0,0,1,32.187,51.9c1.1-2.663,3.12-5.846,6.642-6.726a11.014,11.014,0,0,1,7.39.781c-1.113-2.669-1.944-6.355-.076-9.468A11.008,11.008,0,0,1,51.9,31.813c-2.663-1.1-5.845-3.119-6.725-6.642a11,11,0,0,1,.78-7.39c-2.669,1.113-6.354,1.944-9.468.076A11.007,11.007,0,0,1,31.813,12.1c-1.1,2.663-3.12,5.846-6.642,6.726a11,11,0,0,1-7.39-.781c1.113,2.669,1.944,6.355.076,9.468A11.008,11.008,0,0,1,12.1,32.187c2.663,1.1,5.845,3.119,6.725,6.642a11,11,0,0,1-.78,7.39A14.038,14.038,0,0,1,23.326,45.038Z"></path><path d="M32 43q-.04 0-.081 0A1 1 0 0131 42a3.792 3.792 0 00-1.573-2.966c-1.223-.733-3.237.344-3.846.78a1 1 0 01-1.3-1.506 3.581 3.581 0 00.929-3.154c-.344-1.375-2.549-2.03-3.369-2.167A1 1 0 0122 31a3.789 3.789 0 002.971-1.573c.73-1.218-.317-3.19-.781-3.848a1 1 0 011.522-1.286 3.584 3.584 0 003.14.914c1.379-.345 2.043-2.6 2.166-3.367A1 1 0 0133 22a3.792 3.792 0 001.573 2.966c1.221.732 3.237-.344 3.846-.78a1 1 0 011.288 1.521 3.592 3.592 0 00-.914 3.139c.344 1.375 2.549 2.03 3.369 2.167A1 1 0 0142 33a3.8 3.8 0 00-2.966 1.573c-.73 1.218.317 3.19.781 3.848a1 1 0 01-1.522 1.286 3.588 3.588 0 00-3.14-.914c-1.379.345-2.043 2.6-2.166 3.367A1 1 0 0132 43zm-3.557-6.215a3.846 3.846 0 012.012.534 4.859 4.859 0 011.633 1.611 4.292 4.292 0 012.58-2.077 4.941 4.941 0 012.271-.032 4.324 4.324 0 01.379-3.277 4.877 4.877 0 011.611-1.633 4.29 4.29 0 01-2.076-2.58 4.946 4.946 0 01-.033-2.272 4.316 4.316 0 01-3.277-.378 4.859 4.859 0 01-1.633-1.611 4.292 4.292 0 01-2.58 2.077 4.941 4.941 0 01-2.271.032 4.324 4.324 0 01-.379 3.277 4.877 4.877 0 01-1.611 1.633 4.29 4.29 0 012.076 2.58 4.958 4.958 0 01.035 2.271A5.528 5.528 0 0128.444 36.785zM23.581 12.675a1 1 0 01-.924-.617l-1.913-4.62a1 1 0 111.848-.766l1.913 4.62a1 1 0 01-.541 1.307A1.014 1.014 0 0123.581 12.675zM11.675 24.582a1 1 0 01-.383-.077L6.673 22.591a1 1 0 01.765-1.847l4.62 1.913a1 1 0 01-.383 1.925zM7.056 43.333a1 1 0 01-.383-1.924l4.619-1.914a1 1 0 01.766 1.848l-4.62 1.913A.991.991 0 017.056 43.333zM21.668 57.945a1.014 1.014 0 01-.383-.076 1 1 0 01-.541-1.307l1.913-4.62a1 1 0 011.848.766l-1.913 4.62A1 1 0 0121.668 57.945zM42.332 57.945a1 1 0 01-.924-.617l-1.913-4.62a1 1 0 011.848-.766l1.913 4.62a1 1 0 01-.541 1.307A1.014 1.014 0 0142.332 57.945zM56.944 43.333a.991.991 0 01-.382-.077l-4.62-1.913a1 1 0 11.766-1.848l4.619 1.914a1 1 0 01-.383 1.924zM52.325 24.582a1 1 0 01-.383-1.925l4.62-1.913a1 1 0 01.765 1.847l-4.619 1.914A1 1 0 0152.325 24.582zM40.419 12.675a1.014 1.014 0 01-.383-.076 1 1 0 01-.541-1.307l1.913-4.62a1 1 0 011.848.766l-1.913 4.62A1 1 0 0140.419 12.675zM32 4a1 1 0 01-1-1V1a1 1 0 012 0V3A1 1 0 0132 4zM11.494 12.494a1 1 0 01-.707-.293L9.373 10.787a1 1 0 011.414-1.414L12.2 10.787a1 1 0 01-.707 1.707zM3 33H1a1 1 0 010-2H3a1 1 0 010 2zM10.08 54.92a1 1 0 01-.707-1.707L10.787 51.8A1 1 0 0112.2 53.213l-1.414 1.414A.993.993 0 0110.08 54.92zM32 64a1 1 0 01-1-1V61a1 1 0 012 0v2A1 1 0 0132 64zM53.92 54.92a.993.993 0 01-.707-.293L51.8 53.213A1 1 0 0153.213 51.8l1.414 1.414a1 1 0 01-.707 1.707zM63 33H61a1 1 0 010-2h2a1 1 0 010 2zM52.506 12.494a1 1 0 01-.707-1.707l1.414-1.414a1 1 0 011.414 1.414L53.213 12.2A1 1 0 0152.506 12.494z"></path></svg>
                    {:else}
                        {#if cell.value != -1 && cell.value != 0}
                            {cell.value}
                        {/if}
                        {#if cell.value == -1}
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"
                                ><path
                                    d="M459.1 52.4L442.6 6.5C440.7 2.6 436.5 0 432.1 0s-8.5 2.6-10.4 6.5L405.2 52.4l-46 16.8c-4.3 1.6-7.3 5.9-7.2 10.4c0 4.5 3 8.7 7.2 10.2l45.7 16.8 16.8 45.8c1.5 4.4 5.8 7.5 10.4 7.5s8.9-3.1 10.4-7.5l16.5-45.8 45.7-16.8c4.2-1.5 7.2-5.7 7.2-10.2c0-4.6-3-8.9-7.2-10.4L459.1 52.4zm-132.4 53c-12.5-12.5-32.8-12.5-45.3 0l-2.9 2.9C256.5 100.3 232.7 96 208 96C93.1 96 0 189.1 0 304S93.1 512 208 512s208-93.1 208-208c0-24.7-4.3-48.5-12.2-70.5l2.9-2.9c12.5-12.5 12.5-32.8 0-45.3l-80-80zM200 192c-57.4 0-104 46.6-104 104v8c0 8.8-7.2 16-16 16s-16-7.2-16-16v-8c0-75.1 60.9-136 136-136h8c8.8 0 16 7.2 16 16s-7.2 16-16 16h-8z"
                                /></svg
                            >
                        {/if}
                    {/if}
                </div>
            {:else if flagMask[i][j]}
                <div
                    class="cell"
                    on:contextmenu={(e) => {
                        e.preventDefault();
                        gameStatusWrapper(flagCell, i, j);
                    }}
                >
                    🚩
                </div>
            {:else}
                <div
                    class="hidden-cell"
                    on:click={() => gameStatusWrapper(discover, i, j)}
                    on:contextmenu={(e) => {
                        e.preventDefault();
                        gameStatusWrapper(flagCell, i, j);
                    }}
                    on:keydown={(e) => {}}
                />
            {/if}
        {/each}
    {/each}
</div>

<style lang="scss">


.board {
    display: grid;
    gap: 1px;
    grid-template-columns: repeat(30, 1fr);
    grid-template-rows: repeat(16, 1fr);
}
.cell {
    display: grid;
    place-items: center;
    background-color: lightgrey;
    box-shadow: 1px 1px 1px 0px black;
    cursor: pointer;
}

.bomb {
    display: grid;
    place-items: center;
    background-color: var(--danger);
    box-shadow: 1px 1px 1px 0px black;
    cursor: pointer;
    svg {
        scale: 0.5;
        fill: var(--warning);
    }
}


.hidden-cell {
    width: 32px;
    height: 32px;
    background-color: grey;
    box-shadow: 1px 1px 1px 0px black;
    cursor: pointer;
    &:hover {
        background-color: lightgrey;
    }
}

*{
    user-select: none;
}
</style>
