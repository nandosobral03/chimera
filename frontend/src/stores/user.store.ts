import { writable } from "svelte/store";
import { browser } from "$app/environment";
const createUserStore = () => {
    let user = undefined;
    const { subscribe, set, update } = writable<string | undefined>(user);
    if(!browser) return {
        subscribe,
        set,
    }
    let localUser = localStorage.getItem("user");
    if (localUser) {
        user = localUser;
    }
    return {
        subscribe,
        set: (newUser: string | undefined) => {
            if (newUser === undefined) {
                localStorage.removeItem("user");
            }else{
                localStorage.setItem("user", newUser);
            }
            set(newUser);
            user = newUser;
        },
    
    };
};

export const user = createUserStore();

export default { user };