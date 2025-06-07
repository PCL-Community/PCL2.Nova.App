import {BrowserOpenURL} from "../../wailsjs/runtime";

export function DarkAndThemeToConst(darkMode: boolean, themeMode: number): string {
    let co = [[
        "rgb(6, 66, 154)",
        "rgb(29, 78, 78)",
        "rgb(39, 100, 43)",
        "rgb(71, 107, 32)",
        "rgb(109, 79, 47)",
        "url('/src/assets/images/Themes/BlackDark.png')",
        "url('/src/assets/images/Themes/FoolRainbowDark.png')",
        "url('/src/assets/images/Themes/PinkDark.png')",
        "url('/src/assets/images/Themes/PurpleDark.png')",
        "url('/src/assets/images/Themes/LuckyRainbowDark.png')",
        "url('/src/assets/images/Themes/GoldDark.png')",
        "url('/src/assets/images/Themes/OrangeDark.png')",
        "url('/src/assets/images/Themes/MojangRedDark.png')",
        "url('/src/assets/images/Themes/HackBlueDark.png')",
    ], [
        "rgb(17, 111, 206)",
        "rgb(49, 148, 148)",
        "rgb(69, 160, 83)",
        "rgb(131, 157, 52)",
        "rgb(159, 129, 87)",
        "url('/src/assets/images/Themes/Black.png')",
        "url('/src/assets/images/Themes/FoolRainbow.png')",
        "url('/src/assets/images/Themes/Pink.png')",
        "url('/src/assets/images/Themes/Purple.png')",
        "url('/src/assets/images/Themes/LuckyRainbow.png')",
        "url('/src/assets/images/Themes/Gold.png')",
        "url('/src/assets/images/Themes/Orange.png')",
        "url('/src/assets/images/Themes/MojangRed.png')",
        "url('/src/assets/images/Themes/HackBlue.png')",
    ]]
    return co[darkMode ? 0 : 1][themeMode - 1] ?? "rgb(17, 111, 206)"
}
export function OpenCustomURL(url: string) {
    BrowserOpenURL(url)
}