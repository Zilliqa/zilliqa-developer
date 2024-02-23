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

import path from "path";
import fs from "fs";

const { NODE_ENV, SCILLA_VERSION = 0 } = process.env;
const appDirectory = fs.realpathSync(process.cwd());
const scillaDirectory =
  NODE_ENV === "production"
    ? `/scilla/${SCILLA_VERSION}`
    : path.resolve(appDirectory, "..", "scilla");
const resolveScilla = (relativePath: string) =>
  path.resolve(scillaDirectory, relativePath);

export const Paths = {
  CHECKER: resolveScilla("bin/scilla-checker"),
  RUNNER: resolveScilla("bin/scilla-runner"),
  STDLIB: resolveScilla("src/stdlib/"),
};

console.log(Paths);

export const OptionalRunnerOpts = ["state", "message"];
