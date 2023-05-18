import { PUBLIC_BACKEND_URL } from "$env/static/public"
import axios from "axios";
export let getTimeForNextGame = async () => {
        const url = `${PUBLIC_BACKEND_URL}/game/next`;
        const response = await axios.get(url);
        return response.data;
}
