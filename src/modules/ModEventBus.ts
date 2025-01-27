type EventHandler<T = any> = (payload: T) => void;

class EventBus {
    private events: Map<string, EventHandler[]> = new Map();

    // 订阅事件
    on<T = any>(event: string, handler: EventHandler<T>): void {
        if (!this.events.has(event)) {
            this.events.set(event, []);
        }
        this.events.get(event)!.push(handler);
    }

    // 取消订阅
    off<T = any>(event: string, handler?: EventHandler<T>): void {
        if (!this.events.has(event)) return;

        if (handler) {
            // 移除指定的事件处理器
            const handlers = this.events.get(event)!;
            this.events.set(event, handlers.filter(h => h !== handler));
        } else {
            // 移除所有该事件的处理器
            this.events.delete(event);
        }
    }

    // 触发事件
    emit<T = any>(event: string, payload?: T): void {
        const handlers = this.events.get(event);
        if (handlers) {
            handlers.forEach(handler => handler(payload ?? {}));
        }
    }
}

// 实例化 EventBus
const eventBus = new EventBus();
export default eventBus;
