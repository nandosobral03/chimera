<script lang='ts'>
	import { onMount } from "svelte";

    export let result : GameResult;
    export let game : Game;

    //Generate the sharable canvas
    //Create a 30x16 canvas
    //Draw a point in initial position
    //Select 10 random points
    //Select points from game result
    //Select flags from game result
    //Draw lines between points


    let canvas : HTMLCanvasElement;
    let ctx : CanvasRenderingContext2D | null = null;
    let width = 320;
    let height = 180;
    let pointSize = 5;
    let pointColor = 'rgba(0,0,0,0.5)';
    let lineColor = 'rgba(0,0,0,0.5)';
    let lineWidth = 2;
    let lineDash = [5, 5];


    type Point = {
        x : number;
        y : number;
    }

    let pointsArray : Point[] = [];

    function drawPoint(x : number, y : number, ctx : CanvasRenderingContext2D){
        ctx.beginPath();
        ctx.arc(x, y, pointSize, 0, 2 * Math.PI);
        ctx.fillStyle = pointColor;
        ctx.fill();
    }

    function drawLine(x1 : number, y1 : number, x2 : number, y2 : number, ctx : CanvasRenderingContext2D){
        ctx.beginPath();
        ctx.moveTo(x1, y1);
        ctx.lineTo(x2, y2);
        ctx.strokeStyle = lineColor;
        ctx.lineWidth = lineWidth;
        ctx.setLineDash(lineDash);
        ctx.stroke();
    }

    function draw(ctx : CanvasRenderingContext2D){
        pointsArray.forEach((point)=>{
            drawPoint(point.x, point.y, ctx);
        });
        for(let i = 0; i < pointsArray.length; i++){
            for(let j = i + 1; j < pointsArray.length; j++){
                drawLine(pointsArray[i].x, pointsArray[i].y, pointsArray[j].x, pointsArray[j].y, ctx);
            }
        }
    }

    const shuffle = (array : any[]) => {
        let currentIndex = array.length,  randomIndex;

        // While there remain elements to shuffle...
        while (currentIndex != 0) {

            // Pick a remaining element...
            randomIndex = Math.floor(Math.random() * currentIndex);
            currentIndex--;

            // And swap it with the current element.
            [array[currentIndex], array[randomIndex]] = [
            array[randomIndex], array[currentIndex]];
        }

        return array;
    }

    onMount(()=>{
        if(canvas){
            ctx = canvas.getContext('2d');
           
            


            if(ctx){
                //Set canvas size
                canvas.width = width;
                canvas.height = height;

                //Draw initial point
                pointsArray.push({
                    x : game.initialPosition.y * 10,
                    y : game.initialPosition.x * 10
                });
                drawPoint(game.initialPosition.y * 10, game.initialPosition.x * 10, ctx);

                // Draw points from game result
                // Select 5 random moves
                // let moves = shuffle(result.moves).slice(0, 5);
                let moves = result.moves.slice(0, 5);
                moves.forEach((move)=>{
                    pointsArray.push({
                        y : move.x * 10,
                        x : move.y * 10
                    });
                });   

                // let flags = shuffle(result.flags).slice(0, 5);
                // flags.forEach((move)=>{
                //     console.log(move);
                //     pointsArray.push({
                //         y : move.x * 10,
                //         x : move.y * 10
                //     });
                // });   
              

                //Draw lines between points
                draw(ctx);
            }
        }
    });


</script>

<section>
    Share Result
    <!-- <pre>{JSON.stringify(result)}</pre> -->
    <!-- <pre>{JSON.stringify(game)}</pre> -->
    <canvas bind:this={canvas} width={width} height={height} />


</section>

<style lang='scss'>
    section{
        display: flex;
        flex-direction: column;
        align-items: center;
    }
    canvas{
        border-radius: 5px;
        box-shadow: 0 0 5px rgba(0,0,0,0.5);
    }
</style>