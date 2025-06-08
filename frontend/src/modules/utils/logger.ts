export function CLog(...content: any[]) {
    console.log(`%c[Log]%c ${content}`, "color:skyblue", "");
}

export function CWarn(...content: any[]) {
    console.log(`%c[Warn]%c ${content}`, "color:orange", "");
}

export function CError(...content: any[]) {
    console.log(`%c[Error]%c ${content}`, "color:red", "");
}
