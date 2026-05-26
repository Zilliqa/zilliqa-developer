import { createServer } from "http";
import { app } from "./app";
import { logger } from "./logger";
import { sequelizeRun } from "./sequelize";

const port = process.env.PORT || 3000;

(async () => {
  try {
    await sequelizeRun();
    logger.info("Database synced");

    createServer(app).listen(port, () =>
      logger.info({ port }, "Server started")
    );
  } catch (err) {
    logger.error({ err }, "Startup failed");
    process.exit(1);
  }
})();
