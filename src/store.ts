import { writable } from "svelte/store";

const amount = localStorage.getItem("account") || "0";
export const account = writable(amount);
account.subscribe(value => {
    localStorage.setItem("account", value);
}); 