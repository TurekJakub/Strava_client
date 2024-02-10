type DishInfo = {
	id: string;
	allergens: string[];
	order_state: boolean;
};
type LoginResponse = {
	message: string;
	user: string;
};
type MenuResponse = { 
    menu: Menu
};
type ErrorResponse = {
	message: string;
};
type Success<T> = {
	_t: 'success';
	data: T;
};
type Failure<T> = {
	_t: 'failure';
	error: T;
};
type Result<T,R> = Success<T> | Failure<R>;
type Menu = {
	[key: string]: DailyMenu;
};
type DailyMenu = {
	[key: string]: DishInfo;
};
type MenuData = { [key: string]: DailyMenu }
declare module 'TauriComunicationLayer' {
	export function login(
		username: string,
		password: string,
		cantine: number,
		stayLogged: boolean
	): Promise<string>;
}
declare module 'WebComunicationLayer' {
	export function login(
		username: string,
		password: string,
		cantine: number,
		stayLogged: boolean
	): Promise<Result>;
	export function getUserMenu(): Promise<MenuData>;
}
