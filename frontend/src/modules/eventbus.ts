/**
 * Eventbus - 一个简单的事件总线实现
 * 用于组件间通信，解耦发布者和订阅者
 */

type EventHandler = (...args: any[]) => void;

interface EventHandlerMap {
    [eventName: string]: EventHandler[];
}

export class EventBus {
    private static instance: EventBus;
    private handlers: EventHandlerMap = {};

    /**
     * 获取 EventBus 单例
     */
    public static getInstance(): EventBus {
        if (!EventBus.instance) {
            EventBus.instance = new EventBus();
        }
        return EventBus.instance;
    }

    /**
     * 订阅事件
     * @param eventName 事件名称
     * @param handler 事件处理函数
     */
    public on(eventName: string, handler: EventHandler): void {
        if (!this.handlers[eventName]) {
            this.handlers[eventName] = [];
        }
        this.handlers[eventName].push(handler);
    }

    /**
     * 取消订阅事件
     * @param eventName 事件名称
     * @param handler 事件处理函数，如果不提供则移除该事件的所有处理函数
     */
    public off(eventName: string, handler?: EventHandler): void {
        if (!this.handlers[eventName]) {
            return;
        }

        if (!handler) {
            // 移除该事件的所有处理函数
            delete this.handlers[eventName];
            return;
        }

        // 移除特定的处理函数
        const index = this.handlers[eventName].indexOf(handler);
        if (index !== -1) {
            this.handlers[eventName].splice(index, 1);
        }

        // 如果没有处理函数了，清理事件
        if (this.handlers[eventName].length === 0) {
            delete this.handlers[eventName];
        }
    }

    /**
     * 触发事件
     * @param eventName 事件名称
     * @param args 传递给事件处理函数的参数
     */
    public emit(eventName: string, ...args: any[]): void {
        if (!this.handlers[eventName]) {
            return;
        }

        // 复制一份处理函数列表，防止在处理过程中被修改
        const handlers = [...this.handlers[eventName]];
        handlers.forEach((handler) => {
            try {
                handler(...args);
            } catch (error) {
                console.error(`Error in event handler for ${eventName}:`, error);
            }
        });
    }

    /**
     * 只订阅一次事件，触发后自动取消订阅
     * @param eventName 事件名称
     * @param handler 事件处理函数
     */
    public once(eventName: string, handler: EventHandler): void {
        const onceHandler = (...args: any[]) => {
            handler(...args);
            this.off(eventName, onceHandler);
        };
        this.on(eventName, onceHandler);
    }

    /**
     * 清除所有事件订阅
     */
    public clear(): void {
        this.handlers = {};
    }
}

// 导出默认实例，方便直接使用
export default EventBus.getInstance();
