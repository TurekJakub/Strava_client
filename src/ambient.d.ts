type DishInfo = {
    id: string;
    allergens: string[];
    order_state: boolean;
};
type Menu = {
    [key: string]: DailyMenu;
}
type DailyMenu = {
    [key: string]: DishInfo;

}
type MenuData = [string[], Menu];
declare module 'TauriComunicationLayer' {
  export function login(username: string, password: string, cantine: number, stayLogged: boolean): Promise<string>;
}
declare module 'WebComunicationLayer' {
    export function login(username: string, password: string, cantine: number, stayLogged: boolean): Promise<string>;
    export function getUserMenu(): Promise<MenuData>;
}