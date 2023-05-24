import { PUBLIC_BACKEND_URL } from "$env/static/public"
import axios from "axios";


export let getAlltimeLeaderboard = async () => {
    const url = `${PUBLIC_BACKEND_URL}/game/leaderboard`;
    const response = await axios.get(url);
    return response.data;
}

export let getCurrentLeaderboard = async () => {
    const url = `${PUBLIC_BACKEND_URL}/game/leaderboard/current`;
    const response = await axios.get(url);
    return response.data;
}

export let getDailyLeaderboard = async (day: string) => {
    const url = `${PUBLIC_BACKEND_URL}/game/leaderboard/daily`;
    const params = {
        day: day
    }
    const response = await axios.get(url, { params: params });
    return response.data;
}