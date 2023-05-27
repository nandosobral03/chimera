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

