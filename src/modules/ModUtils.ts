export const $env = import.meta.env;

export const versionFilter: string[] = [
    "全部",
    "1.21.4",
    "1.21.1",
    "1.20.6",
    "1.20.1",
    "1.19.4",
    "1.19.2",
    "1.18.2",
    "1.17.1",
    "1.16.5",
    "1.14.4",
    "1.12.2",
    "1.10.2",
    "1.8.9",
    "1.7.10",
];

export const booleanWrapper: Record<string, boolean> = {
    true: true,
    false: false,
};

export class Panic extends Error {
    constructor(message: string, module: string) {
        super(`thread "${module ?? "unknown"}" panicked at "${message}", traceback under here:`);
        Object.setPrototypeOf(this, Panic.prototype);
    }
}
