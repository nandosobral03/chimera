import { browser } from "$app/environment"
import { PUBLIC_BACKEND_URL } from "$env/static/public"
import axios from "axios";
export let login = async ( username: string, password: string) => {
    if (!browser) return;
    const url = `${PUBLIC_BACKEND_URL}/login`;
    const response = await axios.post(url, {
        username,
        password});
    let token = response.data.token;
    localStorage.setItem("token", token);
    return response.data;
}


export let signup = async (username: string, password:string) => {
    if (!browser) return;
    
    const url = `${PUBLIC_BACKEND_URL}/signup`;
    const response = await axios.post(url, {
        username,
        password
    });
    let token = response.data.token;
    localStorage.setItem("token", token);
    return response.data;
}

export let getCurrentUserGame = async () => {
    if(!browser) return null;
    const token = localStorage.getItem("token");
    if(!token) return null;
    try{
        const url = `${PUBLIC_BACKEND_URL}/user/current`;
        const response = await axios.get(url, {
            headers: {
                Authorization: `Bearer ${token}`
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

export let postGameResultUser = async (result: GameResult) => {
        if(!browser) return null;
        const token = localStorage.getItem("token");
        if(!token) return null;
        const url = `${PUBLIC_BACKEND_URL}/game`;
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
                Authorization: `Bearer ${token}`
            }}
        );
        return response.data;
}

type UserStats = {
    user_id : number,
    total_games : number,
    total_wins : number,
    win_streak : number,
}    


export let getUserStats: () => Promise<UserStats> = async () => {
        if(!browser) return null;
        const token = localStorage.getItem("token");
        if(!token) return null;
        const url = `${PUBLIC_BACKEND_URL}/user/stats`;
        const response = await axios.get(url, {
            headers: {
                Authorization: `Bearer ${token}`
            }
        });
        return response.data;
}

