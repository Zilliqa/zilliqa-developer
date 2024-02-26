const IS_DEV_ENV = !process.env.NODE_ENV || process.env.NODE_ENV === "development"

export const logger = function (...args: any) {
    if (IS_DEV_ENV) {
        console.log.apply(console, args);
    }
}