import { LogError, LogInfo, LogWarning } from "../wailsjs/runtime/runtime";

export function CLog(...content: any[]) {
    console.log(`%c[Log]%c ${content}`, "color:skyblue", "");
    LogInfo(`[Log] ${content}`);
}

export function CWarn(...content: any[]) {
    console.log(`%c[Warn]%c ${content}`, "color:orange", "");
    LogWarning(`[Warn] ${content}`);
}

export function CError(...content: any[]) {
    console.log(`%c[Error]%c ${content}`, "color:red", "");
    LogError(`[Error] ${content}`);
}
