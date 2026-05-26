const rootLogger = require('pino')()

export const logger = rootLogger.child({ level: "info" });
