import { writable } from "svelte/store";

type loginStoreType = {
    isOpen: boolean;
    allowClose: boolean;
}

const loginStore = writable<loginStoreType | undefined>(undefined);
export default loginStore;