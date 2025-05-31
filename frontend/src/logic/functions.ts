import {BrowserOpenURL} from "../../wailsjs/runtime";

export function DarkAndThemeToConst(darkMode: boolean, themeMode: number): string {
    let co = [[
        "rgb(6, 66, 154)",
        "rgb(29, 78, 78)",
        "rgb(39, 100, 43)",
        "rgb(71, 107, 32)",
        "rgb(109, 79, 47)",
        "rgb(30, 35, 30)",
        "linear-gradient(to right, #8A4042, #72390E, #008F8F, #34557D, #621212, #59226C, #786F78)",
        "rgb(181, 86, 94)",
        "rgb(75, 35, 115)",
        "linear-gradient(to right, darkred, darkorange, darkgoldenrod, darkgreen, darkblue, darkorchid, darkviolet)",
        "rgb(139, 123, 22)",
        "rgb(165, 72, 48)",
        "rgb(154, 30, 26)",
        "rgb(38, 43, 133)"
    ], [
        "rgb(17, 111, 206)",
        "rgb(49, 148, 148)",
        "rgb(69, 160, 83)",
        "rgb(131, 157, 52)",
        "rgb(159, 129, 87)",
        "rgb(50, 55, 60)",
        "linear-gradient(to right, #FA8072, #D2691E, #00FFFF, #6495ED, #B22222, #9932CC, #D8BFD8)",
        "rgb(231, 116, 144)",
        "rgb(155, 65, 185)",
        "linear-gradient(to right, red, orange, yellow, green, blue, indigo, violet)",
        "rgb(199, 173, 42)",
        "rgb(225, 122, 48)",
        "rgb(214, 60, 46)",
        "rgb(58, 83, 203)"
    ]]
    return co[darkMode ? 0 : 1][themeMode - 1] ?? "rgb(17, 111, 206)"
}
export function OpenCustomURL(url: string) {
    BrowserOpenURL(url)
}