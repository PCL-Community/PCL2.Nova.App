const _MrApi = import.meta.env["VITE_MR_STAGING"] ? "https://staging-api.modrinth.com/v2" : "https://api.modrinth.com/v2";

export const ModrinthApiUrl = (p: string): string => _MrApi + p;

export const CompTypeMapper: Record<string, string> = {
    mod: " Mod ",
    modpack: "整合包",
    resourcepack: "资源包",
    shader: "光影包",
    datapack: "数据包",
};
