import { ModEventBus } from "../ModEventBus";
import { Panic } from "../ModUtils";

export enum LogLevel {
    TRACE = "trace",
    DEBUG = "debug",
    INFO = "info",
    WARN = "warn",
    ERROR = "error",
}

class ModLogger {
    private logfile_path: string = "";

    constructor(logfile_path?: string) {
        this.logfile_path = logfile_path ?? "";
    }

    public modifyLogfile(logfile_path: string) {
        this.logfile_path = logfile_path;
    }

    public log(message: string, level: LogLevel = LogLevel.INFO) {
        throw new Panic(
            `ModLogger.log called with {message: ${message}, level: ${level}}, but it is not implemented.`,
            "Setup"
        );
    }
}

const Logger = new ModLogger();

ModEventBus.on("logger:modify", (logfile_path: string) => {
    Logger.modifyLogfile(logfile_path);
});

ModEventBus.on("logger:log", (payload: any) => {
    Logger.log(payload.message, payload.level);
});

export { Logger };
