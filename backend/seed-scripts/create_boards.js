const sqlite3 = require('sqlite3');
const dotenv = require('dotenv');
dotenv.config();
console.log(process.env.DATABASE_URL)
const db = new sqlite3.Database(process.env.DATABASE_URL);
const moment = require('moment');

const boards = [];
const start_date = moment('01/01/2023', 'DD/MM/YYYY');
const end_date = moment('01/01/2024', 'DD/MM/YYYY')
const days = end_date.diff(start_date, 'days');

const isNeighbour = (x1, y1, x2, y2) => {
    for(let i = -2; i <= 2; i+=1){
        for(let j = -2; j <= 2; j+=1){
            if(x1 + i === x2 && y1 + j === y2){
                return true;
            }
        }
    }
    return false;
}

function printBoard(mines, initial_position_x, initial_position_y){
    for(let i = 0; i < 16; i++){
        let row = "";
        for(let j = 0; j < 30; j++){
            if(mines.has(`${i}:${j}`)){
                row += "1";
            }else{
                if (i === initial_position_x && j === initial_position_y) row += "2";
                else row += "0";
            }
        }
        console.log(row);
    }
}


function main(){
    for(let i = 0; i < days; i++){
        const date = start_date.add(1, 'days').format('DD/MM/YYYY');
     
        let initial_position_x = Math.floor(Math.random() * 16);
        let initial_position_y = Math.floor(Math.random() * 30);
        const mines = new Set();
        for(let i = 0; i < 99; i++){
            let {x, y} = {x: Math.floor(Math.random() * 16), y: Math.floor(Math.random() * 30)};
            if(mines.has(`${x.toString().padStart(2,"0")}:${y.toString().padStart(2,"0")}`)){
                i--;
            }else{
                if(isNeighbour(initial_position_x, initial_position_y, x, y)){
                    i--;
                }else{
                mines.add(`${x.toString().padStart(2,"0")}:${y.toString().padStart(2,"0")}`);
                }
            }
        }
        const board = {
            day : date,
            initial_position: `${initial_position_x.toString().padStart(2,"0")}:${initial_position_y.toString().padStart(2,"0")}`,
            board: Array.from(mines).join(',')	
        }
        boards.push(board);
    }

    db.serialize(() => {
        db.run('CREATE TABLE IF NOT EXISTS games (id INTEGER PRIMARY KEY AUTOINCREMENT, day TEXT, initial_position TEXT, board TEXT)');
        const stmt = db.prepare('INSERT INTO games (day, initial_position, board) VALUES (?, ?, ?)');
        boards.forEach(board => {
            stmt.run(board.day, board.initial_position, board.board);
        });
        stmt.finalize();
    });
}

main();
