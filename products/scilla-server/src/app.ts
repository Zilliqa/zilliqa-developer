/**
 * This file is part of savant-ide.
 * Copyright (c) 2018 - present Zilliqa Research Pte. Ltd.
 *
 * savant-ide is free software: you can redistribute it and/or modify it under the
 * terms of the GNU General Public License as published by the Free Software
 * Foundation, either version 3 of the License, or (at your option) any later
 * version.
 *
 * savant-ide is distributed in the hope that it will be useful, but WITHOUT ANY
 * WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
 * A PARTICULAR PURPOSE.  See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with
 * savant-ide.  If not, see <http://www.gnu.org/licenses/>.
 */

import bodyParser from "body-parser";
import cors from "cors";
import errorHandler from "errorhandler";
import express, { Handler, Request, Response, NextFunction } from "express";
import fs from "fs";
import lusca from "lusca";
import path from "path";
import { check, run } from "./handlers";

const app = express();
const wrapAsync =
  (fn: Handler) => (req: Request, res: Response, next: NextFunction) => {
    Promise.resolve(fn(req, res, next)).catch(next);
  };

// create temp folder if it doesn't exist
const temp = path.join(process.cwd(), "temp");

if (!fs.existsSync(temp)) {
  fs.mkdirSync(temp);
}

// configure express
app.use(cors());
app.use(bodyParser.json());
app.use(bodyParser.urlencoded({ extended: true }));
app.use(lusca.xframe("SAMEORIGIN"));
app.use(lusca.xssProtection(true));
if (process.env.NODE_ENV === "development") {
  app.use(errorHandler());
}

app.get("/healthcheck", (req: any, res: any) => {
  return res.send({
    status: "running",
  });
});

app.post("/contract/check", wrapAsync(check));
app.post("/contract/call", wrapAsync(run));

export default app;
