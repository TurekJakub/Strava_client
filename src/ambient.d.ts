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