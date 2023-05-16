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

export default {getCurrentGuestGame};