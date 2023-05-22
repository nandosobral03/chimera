type GameResult = {
    won: boolean;
    moves: {x: number, y: number}[];
    flags: {x: number, y: number}[];
    timeTaken: number;
}

type Game = {
    board: boolean[][];
    initialPosition: {x: number, y: number};
}

type DayStats = {
    day: string;
    total_games: number;
    total_wins: number;
    aggregated_board_stats: {
        [key: string]: number;
    }
}