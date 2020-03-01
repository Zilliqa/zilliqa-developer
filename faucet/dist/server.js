"use strict";

require("babel-polyfill");

var _express = require("express");

var _express2 = _interopRequireDefault(_express);

var _bodyParser = require("body-parser");

var _bodyParser2 = _interopRequireDefault(_bodyParser);

var _fsExtra = require("fs-extra");

var _fsExtra2 = _interopRequireDefault(_fsExtra);

var _faucet = require("./faucet");

function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }

function _asyncToGenerator(fn) { return function () { var gen = fn.apply(this, arguments); return new Promise(function (resolve, reject) { function step(key, arg) { try { var info = gen[key](arg); var value = info.value; } catch (error) { reject(error); return; } if (info.done) { resolve(value); } else { return Promise.resolve(value).then(function (value) { step("next", value); }, function (err) { step("throw", err); }); } } return step("next"); }); }; }

require('dotenv').config();

var FAUCET_PORT = process.env.FAUCET_PORT;

var app = (0, _express2.default)();

app.use(_bodyParser2.default.urlencoded({ extended: false }));
app.use(_bodyParser2.default.json());

app.post('/register-account', function () {
    var _ref = _asyncToGenerator( /*#__PURE__*/regeneratorRuntime.mark(function _callee(req, res) {
        var address, result;
        return regeneratorRuntime.wrap(function _callee$(_context) {
            while (1) {
                switch (_context.prev = _context.next) {
                    case 0:
                        address = req.body.address;
                        _context.prev = 1;
                        _context.next = 4;
                        return (0, _faucet.registerUser)(address);

                    case 4:
                        result = _context.sent;
                        return _context.abrupt("return", res.send(result));

                    case 8:
                        _context.prev = 8;
                        _context.t0 = _context["catch"](1);
                        return _context.abrupt("return", res.send(_context.t0));

                    case 11:
                    case "end":
                        return _context.stop();
                }
            }
        }, _callee, undefined, [[1, 8]]);
    }));

    return function (_x, _x2) {
        return _ref.apply(this, arguments);
    };
}());

app.get('/faucet-state', function (req, res) {
    try {
        var state = (0, _faucet.getState)();

        return res.send(state);
    } catch (error) {
        return res.send(error);
    }
});

app.listen(FAUCET_PORT, function () {
    return console.log("Faucet listening on port " + FAUCET_PORT + "!");
});

// Check if faucet contract exists, if not, deploy one.
if (!_fsExtra2.default.existsSync('./faucet-state.json')) {
    console.log('Faucet state file does not exist. Deploying new contract...');

    (0, _faucet.deployFaucet)();
} else {
    console.log('Faucet state:');
    var state = (0, _faucet.getState)();
    console.log(state);
}