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
    public static log(message: string, level: LogLevel = LogLevel.INFO) {
        throw new Panic(`ModLogger.log called with {message: ${message}, level: ${level}}, but it is not implemented.`, "Setup");
    }
}

ModEventBus.on("logger:log", (payload: { message: string; level: LogLevel }) => {
    ModLogger.log(payload.message, payload.level);
});
