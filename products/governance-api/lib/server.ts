import { createServer } from "http";
import { app } from "./app";
import { sequelizeRun } from "./sequelize";

const port = process.env.PORT || 3000;

(async () => {
  await sequelizeRun();

  createServer(app).listen(port, () =>
    console.info(`Server running on port ${port}`)
  );
})();
