type GameResult = {
    won: boolean;
    moves: {x: number, y: number}[];
    flags: {x: number, y: number}[];
}

type Game = {
    board: boolean[][];
    initialPosition: {x: number, y: number};
}