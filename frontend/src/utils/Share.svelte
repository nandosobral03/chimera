<script lang='ts'>
	import { createEventDispatcher, onMount } from "svelte";
	import PrimaryButton from "./PrimaryButton.svelte";

    export let result : GameResult;
    export let game : Game;
    const dispatch = createEventDispatcher();


    let canvas : HTMLCanvasElement;
    let ctx : CanvasRenderingContext2D | null = null;
    let width = 340;
    let height = 200;
    let pointSize = 7;
    let lineWidth = 2;

    type Point = {
        x : number;
        y : number;
        color: string;
    }

    let pointsArray : Point[] = [];
    let colors = [
        "#577590",
        "#4d908e",
        "#43aa8b",
        "#90be6d",
        "#f9c74f",
        "#f8961e",
        "#f3722c",
        "#f94144"
    ];

    const drawPoint = (x : number, y : number, ctx : CanvasRenderingContext2D, pointColor: string) => {
        ctx.beginPath();
        ctx.arc(x, y, pointSize, 0, 2 * Math.PI);
        ctx.fillStyle = pointColor;
        ctx.fill();
    }

    const drawLine = (x1 : number, y1 : number, x2 : number, y2 : number, ctx : CanvasRenderingContext2D) => {
        ctx.beginPath();
        ctx.moveTo(x1, y1);
        ctx.lineTo(x2, y2);
        ctx.strokeStyle = '#78767a50';
        ctx.lineWidth = lineWidth;
        ctx.stroke();
    }

    const draw = (ctx : CanvasRenderingContext2D) => {
        let start = pointsArray[0];
        let end = pointsArray[pointsArray.length - 1];
        let points = pointsArray.slice(1, pointsArray.length - 1);
        let allPoints = [start, ...points, end];
        for(let i = 0; i < allPoints.length -1; i++){
            drawLine(allPoints[i].x, allPoints[i].y, allPoints[i+1].x, allPoints[i+1].y, ctx);
        }

        pointsArray.forEach((point)=>{
            drawPoint(point.x, point.y, ctx, point.color);
        });
    }

    const copyToClipBoard = () => {
        canvas.toBlob((blob:any)=>{
            let item = new ClipboardItem({ 'image/png': blob });
            navigator.clipboard.write([item]);
        });
    }


    onMount(()=>{
        if(canvas){
            ctx = canvas.getContext('2d');
            if(ctx){
                canvas.width = width;
                canvas.height = height;
                pointsArray.push({
                    x : (game.initialPosition.y * 10) + 20,
                    y : (game.initialPosition.x * 10) + 20,
                    color: "#433241"
                });
                let last = result.moves[result.moves.length - 1];
                pointsArray.push({
                    x : (last.y * 10) + 20,
                    y : (last.x * 10) + 20,
                    color: result.won ? colors[3] : colors[7]
                });
                result.moves.pop();
                
                let moves = result.moves;
                moves = moves.filter((move, i)=>{
                    return i % Math.floor(moves.length / 5) == 0;
                });

                let flags = result.flags;
                flags = flags.filter((flag, i)=>{
                    return i % Math.floor(flags.length / 5) == 0;
                });
                moves.forEach((move, i)=>{
                    pointsArray.push({
                        x : (move.y * 10) + 20,
                        y : (move.x * 10) + 20,
                        color: colors[1]
                    });
                });

                flags.forEach((flag, i)=>{
                    pointsArray.push({
                        x : (flag.y * 10) + 20,
                        y : (flag.x * 10) + 20,
                        color: colors[5]
                    });
                });
                draw(ctx);
            }
        }
    });


</script>

<section on:click={()=>{ dispatch('close'); }} on:keydown|preventDefault={(e)=>{ if(e.key == 'Escape') dispatch('close'); }}>
    <div on:click={(e) => { e.stopPropagation(); }} on:keydown|preventDefault={(e)=>{ e.stopPropagation(); }}>
        <canvas bind:this={canvas} width={width} height={height} />
            <PrimaryButton on:click={copyToClipBoard}>Copy to clipboard</PrimaryButton>
    </div>
</section>

<style lang='scss'>
    section{
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        display: grid;
        place-items: center;
        background-color: rgba(0,0,0,0.5);

    }

    div{
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 16px;
        background-color: var(--shade);
        padding: 32px;
        border-radius: 16px;
    }

 
    canvas{
        border-radius: 5px;
        box-shadow: 0 0 5px rgba(0,0,0,0.5);
        background-color: #f5f5f5;
    }
</style>