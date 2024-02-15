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

import fs from "fs";
import { promisify } from "util";

interface File {
  path: string;
  data: string;
}

const writeFile = promisify(fs.writeFile);

export const makeTempFileName = (
  id: string,
  extension: string,
  suffix?: string
): string => {
  return `${process.cwd()}/temp/${id}${
    suffix ? "_" + suffix : ""
  }.${extension}`;
};

/**
 * writeFiles
 *
 * asynchronously writes files to disk
 *
 * @param {File[]} files
 * @returns {Promise<any>}
 */
export const writeFiles = (files: File[]): Promise<any> => {
  return Promise.all(
    files.map((file) => {
      return writeFile(file.path, file.data);
    })
  );
};
