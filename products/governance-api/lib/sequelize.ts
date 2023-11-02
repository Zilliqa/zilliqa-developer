import { Sequelize } from "sequelize-typescript";

import { Hub, Message } from "./models";
import sequelizeENV from "./config/sequelize";

const env = process.env.NODE_ENV || "development";
const config = sequelizeENV[env];

export const sequelize = new Sequelize(config);

sequelize.addModels([Hub, Message]);

export async function sequelizeRun() {
  return sequelize.sync(config.sync);
}
