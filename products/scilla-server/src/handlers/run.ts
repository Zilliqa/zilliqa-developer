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

import { Request, Response, NextFunction } from "express";
import uuid from "uuid/v4";

import { runner, makeTempFileName, writeFiles } from "../util";
import { ScillaError } from "../util/error";
import { Paths, OptionalRunnerOpts } from "../constants";

export const run = async (req: Request, res: Response, next: NextFunction) => {
  const id = uuid();

  const baseRunOpt = {
    code: makeTempFileName(id, "scilla"),
    init: makeTempFileName(id, "json", "init"),
    blockchain: makeTempFileName(id, "json", "blockchain"),
    output: makeTempFileName(id, "json", "output"),
    stdlib: Paths.STDLIB,
    gaslimit: req.body.gaslimit,
  };

  const runOpt = OptionalRunnerOpts.reduce((opts, opt) => {
    return !!req.body[opt]
      ? { ...opts, [opt]: makeTempFileName(id, "json", opt) }
      : opts;
  }, baseRunOpt);

  const toWrite = Object.keys(runOpt)
    .filter((k) => k !== "stdlib" && k !== "gaslimit")
    .map<{ path: string; data: string }>((k: string) => ({
      path: runOpt[k as keyof typeof runOpt],
      data: req.body[k] || "",
    }));

  try {
    await writeFiles(toWrite);
    const result = await runner(runOpt);

    res.status(200).json({
      result: "success",
      message: result,
    });
  } catch (err) {
    if (err instanceof ScillaError) {
      res.status(400).json({
        result: "error",
        message: err.messages,
      });
      return;
    }

    res.status(400).json({
      result: "error",
      message: err.message,
    });
  }
};
