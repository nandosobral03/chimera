import { browser } from "$app/environment"
import { PUBLIC_BACKEND_URL } from "$env/static/public"
import axios from "axios";
export let getCurrentGuestGame = async () => {
    if(!browser) return null;

    try{
        const url = `${PUBLIC_BACKEND_URL}/guest/current`;
        const response = await axios.get(url, {
            headers: {
                guest_id: localStorage.getItem("guest_id")
            }
        });
        return response.data;
    }
    catch(e: any){
        if(e.response.status == 404){
            return null;
        }
    }


}



export let postGameResultGuest = async (result: GameResult) => {
        const url = `${PUBLIC_BACKEND_URL}/game/guest`;
        let body = {
            uncovered: result.moves.map(move => {
                return `${move.x}:${move.y}`
            }).join(","),
            exploded: result.won ? undefined : `${result.moves[result.moves.length - 1].x}:${result.moves[result.moves.length - 1].y}`,
            flags: result.flags.map(flag => {
                return `${flag.x}:${flag.y}`
            }).join(","),
            time_taken: result.timeTaken,
        }
        const response = await axios.post(url,
            body,
            {headers: {
                guest_id: localStorage.getItem("guest_id")
            }}
        );
        return response.data;
}

type GuestStats = {
    id : number,
    total_games : number,
    total_wins : number,
}


export let getGuestStats: () => Promise<GuestStats> = async () => {
        const url = `${PUBLIC_BACKEND_URL}/guest/stats`;
        const response = await axios.get(url, {
            headers: {
                guest_id: localStorage.getItem("guest_id")
            }
        });
        return response.data;
}


export default {getCurrentGuestGame, postGameResultGuest, getGuestStats}