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

const GAS_ERR_RE = new RegExp(/Ran out of gas/g);

interface ErrorObj {
  line: number;
  column: number;
  msg: string;
}

interface CheckerErrorObj {
  error_message: string;
  start_location: {
    file: string;
    line: number;
    column: number;
  };
  end_location: {
    file: string;
    line: number;
    column: number;
  };
}

export class ScillaError extends Error {
  messages: ErrorObj[];
  __proto__: Error;

  constructor(messages: ErrorObj[]) {
    const trueProto = new.target.prototype;
    super();
    this.messages = messages;
    this.__proto__ = trueProto;
  }

  toString() {
    return this.messages
      .map(
        (eObj) =>
          `An error occured at line ${eObj.line}, column ${eObj.column}: ${eObj.msg}`
      )
      .join("\n");
  }
}

export const parseExecutionError = (out: string): ScillaError | null => {
  if (GAS_ERR_RE.exec(out)) {
    return new ScillaError([{ line: 0, column: 0, msg: "Out of gas!" }]);
  }
  const error = out.split("\n");

  if (error && error.length > 0) {
    const [, msg] = error;
    return new ScillaError([
      {
        line: 0,
        column: 0,
        msg,
      },
    ]);
  }

  return null;
};

export const parseCheckerError = (out: string): ScillaError | null => {
  const data = JSON.parse(out);
  const errors = data.errors;
  if (!errors.length) {
    return null;
  }
  return new ScillaError(
    errors.map((error: CheckerErrorObj) => {
      return {
        line: error.start_location.line,
        column: error.start_location.column,
        msg: error.error_message,
      };
    })
  );
};
