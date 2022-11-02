"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var ejs = require("ejs");
function render(content, data) {
    return ejs.render(content, data);
}
exports.render = render;
//# sourceMappingURL=template.js.map