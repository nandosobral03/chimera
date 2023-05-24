<script lang='ts'>
    export let leaderboard: AlltimeLeaderboard;

</script>
<article>
    <div class="entry header">
        <div class="rank">Rank</div>
        <div class="name">Username</div>
        <div class="score">Winstreak</div>
        <div class="score">Wins</div>
        <div class="score">Games</div>

    </div>
    {#each leaderboard.users as entry, i}
    <div class="entry">
        <div class="rank">{i+1}</div>
        <div class="name" title={entry.username}>
            {entry.username}
        </div>
        <div class="score">{entry.win_streak}</div>
        <div class="score">{entry.total_wins}</div>
        <div class="score">{entry.total_games}</div>
    </div>
    {/each}
</article>


<style lang='scss'>
  
    .entry{
        display: grid;
        grid-template-columns: 1fr 2fr 1fr 1fr 1fr;
        gap: 16px;
        min-width:fit-content;
        text-align: center;
        &:nth-child(even){
            background-color: var(--background-clear)
        }

        div{
            flex-grow: 1;
        }
        padding: 12px;
        .rank{
            font-weight: bold;
            width: 100%;
            min-width: 50px;
            display: flex;
            height: 100%;
            align-items: center;
            justify-content: flex-start;
        }
        .name{
            width: 100%;
            min-width: 150px;
            overflow: hidden;
            text-overflow: ellipsis;
            display: flex;
            height: 100%;
            align-items: center;
            justify-content: flex-start;
        }
        
        .score{
            width: 100%;
            min-width: 100px;
            display: flex;
            height: 100%;
            align-items: center;
            justify-content: center;
        }
    }

    article{
        width: clamp(200px, 100%, 650px);
        display: flex;
        flex-direction: column;
        border-radius: 8px;
        border: 1px solid var(--shade);
        flex-grow: 1;
        overflow: auto;
        margin-top: 48px;
    }

    .header{
        background-color: var(--shade);
        color: var(--background);
        text-transform: uppercase;
        height: 4rem;
    }
</style>