import { PUBLIC_BACKEND_URL } from "$env/static/public"
import axios from "axios";


export let getDayStats: (day:string) => Promise<DayStats> = async (day:string) => {
        const url = `${PUBLIC_BACKEND_URL}/game/stats`;
        const response = await axios.get(url, {
            params: {
                day: day
            }
        });
        return response.data;
}
