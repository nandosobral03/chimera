<script lang="ts">
	import { onMount } from 'svelte';
	import type { PageServerData } from './$types';
	export let data: PageServerData;
	const board = data.board;
	const numberedBoard = JSON.parse(JSON.stringify(board));
	const flagMask = Array.from({ length: board.length }, () => Array.from({ length: board[0].length }, () => false));
    let gameStatus = 0;
    
    const countNeighbours = (x: number, y: number) => {
		let count = 0;
		for (let i = -1; i < 2; i++) {
			for (let j = -1; j < 2; j++) {
				if (i == 0 && j == 0) continue;
				if (board[x+i] && board[x+i][y+j]) count++;
			}
		}
		return count;
	};

	onMount(() => {
        console.log(board);
		for (let i = 0; i < board.length; i++) {
			for (let j = 0; j < board[i].length; j++) {
                if(board[i][j]){
                    numberedBoard[i][j] = {
                        shown: false,
                        value: -1
                    }
                }else{
                    numberedBoard[i][j] = {
                    shown: false,
                    value: countNeighbours(i, j)
                    }
                }
			}
		}
        console.log(numberedBoard);
	});

    const discover = (x:number, y:number) => {
        numberedBoard[x][y].shown = true;
        if (numberedBoard.every((row:[{
            shown: boolean,
            value: number
        }]) => row.every(cell => cell.shown || cell.value == -1))) {
            gameStatus = 1;
            return
        }

        if(numberedBoard[x][y].value == 0){
            if(numberedBoard[x] && numberedBoard[x][y-1] && !numberedBoard[x][y-1].shown) discover(x, y-1)
            if(numberedBoard[x] && numberedBoard[x][y+1] && !numberedBoard[x][y+1].shown) discover(x, y+1)
            if(numberedBoard[x-1] && numberedBoard[x-1][y] && !numberedBoard[x-1][y].shown) discover(x-1, y)
            if(numberedBoard[x+1] && numberedBoard[x+1][y] && !numberedBoard[x+1][y].shown) discover(x+1, y)
            if(numberedBoard[x-1] && numberedBoard[x-1][y-1] && !numberedBoard[x-1][y-1].shown) discover(x-1, y-1)
            if(numberedBoard[x+1] && numberedBoard[x+1][y+1] && !numberedBoard[x+1][y+1].shown) discover(x+1, y+1)
            if(numberedBoard[x-1] && numberedBoard[x-1][y+1] && !numberedBoard[x-1][y+1].shown) discover(x-1, y+1)
            if(numberedBoard[x+1] && numberedBoard[x+1][y-1] && !numberedBoard[x+1][y-1].shown) discover(x+1, y-1);
            return;
        }
        if(numberedBoard[x][y].value == -1){
            gameStatus = -1;
            for(let i = 0; i < numberedBoard.length; i++){
                for(let j = 0; j < numberedBoard[i].length; j++){
                    if(numberedBoard[i][j].value == -1){
                        numberedBoard[i][j].shown = true;
                    }
                }
            }
            return
        }
        
        
    }

    const countSurroundingFlags = (x: number, y: number) => {
        let count = 0;
        for(let i = -1; i < 2; i++){
            for(let j = -1; j < 2; j++){
                if(i == 0 && j == 0) continue;
                if(numberedBoard[x+i] && numberedBoard[x+i][y+j] && flagMask[x+i][y+j]){
                    count++;
                }

            }
        }
        return count;
    }


    const showSurronding = (x: number, y :number) => {
        let surroundingFlags = countSurroundingFlags(x, y);
        const cell = numberedBoard[x][y];
        console.log(surroundingFlags)
        if(surroundingFlags == cell.value){
            for(let i = -1; i < 2; i++){
                for(let j = -1; j < 2; j++){
                    if(numberedBoard[x+i] && numberedBoard[x+i][y+j] && !flagMask[x+i][y+j]){
                        discover(x+i, y+j);
                    }
                }         
            } 
            
        }
    }

    const flagCell = (x:number, y:number) => {
        flagMask[x][y] = !flagMask[x][y];
        
    }


    const gameStatusWrapper = (fun: ((x:number, y:number) => void), x : number, y: number) => {
        if(gameStatus == 0){
            fun(x, y);
        }
    }

</script>

<section>
	<div class="board">
		{#each numberedBoard as row, i}
			{#each row as cell, j}
				{#if cell.shown}
                    <div class="cell" 
                        on:click={() => gameStatusWrapper(showSurronding, i, j)}
                       
                        >{cell.value}</div>
               {:else if flagMask[i][j]}
                    <div class="cell" on:contextmenu={(e) =>  {
                        e.preventDefault();
                        gameStatusWrapper(flagCell, i, j)
                    }}>🚩</div>
                {:else}
                    <div class="hidden-cell" on:click={() =>  gameStatusWrapper(discover, i, j)}
                     on:contextmenu={(e) => {
                            e.preventDefault();
                            gameStatusWrapper(flagCell, i, j)}}>
                        </div>
                {/if}

			{/each}
		{/each}
	</div>
</section>

<style lang="scss">
	section {
		height: 100%;
		width: 100%;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
	}

	.board {
		display: grid;
		grid-template-columns: repeat(30, 1fr);
		grid-template-rows: repeat(16, 1fr);
	}
    .cell{

    }

    .hidden-cell{
        width: 32px;
        height: 32px;
        background-color: grey;
        border: 1px solid white;
        &:hover{
            background-color: lightgrey;
        }
    }
</style>
